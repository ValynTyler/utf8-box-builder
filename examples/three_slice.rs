use nslice::three_slice::ThreeSlice;
use unicode_box_builder::scalable::line::ScalableLine;

fn main() {
    let line = ScalableLine {
        length: 10,
        three_slice: ThreeSlice {
            _1: 'x',
            _2: '-',
            _3: 'x',
        },
    };

    println!("{}", line);
}
