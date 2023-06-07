mod cell;
mod grid;

use bevy::prelude::*;
use cell::{Cell, CellBundle, Walls, CELL_COLOR};
use grid::{Generator, Grid};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(run_build_maze)
        .insert_resource(FixedTime::new_from_secs(0.5))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let grid_size = (10, 10);
    let cell_size = (20.0, 20.0);
    let wall_thickness = 1.0;
    let mut cells = Vec::new();

    for row in 0..grid_size.0 {
        let mut cells_row = Vec::new();

        for col in 0..grid_size.1 {
            let p_pos = (row - (grid_size.0 / 2), col - (grid_size.1 / 2));
            let pos = Vec2::new(
                p_pos.0 as f32 * (cell_size.0 + wall_thickness),
                p_pos.1 as f32 * (cell_size.1 + wall_thickness),
            );

            let id = commands
                .spawn(CellBundle {
                    cell: Cell {
                        walls: Walls::all(),
                        visited: false,
                        pos: (row, col),
                    },
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
    _: Res<FixedTime>,
) {
    let grid = grid_query.single_mut();
    let mut generator = generator_query.single_mut();
    if generator.stack.is_empty() {
        return;
    }

    let current_cell_entity = generator.stack.pop().unwrap();
    let (_, current_cell) = cell_query.get_mut(current_cell_entity).unwrap();
    let current_cell = current_cell.to_owned();
    Cell::visit_cell(current_cell_entity, &mut cell_query);

    let neighbours = grid.get_cell_neighbour_entities(current_cell.pos.0, current_cell.pos.1);

    let neighbour = neighbours
        .iter()
        .filter_map(|n| n.0)
        .map(|n| (n, cell_query.get_mut(n).unwrap().1.to_owned()))
        .filter(|n| !n.1.visited)
        .next();

    if let Some((neighbour_entity, neighbour)) = neighbour {
        generator.stack.push(current_cell_entity);
        let wall_dir = current_cell.get_wall_dir(&neighbour);
        let other_wall_dir = neighbour.get_wall_dir(&current_cell);

        Cell::remove_wall(wall_dir, current_cell_entity, &mut cell_query);
        Cell::remove_wall(other_wall_dir, neighbour_entity, &mut cell_query);

        Cell::visit_cell(neighbour_entity, &mut cell_query);
        generator.stack.push(neighbour_entity);
    }
}
