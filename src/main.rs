// ----- Crates ----- //

extern crate piston_window;
extern crate rand;
use piston_window::*;
use rand::{thread_rng, Rng};


// ----- Setup ----- //

// Size of the board.
const NO_COLS: usize = 60;
const NO_ROWS: usize = 60;

// Dimensions of the window.
const W_WIDTH: u32 = 640;
const W_HEIGHT: u32 = 640;

// Size of a cell rectangle.
const CELL_WIDTH: f64 = W_WIDTH as f64 / NO_COLS as f64;
const CELL_HEIGHT: f64 = W_HEIGHT as f64 / NO_ROWS as f64;
const RECT_WIDTH: f64 = CELL_WIDTH - 2.0;
const RECT_HEIGHT: f64 = CELL_HEIGHT - 2.0;

// Framerate.
const FPS: u64 = 2;

// Ratio of dead-to-alive, e.g. if the ratio is 10, 1 in 10 cells will start
// off alive.
const STATUS_RATIO: u8 = 20;

// ----- Enums ----- //

// The status of a cell.
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Status {
	Alive = 1,
	Dead = 0
}

// Brings status variants into scope.
use Status::*;

// Defines the type Board as a 2D array of Status.
type Board = [[Status; NO_COLS]; NO_ROWS];


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
fn update_status(board: &Board, row: usize, col: usize) -> Status {

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

// Updates the full board.
fn update_board(board: &Board) -> Board {

	let mut new_board: Board = [[Dead; NO_COLS]; NO_ROWS];

	for row in 0..NO_ROWS {
		for col in 0..NO_COLS {
			new_board[row][col] = update_status(board, row, col);
		}
	}

	new_board

}

// Generates a random cell status.
fn random_status() -> Status {

	let mut rng = thread_rng();
	let number: u8 = rng.gen_range(0, STATUS_RATIO);
	
	match number {
		1 => Alive,
		_ => Dead
	}

}

// Randomly generates a board of alive or dead cells.
fn random_board() -> Board {

	let mut board: Board = [[Dead; NO_COLS]; NO_ROWS];

	for row in 0..NO_ROWS {
		for col in 0..NO_COLS {
			board[row][col] = random_status();
		}
	}

	board

}

// Generates a random color.
fn random_colour() -> [f32; 4] {

	let mut rng = thread_rng();

	[
		rng.gen_range::<f32>(0.0, 1.0),
		rng.gen_range::<f32>(0.0, 1.0),
		rng.gen_range::<f32>(0.0, 1.0),
		1.0
	]

}

// Draws board on screen.
fn draw_board(board: &Board, context: &Context, graphics: &mut G2d) {

	for row in 0..NO_ROWS {
		for col in 0..NO_COLS {

			if board[row][col] == Alive {

				let x = (row as f64 * CELL_WIDTH) + 1.0;
				let y = (col as f64 * CELL_HEIGHT) + 1.0;

				rectangle(random_colour(), [x, y, RECT_WIDTH, RECT_HEIGHT],
					context.transform, graphics);

			}
			
		}
	}

}

// Launches the piston graphics.
fn run_graphics(board: &mut Board) {

	let window: PistonWindow =
		WindowSettings::new("Game Of Life", [W_WIDTH, W_HEIGHT])
			.exit_on_esc(true).build().unwrap();

	for e in window.max_fps(FPS) {

		e.draw_2d(|c, g| {

			clear([0.0; 4], g);
			draw_board(&board, &c, g);
			
			*board = update_board(&board);

		});

	}

}


// ----- Main ----- //

fn main() {

	let mut board = random_board();
	run_graphics(&mut board);

}
