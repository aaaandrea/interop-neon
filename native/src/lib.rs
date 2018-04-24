#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::vm::{Call, JsResult};
use neon::mem::Handle;
use neon::vm::Lock;
use neon::js::{JsString, JsInteger, JsFunction, Object, Value};
use neon::js::class::{JsClass, Class};

pub struct Greeting {
    greeting: String,
}

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node\n\n     love, \n      rust").unwrap())
}


register_module!(m, {
    m.export("hello", hello)
});
