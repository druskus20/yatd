use bevy::prelude::{App, Plugin};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::StartMenu);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    StartMenu,
    Defense,
    End,
}
