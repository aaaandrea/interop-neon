#[macro_use]
extern crate neon;
extern crate fst;

use std::io;
use std::fs::File;
use std::error::Error;

use neon::mem::Handle;
use neon::vm::{This, FunctionCall, Lock, JsResult};
use neon::js::{JsFunction, JsUndefined, Object, JsString, Value, JsBoolean};
use neon::js::class::{JsClass, Class};

use fst::{Set, SetBuilder};


trait CheckArgument<'a> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }
}

declare_types! {
    pub class JsSetBuilder as JsSetBuilder for Option<SetBuilder<io::BufWriter<File>>>{
        init(mut call) {
            let filename = call
                .check_argument::<JsString>(0)
                ?.value();

            let mut wtr = io::BufWriter::new(File::create(filename).unwrap());
            let mut build = SetBuilder::new(wtr).unwrap();

            Ok(Some(build))
        }

        method  insert(mut call) {
            let word = call
                .check_argument::<JsString>(0)
                ?.value();
            let scope = call.scope;
            let mut this: Handle<JsSetBuilder> = call.arguments.this(scope);
            this.grab(|setbuilder| {
                match setbuilder {
                    Some(builder) => {
                        builder.insert(word).unwrap();
                    },
                    None => {
                        // return error
                        panic!("SetBuilder not available for insertion");
                    }
                }
            });

            Ok(JsUndefined::new().upcast())
        }

        method finish(mut call) {
            let scope = call.scope;
            let mut this: Handle<JsSetBuilder> = call.arguments.this(scope);

            this.grab(|setbuilder| {
                match setbuilder.take() {
                    Some(builder) => {
                        builder.finish();
                    },
                    None => {
                        // return error
                        panic!("SetBuilder not available for finish()");
                    }
                }
            });
            Ok(JsUndefined::new().upcast())
        }
    }

    pub class JsSet as JsSet for Set {
        init(mut call) {
            let filename = call
                .check_argument::<JsString>(0)
                ?.value();
            let set = unsafe { Set::from_path(filename).unwrap() };
            Ok(set)
        }

        method contains(mut call) {
            let word = call
                .check_argument::<JsString>(0)
                ?.value();
            let scope = call.scope;
            let mut this: Handle<JsSet> = call.arguments.this(scope);

            Ok(JsBoolean::new(
                scope,
                this.grab(|set| {
                    set.contains(&word)
                })
            ).upcast())
        }
    }
}

register_module!(m, {

        let class: Handle<JsClass<JsSetBuilder>> = try!(JsSetBuilder::class(m.scope));
        let constructor: Handle<JsFunction<JsSetBuilder>> = try!(class.constructor(m.scope));
        try!(m.exports.set("SetBuilder", constructor));

        let class: Handle<JsClass<JsSet>> = try!(JsSet::class(m.scope));
        let constructor: Handle<JsFunction<JsSet>> = try!(class.constructor(m.scope));
        try!(m.exports.set("Set", constructor));

        Ok(())
});
