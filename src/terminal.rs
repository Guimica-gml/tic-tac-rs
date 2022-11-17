use std::io::{Write, stdout, stdin};
use termion::{self, cursor, clear, color};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

use super::tac::*;
use super::ai::*;

pub fn main() -> std::io::Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;
    let mut stdin = stdin().keys();

    let mut tic_tac_toe = TicTacToe::new(3, 3);
    let mut ai = TicTacToeAi::new();

    let mut cursor_x = 0;
    let mut cursor_y = 0;

    write!(
        stdout, "{}{}",
        clear::All,
        cursor::Hide,
    )?;

    loop {
        write!(stdout, "{}", cursor::Goto(1, 1))?;

        for y in 0..tic_tac_toe.height() {
            print!(" ");
            for x in 0..tic_tac_toe.width() {
                if cursor_x == x && cursor_y == y {
                    write!(
                        stdout, "{}{}",
                        color::Bg(color::White),
                        color::Fg(color::Black),
                    )?;
                }
                else {
                    write!(
                        stdout, "{}{}",
                        color::Bg(color::Reset),
                        color::Fg(color::Reset),
                    )?;
                }

                write!(
                    stdout,
                    "{}{}{}",
                    tic_tac_toe.get_cell(x, y),
                    color::Bg(color::Reset),
                    color::Fg(color::Reset),
                )?;

                if x != tic_tac_toe.width() - 1 {
                    print!(" | ");
                }
            }
            if y != tic_tac_toe.height() - 1 {
                print!("\r\n");
                for i in 0..tic_tac_toe.width() {
                    print!("---");
                    if i != tic_tac_toe.width() - 1 {
                        print!("+");
                    }
                }
            }
            print!("\r\n");
        }

        match tic_tac_toe.check_winner() {
            State::X => {
                write!(stdout, "X win!\r\n")?;
                break;
            },
            State::O => {
                write!(stdout, "O win!\r\n")?;
                break;
            },
            _ if tic_tac_toe.is_full() => {
                write!(stdout, "Draw!\r\n")?;
                break;
            }
            _ => {}
        }

        match stdin.next().unwrap()? {
            Key::Left if cursor_x > 0 => cursor_x -= 1,
            Key::Right if cursor_x < tic_tac_toe.width() - 1 => cursor_x += 1,
            Key::Up if cursor_y > 0 => cursor_y -= 1,
            Key::Down if cursor_y < tic_tac_toe.height() - 1 => cursor_y += 1,
            Key::Ctrl('c') | Key::Esc => break,
            Key::Char('\n') if tic_tac_toe.is_cell_empty(cursor_x, cursor_y) => {
                tic_tac_toe.next(cursor_x, cursor_y);
                match ai.next_move(&tic_tac_toe) {
                    Some((x, y)) => tic_tac_toe.next(x, y),
                    None => {}
                }
            }
            _ => {}
        }
    }

    write!(stdout, "{}", cursor::Show)?;
    stdout.suspend_raw_mode()?;
    Ok(())
}
