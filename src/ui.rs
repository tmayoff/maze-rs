use bevy::prelude::*;

pub struct UI;

impl UI {
    pub fn generate(commands: &mut Commands) {
        commands.spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        });
    }
}
