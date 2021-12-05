use rand::Rng;

pub mod map;

use map::*;

pub fn generate_map<T: Rng + 'static>(
    rng: &mut T,
    dimensions: usize,
    max_tunnels: usize,
    max_len: usize,
) -> Map {
    let mut tunnels_count: usize = 0;
    let mut map = Map::new(MapCell::Wall, dimensions);

    // init for first iteration
    let mut current_cell: Cell = Cell::random(dimensions, rng);
    let mut last_direction: Option<Direction> = None;
    let mut current_direction: Direction;

    while tunnels_count < max_tunnels {
        // Generate direction
        loop {
            current_direction = Direction::random(rng);

            if Some(current_direction.normalize_dir())
                != last_direction.clone().map(|x| x.normalize_dir())
            {
                break;
            }
        }

        let random_length = rng.gen_range(0..max_len);
        let mut tunnel_length = 0;

        while tunnel_length < random_length {
            // break the loop if its going out of the map
            if current_cell.on_edge(current_direction.clone(), dimensions) {
                break;
            }

            // Set the value to room
            map.set(&current_cell, MapCell::Floor);
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
