extern crate gcc;

fn main() {
    gcc::compile_library("libremotery.a", &["external/remotery/lib/Remotery.c"]);
}

