use std::fmt::Display;
use std::ops::ControlFlow;

use crate::pieces::Direction::{
    East, EastNorthEast, EastSouthEast, North, NorthEast, NorthNorthEast, NorthNorthWest,
    NorthWest, South, SouthEast, SouthSouthEast, SouthSouthWest, SouthWest, West, WestNorthWest,
    WestSouthWest,
};
use crate::{board::Board, CursorPosition};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", match self {Color::Black => "SCHWARZ", Color::White => "WEIẞ",})
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceVariant {
    Pawn = 0x2659,
    Knight = 0x2658,
    Bishop = 0x2657,
    Rook = 0x2656,
    Queen = 0x2655,
    King = 0x2654,
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    NorthNorthEast,
    EastNorthEast,
    EastSouthEast,
    SouthSouthEast,
    SouthSouthWest,
    WestSouthWest,
    WestNorthWest,
    NorthNorthWest,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub col: i8,
    pub row: i8,
}

impl From<CursorPosition> for Position {
    fn from(value: CursorPosition) -> Self {
        Position {
            col: value.col,
            row: value.row,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub variant: PieceVariant,
    pub position: Position,
    pub color: Color,
}

impl Piece {
    pub fn new(variant: PieceVariant, position: Position, color: Color) -> Self {
        Self {
            variant,
            position,
            color,
        }
    }

    /// returns a vec of the possible moves of a specific piece on a given board
    pub(crate) fn get_available_moves(&self, board: &Board) -> Vec<Position> {
        let mut valid_moves = Vec::new();
        match self.variant {
            PieceVariant::Pawn => match self.color {
                Color::Black => {
                    if self.position.row == 1 {
                        valid_moves.append(self.generate_pawn_moves_in_direction(South, 2, board).as_mut());
                    } else {
                        valid_moves.append(self.generate_pawn_moves_in_direction(South, 1, board).as_mut());
                    }
                }
                Color::White => {
                    if self.position.row == 6 {
                        valid_moves.append(self.generate_pawn_moves_in_direction(North, 2, board).as_mut());
                    } else {
                        valid_moves.append(self.generate_pawn_moves_in_direction(North, 1, board).as_mut());
                    }
                }
            },
            PieceVariant::Knight => {
                valid_moves.append(self.generate_moves_in_direction(NorthNorthEast, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(EastNorthEast, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(EastSouthEast, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(SouthSouthEast, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(SouthSouthWest, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(WestSouthWest, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(WestNorthWest, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(NorthNorthWest, 1, board).as_mut(),
                );
            }
            PieceVariant::Bishop => {
                valid_moves.append(self.generate_moves_in_direction(NorthEast, 8, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(SouthEast, 8, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(SouthWest, 8, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(NorthWest, 8, board).as_mut(),);
            }
            PieceVariant::Rook => {
                valid_moves.append(self.generate_moves_in_direction(North, 8, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(East, 8, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(South, 8, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(West, 8, board).as_mut());
            }
            PieceVariant::Queen => {
                valid_moves.append(self.generate_moves_in_direction(North, 8, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(NorthEast, 8, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(East, 8, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(SouthEast, 8, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(South, 8, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(SouthWest, 8, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(West, 8, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(NorthWest, 8, board).as_mut(),);
            }
            PieceVariant::King => {
                valid_moves.append(self.generate_moves_in_direction(North, 1, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(NorthEast, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(East, 1, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(SouthEast, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(South, 1, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(SouthWest, 1, board).as_mut(),);
                valid_moves.append(self.generate_moves_in_direction(West, 1, board).as_mut());
                valid_moves.append(self.generate_moves_in_direction(NorthWest, 1, board).as_mut(),);
            }
        }
        valid_moves
    }

    fn generate_moves_in_direction(&self, direction: Direction, range: u8, board: &Board, ) -> Vec<Position> {
        let mut ret_vec = Vec::new();
        let next_move = dir_to_offset(direction);
        for walk_length in 1..=range {
            let new_move = Position::new(self.position.col + (walk_length as i8 * next_move.col), self.position.row + (walk_length as i8 * next_move.row),);
            if let ControlFlow::Break(_) = self.check_next_position(new_move, board, &mut ret_vec) {
                break;
            }
        }
        ret_vec
    }

    fn check_next_position(&self, new_move: Position, board: &Board, ret_vec: &mut Vec<Position>) -> ControlFlow<()> {
        if is_move_on_field(new_move) {
            match field_color(new_move, board) {
                Some(c) => {
                    if c == self.color {
                        return ControlFlow::Break(());
                    } else {
                        ret_vec.push(new_move);
                        return ControlFlow::Break(());
                    }
                },
                None => ret_vec.push(new_move),
            }
        } else {
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    }
    
    fn generate_pawn_moves_in_direction(&self, direction: Direction, range: u8, board: &Board, ) -> Vec<Position> {
        let mut ret_vec = Vec::new();
        let next_move = dir_to_offset(direction);
        for walk_length in 1..=range {
            let new_move = Position::new(self.position.col + (walk_length as i8 * next_move.col), self.position.row + (walk_length as i8 * next_move.row));
            if is_move_on_field(new_move) && field_color(new_move, board).is_none() {
                ret_vec.push(new_move)
            } else {
                // if one move is invalid in one direction, the ones behind it will be invalid as well
                break;
            }
        }
        let left_diagonal = Position::new(self.position.col - 1, self.position.row + next_move.row);
        let right_diagonal = Position::new(self.position.col + 1, self.position.row + next_move.row);
        if is_move_on_field(left_diagonal) && field_color(left_diagonal, board).is_some_and(|c| c != self.color) { 
            ret_vec.push(left_diagonal);
        }
        if is_move_on_field(right_diagonal) && field_color(right_diagonal, board).is_some_and(|c| c != self.color) {
            ret_vec.push(right_diagonal);
        }
        ret_vec
    }

    /// transform a pawn into a desired piece_var if it reached the 0th or 7th row in the array
    pub(crate) fn promote(&mut self, piece_variant: PieceVariant) -> Result<PieceVariant, &str> {
        if self.color == Color::White && self.position.col == 0 || self.color == Color::Black && self.position.col == 7 {
            self.variant = piece_variant;
            Ok(piece_variant)
        } else {
            Err("This piece is not qualified for conversion.")
        }
    }

    pub(crate) fn get_figure(self) -> u32 {
        match self.color {
            Color::Black => self.variant.get_figure() + 6,
            Color::White => self.variant.get_figure(),
        }
    }

    pub fn is_black(self) -> bool {
        matches!(self.color, Color::Black)
    }
}

impl PieceVariant {
    pub(crate) fn get_figure(self) -> u32 {
        self as u32
    }
}

impl Position {
    pub fn new(col: i8, row: i8) -> Self {
        Self { col, row }
    }
}

// Helper functions

/// check if moves go beyond chess field
fn is_move_on_field(check_move: Position) -> bool {
    check_move.row >= 0 && check_move.row <= 7 && check_move.col >= 0 && check_move.col <= 7
}

/// check if moves go on a colors field, and if so which ones
fn field_color(check_move: Position, board: &Board) -> Option<Color> {
    if board[check_move].is_none() {
        None
    } else {
        //unwrap is safe bc of is none chck beforehand
        Some(board[check_move].unwrap().color)
    }
}

/// return offset values for showing possible straight/ diagonal lines
fn dir_to_offset(direction: Direction) -> Position {
    match direction {
        North => Position::new(0, -1),
        NorthEast => Position::new(1, -1),
        East => Position::new(1, 0),
        SouthEast => Position::new(1, 1),
        South => Position::new(0, 1),
        SouthWest => Position::new(-1, 1),
        West => Position::new(-1, 0),
        NorthWest => Position::new(-1, -1),
        NorthNorthEast => Position::new(1, -2),
        EastNorthEast => Position::new(2, -1),
        EastSouthEast => Position::new(2, 1),
        SouthSouthEast => Position::new(1, 2),
        SouthSouthWest => Position::new(-1, 2),
        WestSouthWest => Position::new(-2, 1),
        WestNorthWest => Position::new(-2, -1),
        NorthNorthWest => Position::new(-1, -2),
    }
}
