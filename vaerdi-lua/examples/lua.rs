use blua_worker::Worker;
use gerning::{arguments::Arguments, Callable};
use vaerdi::Value;

#[derive(Debug, Clone)]
pub struct Test;

impl mlua::UserData for Test {}

fn main() {
    let worker = Worker::new(|| async move { Ok(mlua::Lua::new()) }).unwrap();

    let cb = worker
        .with(|vm| {
            Box::pin(async move {
                let table = vm
                    .load(mlua::chunk! {
                      return {
                        returns = "string",
                        params = {
                          "string"
                        },
                        call = function(_, name)
                          return "Hello, " .. name
                        end
                      }
                    })
                    .eval::<mlua::Table>()?;

                vaerdi_lua::gerning::command(vm, table)
            })
        })
        .expect("with");

    let ret = cb
        .call(
            &mut Test,
            Arguments::new(vec![Value::String("World".into())]),
        )
        .expect("call");

    println!("ret: {:?}", ret);
}
