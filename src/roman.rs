use std::fmt::{Display, Formatter};

pub struct Roman {
    pub value: u32,
    pub digits: [u32; 4],
}


impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<u32> for Roman {
    fn from(value: u32) -> Self {
        todo!()
    }
}