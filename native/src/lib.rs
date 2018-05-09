#[macro_use]
extern crate neon;
extern crate fst;
extern crate fst_regex;
extern crate fst_levenshtein;


use std::error::Error;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use fst::{Streamer, Set, SetBuilder, IntoStreamer};

use neon::mem::Handle;
use neon::js::{JsFunction, JsUndefined, Object};
use neon::js::class::{JsClass, Class};


declare_types! {
    pub class WriteFst as WriteFst for SetBuilder<Vec<u8>> {
        init(call) {
            // let scope = call.scope;
            // takes path on disk

            // let mut build = SetBuilder::memory();
            let mut wtr = io::BufWriter::new(File::create("/tmp/set.fst").unwrap());
            let mut build = SetBuilder::new(wtr).unwrap();
            Ok(build)
        }

        method  buildFST(filename: &str) {

            // builds fst on path
            // let mut build = SetBuilder::memory();
            let this: Handle<WriteFst> = call.arguments.this(scope);


            let mut build = this.build;

            let file = File::open(&filename)?;
            let reader = io::BufRreader::new(file);
            let mut buf = String::new();
            while reader.read_line(&mut buf)? > 0 {
                {
                    let line = buf.trim_right();
                    build.insert(line);
                }
                buf.clear();
            }
            let bytes = build.into_inner().unwrap();
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
    let write_class: Handle<JsClass<WriteFst>> = try!(WriteFst::class(m.scope));
    let write_constructor: Handle<JsFunction<WriteFst>> = try!(write_class.constructor(m.scope));

	try!(m.exports.set("WriteFst", write_constructor));
    // try!(m.exports.set("ReadFst", ReadFst));
	Ok(())
});
