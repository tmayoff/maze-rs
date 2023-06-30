mod cell;
mod grid;
mod ui;

use bevy::prelude::*;
use cell::Cell;
use grid::{Generator, Grid};
use rand::seq::SliceRandom;
use ui::UI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(run_build_maze.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(0.01))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    UI::generate(&mut commands);
    Grid::generate(&mut commands);
}

fn run_build_maze(
    mut grid_query: Query<&mut Grid>,
    mut generator_query: Query<&mut Generator>,
    mut cell_query: Query<(&mut Sprite, &mut Cell)>,
    mut wall_query: Query<&mut Sprite, Without<Cell>>,
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
