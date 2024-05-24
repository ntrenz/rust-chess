pub mod board;
pub mod console;
pub mod pieces;
pub mod user_input;

use board::Board;
use console::*;
use pieces::{Piece, Position};
use user_input::KeyPress;

use std::cmp;

#[derive(Copy, Clone)]
pub struct CursorPosition {
    col: i8,
    row: i8,
}

impl CursorPosition {
    const MAX_POSITION: i8 = 7;
    const MIN_POSITION: i8 = 0;

    pub fn new() -> CursorPosition {
        CursorPosition { col: 0, row: 0 }
    }

    pub fn get_position(&self) -> (u16, u16) {
        (self.col as u16, self.row as u16)
    }

    fn calc_manhattan_metrik(&mut self, move_set: &Vec<Position>) -> Vec<i8> {
        move_set
            .iter()
            .map(|c| (self.col - c.col).abs() + (self.row - c.row).abs())
            .collect()
    }

    fn set_to_nearest_pos(&mut self, move_set: &Vec<Position>) {
        let metrik_vec = self.calc_manhattan_metrik(&move_set);
        match metrik_vec.iter().min() {
            Some(m) => {
                let min_index = metrik_vec.iter().position(|c| c == m).unwrap();
                self.col = move_set.get(min_index).unwrap().col;
                self.row = move_set.get(min_index).unwrap().row;
            }
            None => (),
        }
    }

    fn move_with_piece_up(&mut self, move_set: &Vec<Position>) {
        let move_set_vec = move_set
            .iter()
            .filter(|c| c.row < self.row)
            .copied()
            .collect();
        self.set_to_nearest_pos(&move_set_vec);
    }

    fn move_with_piece_down(&mut self, move_set: &Vec<Position>) {
        let move_set_vec = move_set
            .iter()
            .filter(|c| c.row > self.row)
            .copied()
            .collect();
        self.set_to_nearest_pos(&move_set_vec);
    }

    fn move_with_piece_left(&mut self, move_set: &Vec<Position>) {
        let move_set_vec = move_set
            .iter()
            .filter(|c| c.col < self.col)
            .copied()
            .collect();
        self.set_to_nearest_pos(&move_set_vec);
    }

    fn move_with_piece_right(&mut self, move_set: &Vec<Position>) {
        let move_set_vec = move_set
            .iter()
            .filter(|c| c.col > self.col)
            .copied()
            .collect();
        self.set_to_nearest_pos(&move_set_vec);
    }

    fn move_cursor_up(&mut self) {
        self.row = cmp::max(Self::MIN_POSITION, self.row - 1)
    }

    fn move_cursor_down(&mut self) {
        self.row = cmp::min(Self::MAX_POSITION, self.row + 1)
    }

    fn move_cursor_left(&mut self) {
        self.col = cmp::max(Self::MIN_POSITION, self.col - 1)
    }

    fn move_cursor_right(&mut self) {
        self.col = cmp::min(Self::MAX_POSITION, self.col + 1)
    }
}

fn main() {
    // init
    let mut board: Board = Board::new();
    board.initialize();
    let mut cursor_pos = CursorPosition::new();
    let mut selected_piece: Option<Piece> = None;
    let mut current_player_is_white = true;

    let mut move_set: Vec<Position> = vec![];

    // init console output
    match init_display(&board, &cursor_pos.get_position(), &move_set) {
        Ok(()) => (),
        Err(_) => (),
    };

    loop {
        // expect user input
        let Some(pressed_key) = user_input::await_user_input() else {
            continue;
        };

        match (selected_piece, pressed_key) {
            (None, KeyPress::Left) => cursor_pos.move_cursor_left(),
            (None, KeyPress::Right) => cursor_pos.move_cursor_right(),
            (None, KeyPress::Up) => cursor_pos.move_cursor_up(),
            (None, KeyPress::Down) => cursor_pos.move_cursor_down(),
            (None, KeyPress::Enter) => {
                if let Some(p) = board[cursor_pos.into()] {
                    if p.is_black() != current_player_is_white { // prevent white player form selecting black pieces and vice versa
                        selected_piece = Some(p);
                        move_set = p.get_available_moves(&board);
                    }
                }
            }
            (Some(_), KeyPress::Left) => cursor_pos.move_with_piece_left(&move_set),
            (Some(_), KeyPress::Right) => cursor_pos.move_with_piece_right(&move_set),
            (Some(_), KeyPress::Up) => cursor_pos.move_with_piece_up(&move_set),
            (Some(_), KeyPress::Down) => cursor_pos.move_with_piece_down(&move_set),
            (Some(p), KeyPress::Enter) => {
                // prevent placing selected_piece on its own square
                if p.position == cursor_pos.into() {
                    continue;
                }
                match board.update_field(p.position, cursor_pos.into()) {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = display_winner(selected_piece.unwrap().color);
                        break;
                    },
                };
                selected_piece = None;
                move_set = vec![];
                current_player_is_white = !current_player_is_white;
            }
            (_, KeyPress::Esc) => return,
            (_, KeyPress::BackSpace) => {
                selected_piece = None;
                move_set = vec![];
            }
        }

        // redraw board
        match display_board(&board, &cursor_pos.get_position(), &move_set) {
            Ok(()) => (),
            Err(_) => (),
        };
    }
}
