use std::fmt::Display;

use nslice::three_slice::ThreeSlice;

pub struct ScalableLine {
    pub length: usize,
    pub three_slice: ThreeSlice<char>,
}

impl ScalableLine {
    pub fn left(&self) -> char { self.three_slice._1 }
    pub fn center(&self) -> char { self.three_slice._2 }
    pub fn right(&self) -> char { self.three_slice._3 }
}

impl Display for ScalableLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.left().to_string()
            + &self.center().to_string().repeat((self.length - 2) as usize)
            + &self.right().to_string()
        )
    }
}
