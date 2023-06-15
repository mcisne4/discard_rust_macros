mod types;
use types::{FooEnum, FooStruct};

use crate::ver01::types::Ver01;

pub fn run_version_01(run: bool) {
    if !run {
        return;
    }

    let foo_struct = FooStruct { _a: 34 };
    println!("{}\n", foo_struct.dev());

    let foo_enum = FooEnum::Item01("hello");
    println!("{}\n", foo_enum.dev());
}
