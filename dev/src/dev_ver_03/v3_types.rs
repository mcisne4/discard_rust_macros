use ver_03::Version03;

pub trait Version03 {
    fn get_parent(&self) -> &'static str;

    fn get_children() -> &'static str;
}

#[derive(Version03)]
#[parent(a = 4, b = 34)]
enum AnEnum {
    #[child("child data for Item01")]
    Item01,
    #[child("child data for Item02 [{0} {0}]")]
    Item02(String, usize),
    #[child("child data for Item03 [{0} {0} {0} {0}]")]
    Item03(usize, String, String),
}
