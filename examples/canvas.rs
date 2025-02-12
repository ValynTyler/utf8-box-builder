use tui_renderer::terminal::canvas::Canvas;
use unicode_box_builder::{scalable::rect::ScalableRect, BOX_TILES};

fn main() {
    let mut canvas = Canvas::new(50, 10, ' ');
    for i in 0..9 {
        let rect = ScalableRect { nine_slice: BOX_TILES, height: 10 - i, width: 50  - i};
        canvas = canvas.render((0, 0), &rect.into());
    }

    println!("{}", canvas);
}
