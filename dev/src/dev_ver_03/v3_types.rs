use ver_03::Version03;

pub trait Version03 {
    fn get_parent(&self) -> &'static str;
}

#[derive(Version03)]
#[parent(a = 4, b = 34)]
enum AnEnum {
    Item01,
}
