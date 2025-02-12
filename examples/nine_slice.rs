use unicode_box_builder::{scalable::rect::ScalableRect, BOX_TILES};

pub const UNICODE_BOX: ScalableRect = ScalableRect {
    width: 50,
    height: 10,
    nine_slice: BOX_TILES,
};

fn main() {
    println!("{}", UNICODE_BOX);
}
