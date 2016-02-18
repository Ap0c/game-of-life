// ----- Setup ----- //

// The status of a square.
#[derive(PartialEq)]
#[derive(Debug)]
enum Status {
	Alive = 1,
	Dead = 0
}

impl Status {

	fn as_number(&self) -> u8 {

		match *self {
			Alive => 1,
			Dead => 0,
		}

	}

}

use Status::*;


// ----- Functions ----- //

// Checks the surrounding cells and returns the current cell's status.
fn alive_or_dead(surroundings: [Status; 8]) -> Status {

	let sum = surroundings.iter().filter(|cell| **cell == Alive).count();

	match sum {
		2 | 3 => Alive,
		_ => Dead
	}

}


// ----- Main ----- //

fn main() {
	let status = alive_or_dead([Alive, Dead, Dead, Dead, Dead, Dead, Alive, Alive]);
	println!("{}", status.as_number());
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
