use std::{thread, time::Duration};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

const SHAPES: [[[[u8; 4]; 4]; 4]; 7] = [
    // I-Shape
    [
        [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
        [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
    ],
    // L-Shape
    [
        [[0, 0, 0, 0], [0, 1, 1, 1], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 1], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 0, 0, 1], [0, 1, 1, 1], [0, 0, 0, 0]],
        [[0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
    ],
    // J-Shape
    [
        [[0, 0, 0, 0], [0, 1, 1, 1], [0, 0, 0, 1], [0, 0, 0, 0]],
        [[0, 0, 1, 1], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    ],
    // O-Shape
    [
        [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    ],
    // S-Shape
    [
        [[0, 0, 0, 0], [0, 0, 1, 1], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 0, 1, 1], [0, 0, 0, 1], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 0, 1, 1], [0, 1, 1, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
    ],
    // T-Shape
    [
        [[0, 0, 0, 0], [0, 1, 1, 1], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 0, 1, 1], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 1, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
    ],
    // Z-Shape
    [
        [[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [0, 0, 0, 0]],
        [[0, 0, 0, 1], [0, 0, 1, 1], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
    ],
];

#[derive(Clone, Copy, Debug)]
enum TetrominoKind {
    I,
    L,
    J,
    O,
    S,
    T,
    Z,
}

#[derive(Clone, Copy, Debug)]
struct Tetromino {
    kind: TetrominoKind,
    rotation: usize,
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Occupied,
}

struct GameState {
    board: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
    active_piece: Option<Tetromino>,
}

impl GameState {
    fn new() -> Self {
        let mut game = GameState {
            board: [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            active_piece: None,
        };

        game.spawn_piece();
        game
    }

    fn update(&mut self) {
        if let Some(piece) = self.active_piece {
            // Create a potential next position
            let mut next_pos = piece;
            next_pos.y += 1;

            if self.is_valid_position(&next_pos) {
                self.active_piece = Some(next_pos);
            } else {
                self.lock_piece(piece);
                self.spawn_piece();
            }
        }
    }

    fn lock_piece(&mut self, piece: Tetromino) {
        let shape = SHAPES[piece.kind as usize][piece.rotation];
        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] == 1 {
                    let board_x = piece.x as usize + x;
                    let board_y = piece.y as usize + y;
                    if board_y < BOARD_HEIGHT && board_x < BOARD_WIDTH {
                        self.board[board_y][board_x] = Cell::Occupied;
                    }
                }
            }
        }

        self.active_piece = None;
    }

    fn is_valid_position(&self, piece: &Tetromino) -> bool {
        let shape = SHAPES[piece.kind as usize][piece.rotation];

        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] == 1 {
                    let board_x = piece.x + x as isize;
                    let board_y = piece.y + y as isize;

                    if board_x < 0
                        || board_x >= BOARD_WIDTH as isize
                        || board_y >= BOARD_HEIGHT as isize
                    {
                        return false;
                    }

                    if self.board[board_y as usize][board_x as usize] == Cell::Occupied {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn spawn_piece(&mut self) {
        self.active_piece = Some(Tetromino {
            kind: TetrominoKind::L,
            rotation: 0,
            x: 3,
            y: 0,
        })
    }
}

fn render(game_state: &GameState) {
    println!("\x1B[2J\x1B[1;1H");

    let mut display_board = game_state.board;

    if let Some(piece) = game_state.active_piece {
        let shape = SHAPES[piece.kind as usize][piece.rotation];
        for y in 0..4 {
            for x in 0..4 {
                if shape[y][x] == 1 {
                    let board_x = piece.x as usize + x;
                    let board_y = piece.y as usize + y;
                    if board_y < BOARD_HEIGHT && board_x < BOARD_WIDTH {
                        display_board[board_y][board_x] = Cell::Occupied;
                    }
                }
            }
        }
    }

    println!("--- Rusty TETRIS ---");
    for row in display_board.iter() {
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

    loop {
        render(&game_state);
        game_state.update();

        thread::sleep(Duration::from_millis(200));
    }
}
