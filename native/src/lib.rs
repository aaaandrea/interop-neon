#[macro_use]
extern crate neon;
extern crate num_cpus;



use neon::vm::{Call, JsResult};
use neon::mem::Handle;
use neon::js::{JsString, JsInteger, JsFunction, JsNumber, JsValue, JsNull, JsUndefined};
use neon::js::class::{JsClass, Class};

fn threading_hint(call: Call) -> JsResult<JsNumber> {
    Ok(JsNumber::new(call.scope, num_cpus::get() as f64))
}

// example for getting js arguments
// in JS:
// require('neon-module').async_method('show me the money', (n) => console.log(n))
// >> 'show me the money'

fn async_method(call: Call) -> JsResult<JsUndefined> {

	let fn_handle = call.arguments.get(call.scope, 1).unwrap();
	let arg_handle = call.arguments.get(call.scope, 0).unwrap();

	if let Some(function) = fn_handle.downcast::<JsFunction>() {

		let args: Vec<Handle<JsValue>> = vec![arg_handle];
		let _ = function.call(call.scope, JsNull::new(), args);
	}

	Ok(JsUndefined::new())
}

register_module!(m, {
	try!(m.export("async_method", async_method));
	Ok(())
});

// register_module!(m, {
//     m.export("threading_hint", threading_hint)
// });
