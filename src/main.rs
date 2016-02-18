// ----- Setup ----- //

// The status of a square.
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

fn alive_or_dead(surroundings: [Status; 8]) -> Status {

	let sum = surroundings.iter().fold(0, |sum, cell| sum + cell.as_number());

	match sum {
		2 | 3 => Alive,
		_ => Dead
	}

}

fn main() {
	let status = alive_or_dead([Alive, Dead, Dead, Dead, Dead, Dead, Alive, Alive]);
	println!("{}", alive.as_number());
}
