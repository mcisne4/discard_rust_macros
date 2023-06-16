use ver_03::Version03;

pub trait Version03 {
    fn get_parent(&self) -> &'static str;
}

#[derive(Version03)]
enum AnEnum {
    Item01,
}
