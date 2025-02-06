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
