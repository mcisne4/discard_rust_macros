use ver_01::Ver01;

// === TRAITS === //
pub trait Ver01 {
    fn dev(&self) -> &'static str;
}

// === DATA TYPES === //
#[derive(Ver01)]
pub struct FooStruct {
    pub _a: u32,
}

#[derive(Ver01)]
#[sample2(1024)]
pub enum FooEnum {
    #[sample("XYZ {} {}", 3, 5)]
    Item01(&'static str),
}
