use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapCell {
    Wall,
    Floor,
}

impl MapCell {
    pub fn to_char(&self) -> char {
        use MapCell::*;
        match &self {
            Wall => '#',
            Floor => ' ',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn random<T: Rng + 'static>(rng: &mut T) -> Direction {
        use Direction::*;
        match rng.gen_range(0..4) {
            0 => Up,
            1 => Right,
            2 => Down,
            _ => Left,
        }
    }

    pub fn normalize_dir(&self) -> Direction {
        use Direction::*;
        match self {
            Down => Up,
            Left => Right,
            other => other.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn random<T: Rng + 'static>(dimensions: usize, rng: &mut T) -> Cell {
        let current_row: usize = rng.gen_range(0..dimensions);
        let current_col: usize = rng.gen_range(0..dimensions);

        return Cell {
            x: current_row,
            y: current_col,
        };
    }

    pub fn step(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Up => self.y -= 1,
            Right => self.x += 1,
            Down => self.y += 1,
            Left => self.x -= 1,
        };
    }

    pub fn on_edge(&self, direction: Direction, dimensions: usize) -> bool {
        use Direction::*;
        match direction {
            Up => self.y == 0,
            Right => self.x == dimensions - 1,
            Down => self.y == dimensions - 1,
            Left => self.x == 0,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    vec: Vec<Vec<MapCell>>,
}

impl Map {
    pub fn new(cell: MapCell, dimensions: usize) -> Map {
        let mut vec = vec![];

        for i in 0..(dimensions) {
            vec.push(vec![]);
            for _ in 0..(dimensions) {
                vec[i].push(cell.clone());
            }
        }

        Map { vec }
    }

    pub fn set(&mut self, cell: &Cell, state: MapCell) {
        self.vec[cell.x][cell.y] = state;
    }

    pub fn iter(&self) -> std::slice::Iter<Vec<MapCell>> {
        self.vec.iter()
    }

    pub fn ascii_render(&self) -> String {
        let row_iter = self.iter();
        let mut string = String::new();

        for row in row_iter {
            let col_iter = row.iter();

            for cell in col_iter {
                string.push(cell.to_char());
            }
            string.push('\n');
        }

        string
    }
}
