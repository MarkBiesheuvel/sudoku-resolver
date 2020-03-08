mod lib;
use lib::Cell;
use lib::Sudoku;

fn main() {
    // Source: https://youtu.be/9m9t8ie9-EE
    // Source: https://cracking-the-cryptic.web.app/sudoku/PrfmJPgdft
    let sudoku = Sudoku { cells: [
        [
            Cell { digit: Some(5) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(2) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(4) },
            Cell { digit: None }
        ],
        [
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(6) },
            Cell { digit: None },
            Cell { digit: Some(3) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None }
        ],
        [
            Cell { digit: None },
            Cell { digit: Some(3) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(9) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(7) }
        ],
        [
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(3) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(7) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None }
        ],
        [
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(7) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(8) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None }
        ],
        [
            Cell { digit: Some(6) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(2) },
            Cell { digit: None }
        ],
        [
            Cell { digit: None },
            Cell { digit: Some(8) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(3) }
        ],
        [
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(4) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(6) },
            Cell { digit: None },
            Cell { digit: None }
        ],
        [
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(1) },
            Cell { digit: None },
            Cell { digit: None },
            Cell { digit: Some(5) },
            Cell { digit: None },
            Cell { digit: None }
        ]
    ]};

    println!("\n== PRETTY OUTPUT ==");
    println!("{}", sudoku);

    println!("\n== ROWS ==");
    for i in 0..9 {
        println!("{}", sudoku.row(i).is_valid());
    }

    println!("\n== COLUMNS ==");
    for i in 0..9 {
        println!("{}", sudoku.column(i).is_valid());
    }

    println!("\n== SQUARES ==");
    for i in 0..9 {
        println!("{}", sudoku.square(i).is_valid());
    }
}
