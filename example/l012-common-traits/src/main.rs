#[derive(Debug, Clone)]
struct Puzzle{
    name: String,
    num_pieces: u32,
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle { name: String::from("Mistery Puzzle"), num_pieces: 3000 }
    }
}

    impl PartialEq for Puzzle {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name && self.num_pieces == other.num_pieces
        }
    }
    impl Eq for Puzzle {}

impl From<String> for Puzzle {
    fn from(name: String) -> Self {
        Puzzle{
            name,
            num_pieces: 0, // Set default number of pieces.
        }
    }
}

// we need to implement the From trait for String so that we can use Into on the String
impl From<Puzzle> for String {
    fn from(puzzle: Puzzle) -> Self {
        puzzle.name
    }
}

fn show<T: Into<String>>(s: T) {
    println!("{}", s.into());
}

fn main() {
    // 1. test Debug trait
    let puzzle = Puzzle{
        name: String::from("Forest Lake"),
        num_pieces: 10,
    };
    println!("{:?}", puzzle.clone());
    println!("{:#?}", puzzle.clone());

    // 2. test Default trait
    let default_puzzle = Puzzle::default();
    println!("default puzzle name: {}", default_puzzle.name);
    println!("default puzzle pieces: {}", default_puzzle.num_pieces);

    // 3. Implementing PartialEq and Eq (strict equality):
    let puzzle1 = Puzzle{
        name: String::from("puzzle1"),
        num_pieces: 1000,
    };
    let puzzle2 = Puzzle{
        name: String::from("puzzle1"),
        num_pieces: 1000,
    };
    println!("{}", puzzle1 == puzzle2); // True

    // 4. Implementing From<String> for Puzzle:
    let string = String::from("My Puzzle");
    let puzzle: Puzzle = string.into(); // convert String to Puzzle
    println!("{}", puzzle.name);
    
    // 5. generic function using Into:
    let puzzle3 = Puzzle{
        name: String::from("Forest Lake"),
        num_pieces: 1000,
    };
    show(puzzle3); // Prints "Forest Lake"

}
