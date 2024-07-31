use crate::levels::LEVEL1;
pub const WIDTH: usize = 24;
pub const HEIGHT: usize = 23;

pub struct Maze {
    pub grid: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    pub fn new() -> Self {
        let level = LEVEL1;
        let height = level.len();
        let width = level[0].len();
        let mut grid = vec![vec![false; width]; height];
        for y in 0..height {
            for x in 0..width {
                grid[y][x] = level[y][x] == 1;
            }
        }
        Maze { grid, width, height }
    }
    fn load_from_level(&mut self, level: [[u8; WIDTH]; HEIGHT]) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.grid[y][x] = level[y][x] == 1;
            }
        }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }

    pub fn is_spawn_area(&self, x: usize, y: usize) -> bool {
            LEVEL1[y][x] == 0
       
    }
    
}
