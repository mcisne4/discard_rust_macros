#[derive(Debug, deluxe::ExtractAttributes)]
#[deluxe(attributes(parent))]
pub struct ParentAttributes {
    pub a: u8,
    pub b: u8,
}
