mod sudoku;
mod ui;

use sfml::graphics::Font;
use sudoku::{Canvas, Cell};

fn main() {
    let font = Font::from_file("SpaceMono-Regular.ttf").unwrap();

    const ROWS: usize = 9;
    const COLS: usize = 9;
    let mut cells: Vec<Vec<Cell>>;

    cells = Vec::new();
    for i in 0..ROWS {
        cells.push(Vec::new());
        for j in 0..COLS {
            let c = Cell::new(i, j);
            cells[i].push(c);
        }
    }
    // println!("{:?}", cells);
    // for row in cells.iter() {
    //     for cell in row.iter() {
    //         println!("{:?}", cell.calc_position(20, 0));
    //     }
    // }
    let mut c = Canvas::new(400, 400, "Wavefunction Collapse", &font);
    c.setup();
    c.draw();
}
