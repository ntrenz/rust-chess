use crate::board::Board;
use crate::pieces::{Color as PieceColor, Piece, Position};

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, PrintStyledContent, ResetColor, SetBackgroundColor,SetForegroundColor, Stylize,},
    terminal,
};
use std::ops::Div;
use std::io::{self, Write};

const BOARD_SIZE_X: u16 = 16;
const BOARD_SIZE_Y: u16 = 8;
const X_OFFSET_BORDER: u16 = 7;
const X_OFFSET_BOARD: u16 = 8;
const Y_OFFSET_BOARD: u16 = 6;

fn unicode_to_character(piece: &Piece) -> char {
    std::char::from_u32(piece.get_figure()).unwrap_or('�')
}

pub fn position_to_character(int: u16) -> Option<char> {
    match int {
        1 => Some('A'),
        2 => Some('B'),
        3 => Some('C'),
        4 => Some('D'),
        5 => Some('E'),
        6 => Some('F'),
        7 => Some('G'),
        8 => Some('H'),
        ..=u16::MAX => None,
    }
}

pub fn init_display(board: &Board, cursor_pos: &(u16, u16), move_set: &Vec<Position>) -> io::Result<()> {
    const X_OFFSET_BORDER: u16 = 6;
    const Y_OFFSET_BORDER: u16 = 5;

    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    queue!(
        stdout,
        cursor::MoveTo(0, BOARD_SIZE_X + 1),
        PrintStyledContent("Bedienung:".italic()),
        Print("\n Bewegung: Pfeiltasten ←→↑↓\n Figur auswählen: ENTER\n Figur Bewegen: Pfeiltasten ←→↑↓; Besätigung mit ENTER\n Figur abwählen: BACKSPACE\n Spiel beenden: ESC")
    )?;

    queue!(
        stdout,
        cursor::MoveTo(3, 2),
        SetBackgroundColor(Color::Blue),
        Print("Willkommen zu "),
        PrintStyledContent("rusty-chess".italic()),
        SetBackgroundColor(Color::Blue),
        Print("!")
    )?;

    for y in 0..17 {
        for x in 0..32 {
            if (y == 0 || y == 17 - 1) || (x == 0 || x == 32 - 1) {
                queue!(stdout, cursor::MoveTo(x, y), Print(" "))?;
            }
        }
    }

    // draw border
    for x in 0..BOARD_SIZE_X + 4 {
        for y in 0..BOARD_SIZE_Y + 2 {
            if y == 0 || y == BOARD_SIZE_Y + 1 {
                queue!(
                    stdout,
                    cursor::MoveTo(x + X_OFFSET_BORDER, y + Y_OFFSET_BORDER),
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(Color::Grey),
                    if x % 2 == 0 {
                        match position_to_character(x.div_ceil(2)) {
                            Some(c) => PrintStyledContent(c.to_string().italic()),
                            None => PrintStyledContent(" ".to_string().stylize()),
                        }
                    } else {
                        PrintStyledContent(" ".to_string().stylize())
                    }
                )?;
            } else if x == 1 || x == BOARD_SIZE_X + 2 {
                if y == 0 || y == 9 {
                    continue;
                }
                queue!(
                    stdout,
                    cursor::MoveTo(x + X_OFFSET_BORDER, y + Y_OFFSET_BORDER),
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(Color::Grey),
                    PrintStyledContent((BOARD_SIZE_Y + 1 - y).to_string().italic())
                )?;
            } else if x == 0 || x == BOARD_SIZE_X + 3 {
                if y == 0 || y == 9 {
                    continue;
                }
                queue!(
                    stdout,
                    cursor::MoveTo(x + X_OFFSET_BORDER, y + Y_OFFSET_BORDER),
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(Color::Grey),
                    Print(" ")
                )?;
            }
        }
    }

    display_board(board, cursor_pos, move_set)
}

