#[allow(dead_code)]
// Key Point 1: Using Enums for Errors

// Key Point 2: Grouping Errors
// No need for separate enums unless significant differences exist
// Error variants are grouped within PuzzleError enum

// Key Point 3: Not Exposing External Errors
// Convert external dependencies' errors into own error types

// Key Point 4: Non-Exhaustive Enums
#[derive(Debug)]
#[non_exhaustive]
enum PuzzleError {
    ErrorOne,
    ErrorTwo,
}

// Key Point 5: Implementing Traits (Without thiserror)
use std::fmt;

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuzzleError::ErrorOne => write!(f, "Error One occurred"),
            PuzzleError::ErrorTwo => write!(f, "Error Two occurred"),
        }
    }
}

impl std::error::Error for PuzzleError {}

// Key Point 6: Using thiserror Crate
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
#[error("An error occurred: {0}")]
enum PuzzleError2 {
    #[error("ThisError One occurred")]
    ErrorOne,
    #[error("ThisError Two occurred")]
    ErrorTwo,
}

fn main() {
    println!("using standard implementation:");
    let error_one = PuzzleError::ErrorOne;
    println!("\t Error: {}", error_one);

    println!("using thiserror implementation:");
    let error_two = PuzzleError2::ErrorTwo;
    println!("\t Error: {}", error_two);
}
