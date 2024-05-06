use core::fmt;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use blua_worker::WeakWorker;

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
    worker: Arc<WeakWorker>,
    signature: gerning::signature::Signature<Value>,
}

impl gerning::Callable<CallCtx, Value> for LuaCommand {
    fn signature(&self) -> gerning::signature::Signature<Value> {
        self.signature.clone()
    }

    fn call(
        &self,
        ctx: &mut CallCtx,
        args: gerning::arguments::Arguments<Value>,
    ) -> Result<Value, gerning::Error<Value>> {
        let cmd = self.id;
        let value = self
            .worker
            .with(move |vm| async move {
                let func: mlua::Function = vm.named_registry_value(&cmd.to_string())?;

                let val: mlua::Value = func.call_async::<_, _>(()).await?;

                let val = match val {
                    mlua::Value::String(s) => Value::String(s.to_str()?.to_string()),
                    _ => {
                        panic!("value")
                    }
                };
                //
                Ok(val)
            })
            .expect("error");

        Ok(value)
    }
}

impl Drop for LuaCallback {
    fn drop(&mut self) {
        let cmd = self.id;
        self.worker
            .with(move |vm| async move { vm.unset_named_registry_value(&cmd.to_string()) })
            .ok();
    }
}
