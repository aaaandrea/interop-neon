#[macro_use]
extern crate neon;
extern crate fst;

use std::error::Error;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use fst::{Set, SetBuilder, IntoStreamer};

use neon::mem::Handle;
use neon::vm;
use neon::vm::{This, FunctionCall, Lock, JsResult};
use neon::js::{JsFunction, JsUndefined, Object, JsString, Value};
use neon::js::class::{JsClass, Class};


trait CheckArgument<'a> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }
}

declare_types! {
    pub class JsSetBuilder as JsSetBuilder for SetBuilder<io::BufWriter<File>> {
        init(call) {
            // let scope = call.scope;
            // takes path on disk
            // let this: Handle<JsSetBuilder> = call.arguments.this(scope);
            // let mut build = SetBuilder::memory();

            let mut wtr = io::BufWriter::new(File::create("/tmp/set.fst").unwrap());
            let mut build = SetBuilder::new(wtr).unwrap();
            Ok(build)
        }

        method  insert(mut call) {

            // builds fst on path
            // let mut build = SetBuilder::memory();
            let word = call
                .check_argument::<JsString>(0)
                ?.value();
            let scope = call.scope;
            let mut this: Handle<JsSetBuilder> = call.arguments.this(scope);

            // let this = try!(vm::lock(this, |setbuilder| {
            let setbuilder: &mut SetBuilder<io::BufWriter<File>> = this.grab(|setbuilder| setbuilder);
            // add item to setbuilder
            setbuilder.insert(word).unwrap();

            // let mut build = this.build;
            // let bytes = build.into_inner().unwrap();

            Ok(JsUndefined::new().upcast())
        }

        method finish(mut call) {
            Ok(JsUndefined::new().upcast())
        }
    }

    // pub class JsSet as JsSet for Set {
    //     init(call) {
    //         let scope = call.scope;
    //         let set = unsafe { Set::from_path("set.fst").unwrap() };
    //         Ok(set)
    //     }
    //
    //     method search(call) {
    //         let scope = call.scope;
    //         let set = unsafe { Set::from_path("set.fst").unwrap() };
    //         let mut stream = set.into_stream();
    //         let mut keys = vec![];
    //         while let Some(key) = stream.next() {
    //             keys.push(key.to_vec());
    //         }
    //         assert_eq!(keys, vec![
    //             "bruce".as_bytes(), "clarence".as_bytes(), "stevie".as_bytes(),
    //         ]);
    //         Ok(JsUndefined::new().upcast())
    //     }
    // }
}

register_module!(m, {
    let set_builder_class: Handle<JsClass<JsSetBuilder>> = try!(JsSetBuilder::class(m.scope));
    let set_builder_constructor: Handle<JsFunction<JsSetBuilder>> = try!(set_builder_class.constructor(m.scope));

	try!(m.exports.set("SetBuilder", set_builder_constructor));
    // try!(m.exports.set("ReadFst", ReadFst));
	Ok(())
});
