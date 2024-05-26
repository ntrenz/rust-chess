use crate::pieces::Color::{Black, White};
use crate::pieces::{Color, Piece, PieceVariant, Position};
use std::any::Any;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};

pub struct Board {
    // piece_store: Vec<Piece>,
    beaten_white_pieces: Vec<Piece>,
    beaten_black_pieces: Vec<Piece>,
    // to be used as [[rows]columns]
    pub board_array: [[Option<Piece>; 8]; 8],
}

impl Index<Position> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.board_array[index.row as usize][index.col as usize]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.board_array[index.row as usize][index.col as usize]
    }
}

impl Board {
    pub fn new() -> Self {
        Board{ beaten_white_pieces: vec![], beaten_black_pieces: vec![], board_array: Default::default() }
    }

    // initializes a board with the default chess layout
    pub fn initialize(&mut self) {
        Self::fill_with_pieces(&mut self.board_array[0], 0, Black);
        Self::fill_with_pawns(&mut self.board_array[1], 1, Black);
        Self::fill_with_pawns(&mut self.board_array[6], 6, White);
        Self::fill_with_pieces(&mut self.board_array[7], 7, White);
    }

    // clears the Board
    pub fn clear(&mut self) {
        todo!("Do it without piece store")
        // self.piece_store.clear();
    }

    // build a chessboard from an input string in chess notation
    pub fn import_from_str() -> Self {
        todo!()
    }

    pub fn update_field(&mut self, old_piece_pos: Position, new_piece_pos: Position) -> Result<(), ()> {
        if let Some(p) = self[new_piece_pos].take() {
            if p.is_black() {
                self.beaten_black_pieces.push(p);
            } else {
                self.beaten_white_pieces.push(p);
            }
            if p.variant == PieceVariant::King {
                return Err(());
            }
        }
        self[new_piece_pos] = self[old_piece_pos].take();
        self[new_piece_pos].as_mut().unwrap().position = new_piece_pos; // we know there is a piece on this position. This was a nasty line of code (；′⌒`)
        return Ok(());
    }

    fn fill_with_pieces(row: &mut [Option<Piece>; 8], row_index: i8, color: Color) {
        for (col_index, elem) in row.iter_mut().enumerate() {
            // col_index as i8 is safe in the case because the possible values are in the range of 0..7
            if col_index == 0 || col_index == 7 {
                *elem = Some(Piece::new(
                    PieceVariant::Rook,
                    Position::new(col_index as i8, row_index),
                    color,
                ));
            } else if col_index == 1 || col_index == 6 {
                *elem = Some(Piece::new(
                    PieceVariant::Knight,
                    Position::new(col_index as i8, row_index),
                    color,
                ));
            } else if col_index == 2 || col_index == 5 {
                *elem = Some(Piece::new(
                    PieceVariant::Bishop,
                    Position::new(col_index as i8, row_index),
                    color,
                ));
            } else if col_index == 3 {
                *elem = Some(Piece::new(
                    PieceVariant::Queen,
                    Position::new(col_index as i8, row_index),
                    color,
                ));
            } else {
                *elem = Some(Piece::new(
                    PieceVariant::King,
                    Position::new(col_index as i8, row_index),
                    color,
                ));
            }
        }
    }

    fn fill_with_pawns(row: &mut [Option<Piece>; 8], row_index: i8, color: Color) {
        for (col_index, elem) in row.iter_mut().enumerate() {
            // col_index as i8 is safe in the case because the possible values are in the range of 0..7
            *elem = Some(Piece::new(
                PieceVariant::Pawn,
                Position::new(col_index as i8, row_index),
                color,
            ));
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // output the state of the board in chess notation
        let mut piece_str = String::from("");
        for p in self.board_array.iter().flatten().flatten() {
            piece_str.push(match p.variant {
                PieceVariant::Pawn => 'P',
                PieceVariant::Rook => 'R',
                PieceVariant::Knight => 'N',
                PieceVariant::Bishop => 'B',
                PieceVariant::Queen => 'Q',
                PieceVariant::King => 'K',
            });

            piece_str.push(match p.position {
                Position { col: _row, row: col } => match col {
                    0 => 'A',
                    1 => 'B',
                    2 => 'C',
                    3 => 'D',
                    4 => 'E',
                    5 => 'F',
                    6 => 'G',
                    7 => 'H',
                    _ => '?',
                },
            });

            piece_str.push(match p.position {
                Position { col: row, row: _col } => match row {
                    0 => '8',
                    1 => '7',
                    2 => '6',
                    3 => '5',
                    4 => '4',
                    5 => '3',
                    6 => '2',
                    7 => '1',
                    _ => '?',
                },
            });

            piece_str.push(' ');
        }
        write!(f, "{}", piece_str)
    }
}
