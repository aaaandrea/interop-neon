#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsInteger, JsFunction, JsNumber, Object, Value};
use neon::js::class::{JsClass, Class};

fn threading_hint(call: Call) -> JsResult<JsNumber> {
    Ok(JsNumber::new(call.scope, num_cpus::get() as f64))
}

register_module!(m, {
    m.export("threading_hint", threading_hint)
});
