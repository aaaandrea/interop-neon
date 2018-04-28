#[macro_use]
extern crate neon;
extern crate num_cpus;
extern crate fst;

// write a program that ties that with a very basic usage of the Rust fst library's 
// Set type. You'll want to expose two classes: one that takes a path on disk,
// builds an FST there, and exposes a function to JS for adding words and a function
// for finalizing the structure, and another class that accepts the path of a built
// FST, loads it, and provides a read-only interface to its contents that can test
// if a word is in it or not. Functionality-wise, these will approximately wrap
// SetBulder and Set respectively. For the latter, ideally we'd load the built
// file via mmap.


use fst::Set;
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
