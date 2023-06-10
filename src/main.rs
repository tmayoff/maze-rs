mod cell;
mod grid;

use bevy::prelude::*;
use cell::{Cell, CellBundle, Walls, CELL_COLOR};
use grid::{Generator, Grid};
use rand::seq::SliceRandom;
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(run_build_maze.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(0.1))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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

fn run_build_maze(
    mut grid_query: Query<&mut Grid>,
    mut generator_query: Query<&mut Generator>,
    mut cell_query: Query<(&mut Sprite, &mut Cell)>,
    mut wall_query: Query<&mut Sprite, Without<Cell>>,
    _: Res<FixedTime>,
) {
    let grid = grid_query.single_mut();
    let mut generator = generator_query.single_mut();
    if generator.stack.is_empty() {
        return;
    }

    let e = grid.cells[generator.current_cell.0 as usize][generator.current_cell.1 as usize];
    Cell::unmark_current(e, &mut cell_query);

    let current_cell_entity = generator.stack.pop().unwrap();
    let (_, current_cell) = cell_query.get_mut(current_cell_entity).unwrap();
    let current_cell = current_cell.to_owned();
    Cell::visit_cell(current_cell_entity, &mut cell_query);
    Cell::mark_current(current_cell_entity, &mut cell_query);
    generator.current_cell = current_cell.pos;

    let neighbours = grid.get_cell_neighbour_entities(current_cell.pos.0, current_cell.pos.1);

    let neighbour = neighbours
        .iter()
        .filter_map(|n| n.0)
        .map(|n| (n, cell_query.get_mut(n).unwrap().1.to_owned()))
        .filter(|n| !n.1.visited)
        .collect::<Vec<_>>()
        .choose(&mut rand::thread_rng())
        .map(|s| s.to_owned());

    if let Some((neighbour_entity, neighbour)) = neighbour {
        generator.stack.push(current_cell_entity);
        let wall_dir = current_cell.get_wall_dir(&neighbour);
        let other_wall_dir = neighbour.get_wall_dir(&current_cell);

        Cell::remove_wall(
            wall_dir,
            current_cell_entity,
            &mut cell_query,
            &mut wall_query,
        );
        Cell::remove_wall(
            other_wall_dir,
            neighbour_entity,
            &mut cell_query,
            &mut wall_query,
        );

        Cell::visit_cell(neighbour_entity, &mut cell_query);
        generator.stack.push(neighbour_entity);
    }
}
