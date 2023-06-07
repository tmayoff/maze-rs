use bevy::prelude::*;
use bitflags::bitflags;

pub const CELL_COLOR: Color = Color::WHITE;
pub const VISTITED_CELL_COLOR: Color = Color::BLUE;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Walls : u32 {
        const TOP = 0b0001;
        const RIGHT = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT = 0b1000;
    }
}

#[derive(Component, Clone)]
pub struct Cell {
    pub walls: Walls,
    pub visited: bool,
    pub pos: (i32, i32),
}

impl Cell {
    pub fn get_wall_dir(&self, other: &Cell) -> Walls {
        let w: Walls;
        if self.pos.0 < other.pos.0 {
            w = Walls::LEFT;
        } else if self.pos.0 > other.pos.0 {
            w = Walls::RIGHT;
        } else if self.pos.1 > other.pos.1 {
            w = Walls::TOP;
        } else {
            w = Walls::BOTTOM;
        }

        w
    }

    pub fn visit_cell(cell_entity: Entity, query: &mut Query<(&mut Sprite, &mut Cell)>) {
        let (mut sprite, mut cell) = query.get_mut(cell_entity).unwrap();
        cell.visited = true;
        sprite.color = VISTITED_CELL_COLOR;
    }

    pub fn remove_wall(
        wall: Walls,
        cell_entity: Entity,
        query: &mut Query<(&mut Sprite, &mut Cell)>,
    ) {
        let (_, mut cell) = query.get_mut(cell_entity).unwrap();
        cell.walls &= !wall;
    }
}

#[derive(Bundle, Clone)]
pub struct CellBundle {
    pub cell: Cell,
    pub sprite: SpriteBundle,
}
