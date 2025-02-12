use unicode_box_builder::three_slice::ThreeSliceLine;

fn main() {
    let rod = ThreeSliceLine {
        _1: 'x',
        _2: '-',
        _3: 'x',
    };

    println!("{}", rod.render(5));
}