pub fn display_board(board: &Board, cursor_pos: &(u16, u16), move_set: &Vec<Position>) -> io::Result<()> {   
    let mut stdout = io::stdout();

    // draw pieces onto board
    for x in (0..BOARD_SIZE_X).step_by(2) {
        for y in 0..BOARD_SIZE_Y {
            let board_coordinate_x = x.div(2);
            let piece = board.board_array[y as usize][board_coordinate_x as usize];
            if (board_coordinate_x + y) % 2 == 0 {
                // white
                display_field(x, y, piece, &stdout, Color::Rgb { r: 234, g: 234, b: 234 })?;
            } else {
                // black
                display_field(x, y, piece, &stdout, Color::Rgb { r: 118, g: 147, b: 84 })?;
            }
        }
    }

    // display available moves
    for i in move_set.into_iter() {
        let board_coordinate_x: u16 = i.col as u16 * 2;
        let piece = board[*i];
        queue!(
            stdout,
            cursor::MoveTo(board_coordinate_x + X_OFFSET_BOARD, i.row as u16 + Y_OFFSET_BOARD),
            SetBackgroundColor(Color::Rgb { r: 249, g: 215, b: 0 }), // rgb(249, 215, 0)
            match piece {
                Some(piece) => {
                    match piece.color {
                        PieceColor::Black => SetForegroundColor(Color::Black),
                        PieceColor::White => SetForegroundColor(Color::White),
                    };
                    Print(unicode_to_character(&piece).to_string())
                },
                None => Print(" ".to_string()),
            }
        )?;
        queue!(
            stdout,
            cursor::MoveTo(board_coordinate_x + X_OFFSET_BOARD + 1, i.row as u16 + Y_OFFSET_BOARD),
            SetBackgroundColor(Color::Rgb { r: 249, g: 215, b: 0 }),
            Print(" ")
        )?;
    }

    let selected_piece = board.board_array[cursor_pos.1 as usize][cursor_pos.0 as usize];
    // draw cursor
    queue!(
        stdout,
        cursor::MoveTo(cursor_pos.0 * 2 + X_OFFSET_BOARD , cursor_pos.1 + Y_OFFSET_BOARD),
        SetBackgroundColor(Color::Red),
        match selected_piece {
            Some(piece) => Print(unicode_to_character(&piece).to_string()),
            None => Print(" ".to_string()),
        },
        ResetColor
    )?;
    queue!(
        stdout,
        cursor::MoveTo(cursor_pos.0 * 2 + X_OFFSET_BOARD + 1, cursor_pos.1 + Y_OFFSET_BOARD),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print(" "),
        ResetColor
    )?;

    queue!(stdout, cursor::MoveTo(0, 17))?;

    stdout.flush()?;
    Ok(())
}

fn display_field(x: u16, y: u16, piece: Option<Piece>, mut stdout: &io::Stdout, background_color: Color) -> io::Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(x + X_OFFSET_BOARD, y + Y_OFFSET_BOARD),
        SetBackgroundColor(background_color), // Color::White -> Beige 234, 234, 208
        match piece {
            Some(piece) => {
                match piece.color {
                    PieceColor::Black => SetForegroundColor(Color::Black),
                    PieceColor::White => SetForegroundColor(Color::White),
                };
                Print(unicode_to_character(&piece))
            },
            None => Print(' '),
        }
    )?;
    queue!(
        stdout,
        cursor::MoveTo(x + X_OFFSET_BORDER + 2, y + Y_OFFSET_BOARD),
        SetBackgroundColor(background_color),
        Print(" ")
    )
}

pub fn display_winner(color: PieceColor) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(
        stdout,
        cursor::MoveTo(2, 4),
    )?;

    println!("Spieler {:} hat gewonnen!", color);

    queue!(
        stdout,
        cursor::MoveTo(0, BOARD_SIZE_X + 6),
        Print(" ")
    )?;

    Ok(())
}