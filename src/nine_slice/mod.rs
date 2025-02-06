use std::fmt::Write;

use crate::three_slice::ThreeSliceLine;

pub struct NineSliceBox {
    pub _1: char,
    pub _2: char,
    pub _3: char,
    pub _4: char,
    pub _5: char,
    pub _6: char,
    pub _7: char,
    pub _8: char,
    pub _9: char,
}

impl NineSliceBox {
    pub fn render(&self, width: u16, height: u16) -> String {
        match width >= 2 && height >= 2 {
            false => String::new(),
            true => {
                let top = ThreeSliceLine { _1: self._1, _2: self._2, _3: self._3 };
                let mid = ThreeSliceLine { _1: self._4, _2: self._5, _3: self._6 };
                let bot = ThreeSliceLine { _1: self._7, _2: self._8, _3: self._9 };

                top.render(width) + "\r\n"
                + &(mid.render(width) + "\r\n").repeat((height - 2) as usize)
                + &(bot.render(width))
            }
        }
    }

    pub fn top_left(&self)      -> char { self._1 }
    pub fn top_middle(&self)    -> char { self._2 }
    pub fn top_right(&self)     -> char { self._3 }
    pub fn middle_left(&self)   -> char { self._4 }
    pub fn center(&self)        -> char { self._5 }
    pub fn middle_right(&self)  -> char { self._6 }
    pub fn bottom_left(&self)   -> char { self._7 }
    pub fn bottom_middle(&self) -> char { self._8 }
    pub fn bottom_right(&self)  -> char { self._9 }
}
