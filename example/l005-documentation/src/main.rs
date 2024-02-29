// Define a library with inner documentation comment
//! This is the top-level documentation for the `puzzle_solver` library

// Define a struct with field and function documentation
pub struct Puzzle {
    // number of pieces in the puzzle
    pub num_pieces: u32,
    /// name of the puzzle
    pub name: String,
}

impl Puzzle {
    // function to solve the puzzle
    pub fn solve(self) -> bool {
        // ... implementation details ...

        // Documentation for the solve function
        //! This function attempts to solve the puzzle and returns true if successful.
        false
    }
}

// constant with outer documentation comment
/// the number of puzzle pieces required to complete the world.
pub const PUZZLE_PIECES: u32 = 42;

fn main() {
    // example usage with intra-doc links
    let puzzle = Puzzle {
        num_pieces: 10,
        name: String::from("My Puzzle"),
    };

    println!("this puzzle has {} pieces. see the [PUZZLE_PIECES] constant for the world completion count.", puzzle.num_pieces);

    if puzzle.solve() {
        println!("Congratulations! You solved the puzzle!");
    } else {
        println!("Oh no, the puzzle remains unsolved. Try again!");
    }
}
