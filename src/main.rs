//! This crate is a simple implementation of minesweeper. It is carefully documented to encourage

//! newbies to add new games to the repository.

extern crate termion;

//use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use std::env;
use std::io::{self, Read, Write};
use std::process;

mod game;
mod game_io;

use game::Game;
use game_io::GameIO;

/// The help page.
const HELP: &'static str = r#"
minesweeper ~ a simple minesweeper implementation.
rules:
    Select a cell to reveal, printing the number of adjacent cells holding a mine.
    If no adjacent cells hold a mine, the cell is called free. Free cell will recursively
    reveal their neighboring cells. If a mine is revealed, you loose. The grid wraps.
flags:
    -r | --height N ~ set the height of the grid.
    -c | --width N  ~ set the width of the grid.
    -h | --help     ~ this help page.
    -b              ~ beginner mode.
    -i              ~ intermediate mode.
    -a              ~ advanced mode.
    -g              ~ god mode.
controls:
    ---selection--------------------
    space ~ reveal the current cell.
    ---movement---------------------
    h | a ~ move left.
    j | s ~ move down.
    k | w ~ move up.
    l | d ~ move right.
    ---flags------------------------
    f     ~ set flag.
    F     ~ remove flag.
    ---control----------------------
    q     ~ quit game.
    r     ~ restart game.
author:
    ticki.
"#;

/// Initialize the game.
fn init<W: Write, R: Read>(mut stdout: W, stdin: R, w: u16, h: u16) {
  write!(stdout, "{}", clear::All).unwrap();

  // Set the initial game state.
  let mut game = Game::new(w, h);

  let mut game_io = GameIO {
    stdin: stdin.keys(),
    stdout: stdout,
    x: 0,
    y: 0,
  };

  // Reset that game.
  game.reset();

  game_io.print_border(&game);

  let mut autorun = false;

  let mut aync_stdin = termion::async_stdin().bytes();

  let ten_millis = std::time::Duration::from_millis(10);
  // Start the event loop.
  loop {
    game_io.print_map(&game);

    // Make sure the cursor is placed on the current position.
    write!(
      game_io.stdout,
      "{}",
      cursor::Goto(game_io.x + 2, game_io.y + 2)
    )
    .unwrap();
    game_io.stdout.flush().unwrap();


    use termion::event::Key::*;
    let b;
    if autorun{
        let a = aync_stdin.next();
        termion::event::Key::From(a);
    }else{
        b = game_io.stdin.read();
    }
    
    if let Some(b) = b{
        let b = b.unwrap();
        match b {
        Char('h') | Char('a') | Left => game_io.x = game.left(game_io.x),
        Char('j') | Char('s') | Down => game_io.y = game.down(game_io.y),
        Char('k') | Char('w') | Up => game_io.y = game.up(game_io.y),
        Char('l') | Char('d') | Right => game_io.x = game.right(game_io.x),
        Char(' ') => {
            // Check if it was a mine.
            let (x, y) = (game_io.x, game_io.y);
            game.toggle(x, y);
        }
        Char('r') => {
            game.reset();
        }
        Char('n') => {
            game.step();
        }
        Char('p') => {
            autorun = !autorun;
        }
        Char('q') => return,
        _ => {}
        }
    }

    if autorun {
        game.step();
        std::thread::sleep(ten_millis);
    }
  }
}

fn main() {
  let mut args = env::args().skip(1);
  let mut width: Option<u16> = None;
  let mut height: Option<u16> = None;

  // Get and lock the stdios.
  let stdout = io::stdout();
  let mut stdout = stdout.lock();
  let stdin = io::stdin();
  let stdin = stdin.lock();
  let stderr = io::stderr();
  let mut stderr = stderr.lock();

  loop {
    // Read the arguments.
    // Does not use a for loop because each argument may have second parameter.

    let arg = if let Some(x) = args.next() {
      x
    } else {
      break;
    };

    match arg.as_str() {
      "-r" | "--height" => {
        if height.is_none() {
          height = Some(
            args
              .next()
              .unwrap_or_else(|| {
                stderr.write(b"no height given.\n").unwrap();
                stderr.flush().unwrap();
                process::exit(1);
              })
              .parse()
              .unwrap_or_else(|_| {
                stderr.write(b"invalid integer given.\n").unwrap();
                stderr.flush().unwrap();
                process::exit(1);
              }),
          );
        } else {
          stderr.write(b"you may only input one height.\n").unwrap();
          stderr.flush().unwrap();
          process::exit(1);
        }
      }
      "-c" | "--width" => {
        if width.is_none() {
          width = Some(
            args
              .next()
              .unwrap_or_else(|| {
                stderr.write(b"no width given.\n").unwrap();
                stderr.flush().unwrap();
                process::exit(1);
              })
              .parse()
              .unwrap_or_else(|_| {
                stderr.write(b"invalid integer given.\n").unwrap();
                stderr.flush().unwrap();
                process::exit(1);
              }),
          );
        } else {
          stderr.write(b"you may only input one width.\n").unwrap();
          stderr.flush().unwrap();
          process::exit(1);
        }
      }
      "-h" | "--help" => {
        // Print the help page.
        stdout.write(HELP.as_bytes()).unwrap();
        stdout.flush().unwrap();
        process::exit(0);
      }
      _ => {
        stderr.write(b"Unknown argument.\n").unwrap();
        stderr.flush().unwrap();
        process::exit(1);
      }
    }
  }

  // We go to raw mode to make the control over the terminal more fine-grained.
  let stdout = stdout.into_raw_mode().unwrap();

  // let termsize = termion::terminal_size().ok();
  // let termwidth = termsize.map(|(w, _)| w - 2);
  // let termheight = termsize.map(|(_, h)| h - 2);

  // Initialize the game!
  init(stdout, stdin, 10, 10);

  //write!(stderr, "out was {} {}.\n", termwidth.unwrap_or(0), termheight.unwrap_or(0));
}
