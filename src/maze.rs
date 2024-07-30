use rand::Rng;

pub const WIDTH: usize = 20;
pub const HEIGHT: usize = 20;

pub struct Maze {
    pub grid: [[bool; WIDTH]; HEIGHT],
}

impl Maze {
    pub fn new() -> Self {
        let mut maze = Maze {
            grid: [[false; WIDTH]; HEIGHT],
        };
        maze.generate();
        maze
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if x == 0 || y == 0 || x == WIDTH - 1 || y == HEIGHT - 1 {
                    self.grid[y][x] = true; // Toujours des murs sur les bords
                } else {
                    self.grid[y][x] = rng.gen_bool(0.3);
                }
            }
        }
        // Assurez-vous que la position de départ du joueur est libre
        self.grid[1][1] = false;
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.grid[y][x]
    }
}