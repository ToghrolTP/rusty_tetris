use std::{thread, time::Duration};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Occupied,
}

struct GameState {
    board: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl GameState {
    fn new() -> Self {
        GameState {
            board: [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

fn render(game_state: &GameState) {
    println!("\x1B[2J\x1B[1;1H");
    println!("--- Rusty TETRIS ---");

    for row in game_state.board.iter() {
        for &cell in row.iter() {
            match cell {
                Cell::Empty => print!(". "),
                Cell::Occupied => print!("# "),
            }
        }

        println!();
    }
}

fn main() {
    let mut game_state = GameState::new();
    game_state.board[0][4] = Cell::Occupied;
    game_state.board[1][4] = Cell::Occupied;
    game_state.board[19][5] = Cell::Occupied;

    loop {
        render(&game_state);

        thread::sleep(Duration::from_millis(500));
    }
}
