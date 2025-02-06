pub struct ThreeSliceLine {
    pub _1: char,
    pub _2: char,
    pub _3: char,
}

impl ThreeSliceLine {
    pub fn render(&self, length: u16) -> String {
        self.left_end().to_string()
        + &self.center().to_string().repeat((length - 2) as usize)
        + &self.right_end().to_string()
    }

    pub fn left_end(&self) -> char {
        self._1
    }

    pub fn center(&self) -> char {
        self._2
    }

    pub fn right_end(&self) -> char {
        self._3
    }
}
