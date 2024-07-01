
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct RGB {
    pub name: Option<String>,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB {
            name: None,
            red,
            green,
            blue
        }
    }

    pub fn named(name: &str, red: u8, green: u8, blue: u8) -> RGB {
        RGB {
            name: Some(name.to_string()),
            red,
            green,
            blue
        }
    }
}