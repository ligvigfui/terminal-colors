
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct U8Color {
    pub name: Option<String>,
    pub color: u8
}

impl U8Color {
    pub fn new(color: u8) -> U8Color {
        U8Color {
            name: None,
            color
        }
    }

    pub fn named(name: &str, color: u8) -> U8Color {
        U8Color {
            name: Some(name.to_string()),
            color
        }
    }
}