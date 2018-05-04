#[macro_use]
extern crate neon;
extern crate num_cpus;
extern crate fst;
extern crate fst_regex;

use std::error::Error;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use fst::{Streamer, Set, SetBuilder};
use fst_regex::Regex;

use neon::vm::{Call, JsResult};
use neon::mem::Handle;
use neon::js::{JsString, JsInteger, JsFunction, JsNumber, JsValue, JsNull, JsUndefined, Object};
use neon::js::class::{JsClass, Class};

pub struct FstSet {
    pub something: Set,
}

// fn example() -> Result<(), Box<Error>> {
//     let set1 = Set::from_iter(&["AC/DC", "Aerosmith"])?;
//     let set2 = Set::from_iter(&["Bob Seger", "Bruce Springsteen"])?;
//     let set3 = Set::from_iter(&["George Thorogood", "Golden Earring"])?;
//     let set4 = Set::from_iter(&["Kansas"])?;
//     let set5 = Set::from_iter(&["Metallica"])?;
//
//     Ok(())
// }

declare_types! {
    pub class WriteFst as WriteFst for SetBuilder<Vec<u8>> {
        init(mut call) {
            let scope = call.scope;
            // takes path on disk

            let mut build = SetBuilder::memory();
            // let mut wtr = io::BufWriter::new(File::create("set.fst").unwrap());
            // let pathway = Set::from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
            //     raw::Fst::frin_path(path).map(Set);
            // }
            Ok(build)
        }

        method  buildFST(call) {
            let scope = call.scope;
            // builds fst on path
            let mut build = SetBuilder::memory();
            build.insert("bruce").unwrap();
            build.insert("clarence").unwrap();
            build.insert("stevie").unwrap();

            // You could also call `finish()` here, but since we're building the set in
            // memory, there would be no way to get the `Vec<u8>` back.
            let bytes = build.into_inner().unwrap();
            Ok(JsUndefined::new().upcast())
        }
    }
    // pub class ReadFst for Set {
    //     init(mut call) {
    //         let scope = call.scope;
    //         // takes path on disk
    //         // let pathway = Set::from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
    //         //     raw::Fst::frin_path(path).map(Set);
    //         // }
    //         Ok(Set::new())
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
    //         Ok(JsUndefined::new())
    //     }
    // }
}

register_module!(m, {
    // try!(m.export("example", example));
    let writeClass: Handle<JsClass<WriteFst>> = try!(WriteFst::class(m.scope));
    let writeConstructor: Handle<JsFunction<WriteFst>> = try!(writeClass.constructor(m.scope));

	try!(m.exports.set("WriteFst", writeConstructor));
    // try!(m.export("ReadFst", ReadFst));
	Ok(())
});
