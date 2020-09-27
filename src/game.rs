/// A cell in the grid.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Cell {
    pub alive: bool,
}

/// The game state.
pub struct Game {
/// Width of the grid.
pub width: u16,
pub height: u16,

/// the actual map
grid: Box<[Cell]>,
}

impl Game {
    
    pub fn new(w: u16, h: u16) -> Game {
        Game {
            width: w,
            height: h,
            grid: vec![Cell {
                alive: false,
            }; w as usize * h as usize].into_boxed_slice(),
        }
    }

    /// Get the grid position of a given coordinate.
    fn pos(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }

    /// Get the cell at (x, y).
    pub fn get(&self, x: u16, y: u16) -> Cell {
        let pos = self.pos(x, y);
        self.grid[pos]
    }

    /// Get a mutable reference to the cell at (x, y).\
    pub fn toggle(&mut self, x: u16, y: u16) {
        let pos = self.pos(x, y);
        let is_alive = self.grid[pos].alive;
        self.grid[pos].alive = !is_alive;
    }

    /// Reset the game.
    ///
    /// This will display the starting grid
    pub fn reset(&mut self) {
        // Reset the grid.
        for i in 0..self.grid.len() {
            // all is dead
            self.grid[i] = Cell {
                alive: false,
            };
        }
    }

    /// Calculate the y coordinate of the cell "above" a given y coordinate.
    ///
    /// This wraps when _y = 0_.
    pub fn up(&self, y: u16) -> u16 {
        if y == 0 {
            // Upper bound reached. Wrap around.
            self.height - 1
        } else {
            y - 1
        }
    }
    /// Calculate the y coordinate of the cell "below" a given y coordinate.
    ///
    /// This wraps when _y = h - 1_.
    pub fn down(&self, y: u16) -> u16 {
        if y + 1 == self.height {
            // Lower bound reached. Wrap around.
            0
        } else {
            y + 1
        }
    }
    /// Calculate the x coordinate of the cell "left to" a given x coordinate.
    ///
    /// This wraps when _x = 0_.
    pub fn left(&self, x: u16) -> u16 {
        if x == 0 {
            // Lower bound reached. Wrap around.
            self.width - 1
        } else {
            x - 1
        }
    }
    /// Calculate the x coordinate of the cell "left to" a given x coordinate.
    ///
    /// This wraps when _x = w - 1_.
    pub fn right(&self, x: u16) -> u16 {
        if x + 1 == self.width {
            // Upper bound reached. Wrap around.
            0
        } else {
            x + 1
        }
    }

    pub fn step(&mut self) {
        self.grid = self.int_step();
    }

    fn wrapCoord(&self, i: i32) -> usize{
        let w = self.width as i32;
        let mut x = i % w;
        let mut y = i / w;
        if x < 0{
            x = w + x;
        }
        if y < 0{
            y = w + y;
        }
        (y * w + x) as usize
    }

    fn int_step(&self) -> Box<[Cell]>{
        let mut new_grid = vec![Cell {
            alive: false,
        }; self.width as usize * self.height as usize].into_boxed_slice();
        
        for i in 0..self.grid.len() {
            let is_alive = self.grid[i].alive;
            let is = i as i32;
            let w = self.width as usize;
            
            let x = i % w;
            let y = i / w;

            let neighbors = vec![
                (x, y - 1),
                (x+1, y-1),
                (x + 1, y),
                (x+1, y+1),
                (x, y+1),
                (x-1, y+1),
                (x-1, y),
                (x-1, y-1),
                (x-1, y),
                (x-1, y + 1),
            ].into_iter(|x| self.wrapCoord(x)).collect();

            let mut alive_neighbors_count = 0;
            for &n in neighbors.iter() {
                if self.grid[n as usize].alive{
                    alive_neighbors_count += 1;
                }
            }
            
            new_grid[i].alive = self.grid[i].alive;

            if alive_neighbors_count > 2{
                new_grid[i].alive = true;
            }

            if is_alive && alive_neighbors_count < 2 {
                new_grid[i].alive = false;
            }
        }
        new_grid
    }
}