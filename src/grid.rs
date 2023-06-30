use std::collections::HashMap;

use bevy::prelude::*;
use rand::prelude::*;

use crate::cell::{Cell, CellBundle, Walls, CELL_COLOR};

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
    pub fn generate(commands: &mut Commands) {
        let grid_size = (20, 20);
        let cell_size = (20.0, 20.0);
        let wall_thickness = 2.0;
        let mut cells = Vec::new();

        for row in 0..grid_size.0 {
            let mut cells_row = Vec::new();

            for col in 0..grid_size.1 {
                let p_pos = (row - (grid_size.0 / 2), col - (grid_size.1 / 2));
                let pos = Vec2::new(
                    p_pos.0 as f32 * (cell_size.0),
                    p_pos.1 as f32 * (cell_size.1),
                );

                let walls = HashMap::from([
                    (
                        Walls::TOP,
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(pos.x, pos.y + (cell_size.0 / 2.0), 1.0),
                                    scale: Vec3 {
                                        x: cell_size.0,
                                        y: wall_thickness,
                                        z: 1.0,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .id(),
                    ),
                    (
                        Walls::BOTTOM,
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(pos.x, pos.y - (cell_size.0 / 2.0), 1.0),
                                    scale: Vec3 {
                                        x: cell_size.0,
                                        y: wall_thickness,
                                        z: 1.0,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .id(),
                    ),
                    (
                        Walls::LEFT,
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(pos.x - (cell_size.0 / 2.0), pos.y, 1.0),
                                    scale: Vec3 {
                                        x: wall_thickness,
                                        y: cell_size.0,
                                        z: 1.0,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .id(),
                    ),
                    (
                        Walls::RIGHT,
                        commands
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: Vec3::new(pos.x + (cell_size.0 / 2.0), pos.y, 1.0),
                                    scale: Vec3 {
                                        x: wall_thickness,
                                        y: cell_size.0,
                                        z: 1.0,
                                    },
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .id(),
                    ),
                ]);

                let id = commands
                    .spawn(CellBundle {
                        cell: Cell::new((row, col), walls),
                        sprite: SpriteBundle {
                            sprite: Sprite {
                                color: CELL_COLOR,
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: pos.extend(0.0),
                                scale: Vec3 {
                                    x: cell_size.0,
                                    y: cell_size.1,
                                    z: 1.0,
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    })
                    .id();
                cells_row.push(id);
            }

            cells.push(cells_row);
        }

        let g = Grid {
            size: grid_size,
            cell_size,
            wall_thickness,
            cells,
        };

        commands.spawn(Generator::new(&g));
        commands.spawn(g);
    }

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
