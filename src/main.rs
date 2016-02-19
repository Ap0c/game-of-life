// ----- Setup ----- //

// Size of the board.
const NO_COLS: usize = 5;
const NO_ROWS: usize = 5;

// The status of a cell.
#[derive(PartialEq)]
#[derive(Debug)]
enum Status {
	Alive = 1,
	Dead = 0
}

use Status::*;


// ----- Functions ----- //

// Checks the surrounding cells and returns the current cell's status.
fn alive_or_dead(surroundings: &[&Status; 8]) -> Status {

	let sum = surroundings.iter().filter(|cell| ***cell == Alive).count();

	match sum {
		2 | 3 => Alive,
		_ => Dead
	}

}

// Figures out what the surrounding cells are and returns the current cell's
// status.
fn update_status(board: &[[Status; NO_COLS]; NO_COLS], row: usize, col: usize)
	-> Status {

	let left_col = if col > 0 { col - 1 } else { NO_COLS - 1 };
	let right_col = if col < NO_COLS - 1 { col + 1 } else { 0 };
	let row_above = if row > 0 { row - 1 } else { NO_ROWS - 1 };
	let row_below = if row < NO_ROWS - 1 { row + 1 } else { 0 };

	alive_or_dead(&[
		&board[row_above][left_col],
		&board[row_above][col],
		&board[row_above][right_col],
		&board[row][left_col],
		&board[row][right_col],
		&board[row_below][left_col],
		&board[row_below][col],
		&board[row_below][right_col]
	])

}


// ----- Main ----- //

fn main() {

	let board = [
		[Dead, Dead, Dead, Dead, Dead],
		[Dead, Alive, Alive, Dead, Dead],
		[Dead, Alive, Alive, Dead, Dead],
		[Dead, Dead, Dead, Dead, Dead],
		[Dead, Dead, Dead, Dead, Dead]
	];

	let status = update_status(&board, 1, 0);
	println!("{:?}", status);

}


// ----- Tests ----- //

#[cfg(test)]
mod test {

	use super::Status::*;
	use super::alive_or_dead;

	#[test]
	// Checks that a cell is dead if not enough of the surroundings are alive.
	fn not_enough() {

		let mut surroundings = [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead];
		assert_eq!(Dead, alive_or_dead(surroundings));

		surroundings = [Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead];
		assert_eq!(Dead, alive_or_dead(surroundings));

	}

	#[test]
	// Checks that a cell is alive if the conditions are right.
	fn just_right() {

		let mut surroundings = [Alive, Alive, Dead, Dead, Dead, Dead, Dead, Dead];
		assert_eq!(Alive, alive_or_dead(surroundings));

		surroundings = [Alive, Alive, Alive, Dead, Dead, Dead, Dead, Dead];
		assert_eq!(Alive, alive_or_dead(surroundings));

	}

	#[test]
	// Checks that a cell is dead if too much of the surroundings are alive.
	fn too_much() {

		let surroundings = [Alive, Alive, Alive, Alive, Dead, Dead, Dead, Dead];
		assert_eq!(Dead, alive_or_dead(surroundings));

	}

}
