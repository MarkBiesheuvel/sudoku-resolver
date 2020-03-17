mod lib;
use lib::build_cell;
use lib::build_sudoku;

fn main() {
    // Source: https://youtu.be/9m9t8ie9-EE
    // Source: https://cracking-the-cryptic.web.app/sudoku/PrfmJPgdft
    let sudoku = build_sudoku([
        build_cell("5"),
        build_cell(""),
        build_cell(""),
        build_cell("2"),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell("4"),
        build_cell(""),
        //
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell("6"),
        build_cell(""),
        build_cell("3"),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        //
        build_cell(""),
        build_cell("3"),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell("9"),
        build_cell(""),
        build_cell(""),
        build_cell("7"),
        //
        build_cell(""),
        build_cell(""),
        build_cell("3"),
        build_cell(""),
        build_cell(""),
        build_cell("7"),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        //
        build_cell(""),
        build_cell(""),
        build_cell("7"),
        build_cell(""),
        build_cell(""),
        build_cell("8"),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        //
        build_cell("6"),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell("2"),
        build_cell(""),
        //
        build_cell(""),
        build_cell("8"),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell("3"),
        //
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell("4"),
        build_cell(""),
        build_cell(""),
        build_cell("6"),
        build_cell(""),
        build_cell(""),
        //
        build_cell(""),
        build_cell(""),
        build_cell(""),
        build_cell("1"),
        build_cell(""),
        build_cell(""),
        build_cell("5"),
        build_cell(""),
        build_cell("")
    ]);

    println!("\n== PRETTY OUTPUT ==");
    println!("{}", sudoku);

    println!("\n== POSIBILITIES ==");
    sudoku.debug_posibilities()
}
