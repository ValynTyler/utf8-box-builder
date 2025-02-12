use nslice::nine_slice::NineSlice;
use unicode_box_builder::scalable::rect::ScalableRect;

pub const UNICODE_BOX: ScalableRect = ScalableRect {
    width: 50,
    height: 10,
    nine_slice: NineSlice {
        _1: '┌',
        _2: '─',
        _3: '┐',
        _4: '│',
        _5: ' ',
        _6: '│',
        _7: '└',
        _8: '─',
        _9: '┘',
    }
};

fn main() {
    println!("{}", UNICODE_BOX);
}
