pub struct ThreeSliceLine {
    left_end: char,
    center: char,
    right_end: char,
}

impl ThreeSliceLine {
    pub fn render(&self, length: u32) -> String {
        self.left_end.to_string()
        + &self.center.to_string().repeat(length as usize)
        + &self.right_end.to_string()
    }
}
