#[derive(Debug, deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(parent))]
#[deluxe(default)]
pub struct ParentAttributes {
    pub a: u8,
    pub b: u8,
}
