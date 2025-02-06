use nine_slice::NineSliceBox;

pub mod grid;
pub mod nine_slice;
pub mod three_slice;

pub const HL: char = '─';
pub const VL: char = '│';

pub const TL: char = '┌';
pub const BL: char = '└';
pub const TR: char = '┐';
pub const BR: char = '┘';

pub const UNICODE_BOX: NineSliceBox = NineSliceBox {
    _1: '┌',
    _2: '─',
    _3: '┐',
    _4: '│',
    _5: ' ',
    _6: '│',
    _7: '└',
    _8: '─',
    _9: '┘',
};

pub const MIXED_BOX: NineSliceBox = NineSliceBox {
    _1: '┌',
    _2: '─',
    _3: '┐',
    _4: '|',
    _5: ' ',
    _6: '|',
    _7: '└',
    _8: '─',
    _9: '┘',
};
