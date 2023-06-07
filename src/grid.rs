use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Generator {
    pub current_cell: (i32, i32),
    pub stack: Vec<Entity>,
}

impl Generator {
    pub fn new(grid: &Grid) -> Self {
        let mut rng = rand::thread_rng();
        let start_cell = (
            rand::distributions::Uniform::new(0, grid.size.0).sample(&mut rng),
            rand::distributions::Uniform::new(0, grid.size.1).sample(&mut rng),
        );

        let cell_entity = grid.get_cell_entity(start_cell.0, start_cell.1);
        Generator {
            current_cell: start_cell,
            stack: vec![cell_entity.unwrap()],
        }
    }
}

#[derive(Component, Clone)]
pub struct Grid {
    pub size: (i32, i32),
    pub cell_size: (f32, f32),
    pub wall_thickness: f32,
    pub cells: Vec<Vec<Entity>>,
}

impl Grid {
    pub fn get_cell_neighbour_entities(&self, x: i32, y: i32) -> [(Option<Entity>, (i32, i32)); 4] {
        [
            (self.get_cell_entity(x - 1, y), (x - 1, y)),
            (self.get_cell_entity(x, y - 1), (x, y - 1)),
            (self.get_cell_entity(x + 1, y), (x + 1, y)),
            (self.get_cell_entity(x, y + 1), (x, y + 1)),
        ]
    }

    pub fn get_cell_entity(&self, x: i32, y: i32) -> Option<Entity> {
        if x < 0 || x > self.size.0 - 1 || y < 0 || y > self.size.1 - 1 {
            return None;
        }

        Some(self.cells.as_slice()[x as usize][y as usize])
    }
}
