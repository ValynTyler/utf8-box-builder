use std::fmt::Display;

use nslice::{nine_slice::NineSlice, three_slice::ThreeSlice};
use tui_renderer::terminal::canvas::Canvas;

use crate::scalable::line::ScalableLine;

pub struct ScalableRect {
    pub width: usize,
    pub height: usize,
    pub nine_slice: NineSlice<char>,
}

impl ScalableRect {
    pub fn top_left(&self)      -> char { self.nine_slice._1 }
    pub fn top_middle(&self)    -> char { self.nine_slice._2 }
    pub fn top_right(&self)     -> char { self.nine_slice._3 }
    pub fn middle_left(&self)   -> char { self.nine_slice._4 }
    pub fn center(&self)        -> char { self.nine_slice._5 }
    pub fn middle_right(&self)  -> char { self.nine_slice._6 }
    pub fn bottom_left(&self)   -> char { self.nine_slice._7 }
    pub fn bottom_middle(&self) -> char { self.nine_slice._8 }
    pub fn bottom_right(&self)  -> char { self.nine_slice._9 }
}

impl From::<ScalableRect> for Canvas {
    fn from(value: ScalableRect) -> Self {
        value
            .to_string()
            .as_str()
            .into()
    }
}

impl Display for ScalableRect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.width >= 2 && self.height >= 2 {
            false => String::new(),
            true => {
                let top = ScalableLine {
                    length: self.width,
                    three_slice: ThreeSlice { _1: self.nine_slice._1, _2: self.nine_slice._2, _3: self.nine_slice._3 }
                };
                let mid = ScalableLine {
                    length: self.width,
                    three_slice: ThreeSlice { _1: self.nine_slice._4, _2: self.nine_slice._5, _3: self.nine_slice._6 },
                };
                let bot = ScalableLine {
                    length: self.width,
                    three_slice: ThreeSlice { _1: self.nine_slice._7, _2: self.nine_slice._8, _3: self.nine_slice._9 },
                };

                top.to_string() + "\r\n"
                + &(mid.to_string() + "\r\n").repeat((self.height - 2) as usize)
                + &(bot.to_string())
            }
        })
    }
}
