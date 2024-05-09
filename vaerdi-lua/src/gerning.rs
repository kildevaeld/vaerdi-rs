use core::fmt;
use gerning::arguments::Arguments;
use mlua::{MetaMethod, MultiValue};
use std::{
    borrow::BorrowMut,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use vaerdi::{Type, Value};

use blua_worker::{LuaExt, WeakWorker};

use crate::convert;

#[derive(Debug, Clone, Copy)]
struct CallbackId(usize);

impl CallbackId {
    pub fn new() -> CallbackId {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        CallbackId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl fmt::Display for CallbackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct LuaCallback {
    id: CallbackId,
    worker: WeakWorker,
    signature: gerning::signature::Signature<Value>,
}

impl<CTX: 'static + Clone + mlua::UserData + Send> gerning::Callable<CTX, Value> for LuaCallback {
    fn signature(&self) -> gerning::signature::Signature<Value> {
        self.signature.clone()
    }

    fn call(
        &self,
        ctx: &mut CTX,
        args: gerning::arguments::Arguments<Value>,
    ) -> Result<Value, gerning::Error<Value>> {
        let cmd = self.id;
        let ctx = ctx.clone();
        let value = self
            .worker
            .with(move |vm| {
                Box::pin(async move {
                    let func: mlua::Function = vm.named_registry_value(&cmd.to_string())?;

                    let args = args
                        .into_iter()
                        .map(|v| convert::into_lua(vm, &v, true))
                        .collect::<Result<Vec<_>, _>>()?;

                    let m = MultiValue::from_vec(args);

                    let ctx = vm.create_userdata(ctx)?;

                    let val = func.call_async::<_, _>((ctx.clone(), m)).await;

                    let _ = ctx.take::<CTX>();

                    convert::from_lua(val?)
                })
            })
            .expect("error");

        Ok(value)
    }
}

impl<CTX: 'static + Clone + mlua::UserData + Send> gerning::AsyncCallable<CTX, Value>
    for LuaCallback
{
    type Future<'a> = Pin<Box<dyn Future<Output = Result<Value, gerning::Error<Value>>>>>;
    fn signature(&self) -> gerning::signature::Signature<Value> {
        self.signature.clone()
    }

    fn call_async<'a>(&'a self, ctx: &'a mut CTX, args: Arguments<Value>) -> Self::Future<'a> {
        let worker = self.worker.clone();
        let cmd = self.id;
        let ctx = ctx.clone();
        Box::pin(async move {
            let value = worker
                .with_async(move |vm| {
                    Box::pin(async move {
                        let func: mlua::Function = vm.named_registry_value(&cmd.to_string())?;

                        let args = args
                            .into_iter()
                            .map(|v| convert::into_lua(vm, &v, true))
                            .collect::<Result<Vec<_>, _>>()?;

                        let m = MultiValue::from_vec(args);

                        let ctx = vm.create_userdata(ctx)?;

                        let val = func.call_async::<_, _>((ctx.clone(), m)).await;

                        let _ = ctx.take::<CTX>();

                        convert::from_lua(val?)
                    })
                })
                .await
                .expect("error");

            Ok(value)
        })
    }
}

impl Drop for LuaCallback {
    fn drop(&mut self) {
        let cmd = self.id;
        self.worker
            .with(move |vm| {
                Box::pin(async move { vm.unset_named_registry_value(&cmd.to_string()) })
            })
            .ok();
    }
}

pub struct LuaParam(Type);

impl<'lua> mlua::FromLua<'lua> for LuaParam {
    fn from_lua(
        value: mlua::prelude::LuaValue<'lua>,
        lua: &'lua mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<Self> {
        let Some(str) = value.as_str() else {
            return Err(mlua::Error::external("expected string"));
        };

        let kind = match str {
            "string" => Type::String,
            "bool" => Type::Bool,
            "bytes" => Type::Bytes,
            _ => return Err(mlua::Error::external(format!("unknown type: {str}"))),
        };

        Ok(LuaParam(kind))
    }
}

impl<'lua> mlua::IntoLua<'lua> for LuaParam {
    fn into_lua(
        self,
        lua: &'lua mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
        let str = match self.0 {
            Type::Bool => "bool",
            Type::String => "string",
            Type::Bytes => "bytes",
            _ => {
                panic!()
            }
        };

        lua.create_string(str).map(mlua::Value::String)
    }
}

pub fn command<'lua>(vm: &'lua mlua::Lua, map: mlua::Table<'lua>) -> mlua::Result<LuaCallback> {
    let lparams: Vec<LuaParam> = map.get("params")?;
    let returns: Option<LuaParam> = map.get("returns")?;
    let function: mlua::Function = map.get("call")?;

    let mut params = gerning::signature::Parameters::<Value>::build();

    for p in lparams {
        params.add(p.0);
    }

    let signature = gerning::signature::Signature::new(
        params.build(),
        returns.map(|m| m.0).unwrap_or(Type::Bool),
    );

    let cmd = CallbackId::new();

    vm.set_named_registry_value(&cmd.to_string(), function)?;

    Ok(LuaCallback {
        id: cmd,
        worker: vm.worker()?,
        signature,
    })
}

pub struct Callback<C, T>(T, PhantomData<C>);

impl<C, T> Callback<C, T> {
    pub fn new(call: T) -> Callback<C, T> {
        Callback(call, PhantomData)
    }
}

impl<C, T> mlua::UserData for Callback<C, T>
where
    T: gerning::Callable<C, Value>,
    C: mlua::UserData + 'static,
{
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {}

    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(
            MetaMethod::Call,
            |vm, this, (mut ctx, args): (mlua::UserDataRefMut<C>, mlua::Variadic<mlua::Value>)| {
                let args = args
                    .into_iter()
                    .map(|v| convert::from_lua(v))
                    .collect::<Result<_, _>>()?;

                let ret = this
                    .0
                    .call(ctx.borrow_mut(), Arguments::new(args))
                    .map_err(|err| mlua::Error::external(err.to_string()))?;

                convert::into_lua(vm, &ret, true)
            },
        )
    }
}
