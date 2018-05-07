#[macro_use]
extern crate neon;
extern crate num_cpus;
extern crate fst;
extern crate fst_regex;

use std::error::Error;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use fst::{Streamer, Set, SetBuilder, IntoStreamer};

use neon::mem::Handle;
use neon::js::{JsFunction, JsUndefined, Object};
use neon::js::class::{JsClass, Class};

pub struct FstSet {
    pub something: Set,
}

declare_types! {
    pub class WriteFst as WriteFst for SetBuilder<Vec<u8>> {
        init(mut call) {
            let scope = call.scope;
            // takes path on disk

            let mut build = SetBuilder::memory();
            Ok(build)
        }

        method  buildFST(filename: &str) {

            // builds fst on path
            let mut build = SetBuilder::memory();
            let this: Handle<WriteFst> = call.arguments.this(scope);

            let file = File::open(&filename)?;
            let reader = io::BufRreader::new(file);
            for line in reader.lines() {
                let line = line?;
                build.insert(line).unwrap();
            }
            // let bytes = build.into_inner().unwrap();
            Ok(JsUndefined::new().upcast())
        }
    }
    // pub class ReadFst for Set {
    //     init(call) {
    //         let scope = call.scope;
    //         let set = unsafe { Set::from_path("set.fst").unwrap() };
    //         Ok(set)
    //     }
    //
    //     method search(call) {
    //         let scope = call.scope;
    //         // let set = scope.set;
    //         // let set = unsafe { Set::from_path("set.fst").unwrap() };
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
    let write_class: Handle<JsClass<WriteFst>> = try!(WriteFst::class(m.scope));
    let write_constructor: Handle<JsFunction<WriteFst>> = try!(write_class.constructor(m.scope));

	try!(m.exports.set("WriteFst", write_constructor));
    // try!(m.exports.set("ReadFst", ReadFst));
	Ok(())
});
