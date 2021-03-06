use rand::{Rng};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapCell {
    Wall,
    Floor
}

impl MapCell {
    fn to_sym(&self) -> String {
        use MapCell::*;
        match &self {
            Wall => "#".to_string(),
            Floor => " ".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn random<T: Rng + 'static>(rng: &mut T) -> Direction {
        use Direction::*;
        match rng.gen_range(0, 4) {
            0 => Up,
            1 => Right,
            2 => Down,
            _ => Left
        }
    }

    fn normalize_dir(&self) -> Direction {
        use Direction::*;
        match self {
            Down  => Up,
            Left  => Right,
            other => other.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    x: usize,
    y: usize
}

impl Cell {
    fn random<T: Rng + 'static>(dimensions: usize, rng: &mut T) -> Cell {
        let current_row: usize = rng.gen_range(0, dimensions);
        let current_col: usize = rng.gen_range(0, dimensions);

        return Cell { x: current_row, y: current_col}
    }

    fn step(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Up => self.y -= 1,
            Right => self.x += 1,
            Down => self.y += 1,
            Left => self.x -= 1
        };
    }

    fn on_edge(&self, direction: Direction, dimensions: usize) -> bool {
        use Direction::*;
        match direction {
            Up    => self.y == 0,
            Right => self.x == dimensions - 1,
            Down  => self.y == dimensions - 1,
            Left  => self.x == 0
        }
    }
}

pub type Map = Vec<Vec<MapCell>>;

fn create_map(cell: MapCell, dimensions: usize) -> Map {
    let mut map: Map = vec![];

    for i in 0..(dimensions) {
        map.push(vec![]);
        for _ in 0..(dimensions) {
            map[i].push(cell.clone());
        }
    }

    map
}

fn set_map(map: &mut Map, cell: &Cell, state: MapCell) {
    map[cell.x][cell.y] = state;
}

pub fn ascii_render_map(map: &Map) {
    let row_iter = map.iter();

    for row in row_iter {
        let col_iter = row.iter();

        for cell in col_iter {
            print!("{}", cell.to_sym());
        }
        print!("\n");
    }
}

pub fn generate_map<T: Rng + 'static>(rng: &mut T, dimensions: usize, max_tunnels: usize, max_len: usize) -> Map {
    let mut tunnels_count: usize = 0;
    let mut map = create_map(MapCell::Wall, dimensions);

    // init for first iteration
    let mut current_cell: Cell = Cell::random(dimensions, rng);
    let mut last_direction: Option<Direction> = None;
    let mut current_direction: Direction;

    while tunnels_count < max_tunnels {
        // Generate direction
        loop {
            current_direction = Direction::random(rng);

            if Some(current_direction.normalize_dir()) != last_direction.clone().map(|x| x.normalize_dir()) {
                break;
            }
        }

        let random_length = rng.gen_range(0, max_len);
        let mut tunnel_length = 0;

        while tunnel_length < random_length {
            // break the loop if its going out of the map
            if current_cell.on_edge(current_direction.clone(), dimensions) {
                break;
            }

            // Set the value to room
            set_map(&mut map, &current_cell, MapCell::Floor);
            current_cell.step(current_direction.clone());
            tunnel_length += 1;
        }

        if tunnel_length > 0 {
            last_direction = Some(current_direction);
            tunnels_count += 1;
        }
    }

    map
}
