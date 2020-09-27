extern crate termion;

use crate::Game;

use termion::cursor;

use std::io::Write;

/// The upper and lower boundary char.
const HORZ_BOUNDARY: &'static str = "─";
/// The left and right boundary char.
const VERT_BOUNDARY: &'static str = "│";

/// The top-left corner
const TOP_LEFT_CORNER: &'static str = "┌";
/// The top-right corner
const TOP_RIGHT_CORNER: &'static str = "┐";
/// The bottom-left corner
const BOTTOM_LEFT_CORNER: &'static str = "└";
/// The bottom-right corner
const BOTTOM_RIGHT_CORNER: &'static str = "┘";

const CELL_ALIVE: &'static str = "█";
const CELL_DEAD: &'static str = "░";

pub struct GameIO<W: Write> {
  /// Standard output.
  pub stdout: W,

  /// The cursor coordinate.
  pub x: u16,
  pub y: u16,
}

impl<W: Write> GameIO<W> {
  pub fn print_border(&mut self, game: &Game) {
    // Reset the cursor.
    write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();

    // Write the upper part of the frame.
    self.stdout.write(TOP_LEFT_CORNER.as_bytes()).unwrap();
    for _ in 0..game.width {
      self.stdout.write(HORZ_BOUNDARY.as_bytes()).unwrap();
    }
    self.stdout.write(TOP_RIGHT_CORNER.as_bytes()).unwrap();
    self.stdout.write(b"\n\r").unwrap();

    // Conceal all the cells.
    for h in 0..game.height {
      // The left part of the frame
      self.stdout.write(VERT_BOUNDARY.as_bytes()).unwrap();

      // The right part of the frame
      write!(self.stdout, "{}", cursor::Goto(game.width + 2, h + 2)).unwrap();
      self.stdout.write(VERT_BOUNDARY.as_bytes()).unwrap();
      self.stdout.write(b"\n\r").unwrap();
    }

    // Write the lower part of the frame.
    self.stdout.write(BOTTOM_LEFT_CORNER.as_bytes()).unwrap();
    for _ in 0..game.width {
      self.stdout.write(HORZ_BOUNDARY.as_bytes()).unwrap();
    }
    self.stdout.write(BOTTOM_RIGHT_CORNER.as_bytes()).unwrap();

    write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
    self.stdout.flush().unwrap();
  }

  pub fn print_map(&mut self, game: &Game) {
    for h in 0..game.height {
      write!(self.stdout, "{}", cursor::Goto(2, h + 2)).unwrap();
      for w in 0..game.width {
        let cell = if game.get(w, h).alive {
          CELL_ALIVE
        } else {
          CELL_DEAD
        };
        self.stdout.write(cell.as_bytes()).unwrap();
      }
    }
  }
}
