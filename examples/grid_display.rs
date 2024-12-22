use utf8_box_builder::grid::Grid;

fn main() {
    let grid = Grid(vec![
        vec![1, 2, 3],
        vec![4, 5, 6]
    ]);
    println!("{}", grid);
}
