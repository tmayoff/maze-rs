use bevy::prelude::*;
use bitflags::bitflags;
use std::collections::HashMap;

pub const CELL_COLOR: Color = Color::WHITE;
pub const VISIITED_CELL_COLOR: Color = Color::Rgba {
    red: 0.9,
    green: 0.9,
    blue: 0.9,
    alpha: 1.0,
};

bitflags! {
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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
    pub wall_sprites: HashMap<Walls, Entity>,
    pub visited: bool,
    pub pos: (i32, i32),
}

impl Cell {
    pub fn new(pos: (i32, i32), walls: HashMap<Walls, Entity>) -> Self {
        Cell {
            walls: Walls::all(),
            wall_sprites: walls,
            visited: false,
            pos,
        }
    }

    pub fn get_wall_dir(&self, other: &Cell) -> Walls {
        let dx = self.pos.0 - other.pos.0;
        let dy = self.pos.1 - other.pos.1;

        if dx < 0 {
            Walls::RIGHT
        } else if dx > 0 {
            Walls::LEFT
        } else if dy < 0 {
            Walls::TOP
        } else if dy > 0 {
            Walls::BOTTOM
        } else {
            unreachable!("Unknown wall position")
        }
    }

    pub fn visit_cell(cell_entity: Entity, query: &mut Query<(&mut Sprite, &mut Cell)>) {
        let (mut sprite, mut cell) = query.get_mut(cell_entity).unwrap();
        cell.visited = true;
        sprite.color = VISIITED_CELL_COLOR;
    }

    pub fn mark_current(cell_entity: Entity, query: &mut Query<(&mut Sprite, &mut Cell)>) {
        let (mut sprite, _) = query.get_mut(cell_entity).unwrap();
        sprite.color = Color::BLUE;
    }

    pub fn unmark_current(cell_entity: Entity, query: &mut Query<(&mut Sprite, &mut Cell)>) {
        let (mut sprite, _) = query.get_mut(cell_entity).unwrap();
        sprite.color = VISIITED_CELL_COLOR;
    }

    pub fn remove_wall(
        wall: Walls,
        cell_entity: Entity,
        query: &mut Query<(&mut Sprite, &mut Cell)>,
        wall_query: &mut Query<&mut Sprite, Without<Cell>>,
    ) {
        let (_, mut cell) = query.get_mut(cell_entity).unwrap();
        cell.walls &= !wall;
        wall_query
            .get_mut(*cell.wall_sprites.get(&wall).unwrap())
            .unwrap()
            .color = Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 0.0,
        };
    }
}

#[derive(Bundle, Clone)]
pub struct CellBundle {
    pub cell: Cell,
    pub sprite: SpriteBundle,
}

mod tests {

    #[test]
    fn test_wall_dir() {
        use super::*;

        let tests = vec![
            (
                Cell::new((0, 0), HashMap::new()),
                Cell::new((1, 0), HashMap::new()),
                Walls::RIGHT,
                Walls::LEFT,
            ),
            (
                Cell::new((1, 0), HashMap::new()),
                Cell::new((0, 0), HashMap::new()),
                Walls::LEFT,
                Walls::RIGHT,
            ),
            (
                Cell::new((0, 0), HashMap::new()),
                Cell::new((0, 1), HashMap::new()),
                Walls::TOP,
                Walls::BOTTOM,
            ),
            (
                Cell::new((0, 1), HashMap::new()),
                Cell::new((0, 0), HashMap::new()),
                Walls::BOTTOM,
                Walls::TOP,
            ),
        ];

        for t in tests {
            let c1 = t.0;
            let c2 = t.1;

            let wall1 = c1.get_wall_dir(&c2);
            let wall2 = c2.get_wall_dir(&c1);
            assert_eq!(wall1, t.2);
            assert_eq!(wall2, t.3);
        }
    }
}
