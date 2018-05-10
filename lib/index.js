var addon = require('../native');

let build = new fst.SetBuilder("set.fst");

build.insert("bruce");
build.insert("clarence");
build.insert("stevie");

build.finish();

let set = new fst.Set("set.fst");
for (let name of ["bruce", "clarence", "stevie"]) {
    assert.ok(set.contains(name));
}

// module.exports = addon.threading_hint;
