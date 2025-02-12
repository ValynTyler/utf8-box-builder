use nslice::nine_slice::NineSlice;

pub mod scalable;

pub const BOX_TILES: NineSlice<char> = NineSlice {
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
