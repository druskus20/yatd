use bevy_tweening::{lens::*, *};
use std::time::Duration;

use crate::game_state::GameState;
use bevy::prelude::*;

#[derive(Default)]
pub struct StartMenuPlugin {
    desired_state: Option<GameState>,
}

impl StartMenuPlugin {
    pub fn new() -> Self {
        Self {
            desired_state: None,
        }
    }

    pub fn run_in_state(state: GameState) -> Self {
        Self {
            desired_state: Some(state),
        }
    }
}

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        if let Some(desired_state) = self.desired_state {
            app.add_system_set(SystemSet::on_enter(desired_state).with_system(setup))
                .add_system_set(SystemSet::on_update(desired_state).with_system(button_selection))
                .add_system_set(SystemSet::on_exit(desired_state).with_system(destroy));
        } else {
            panic!("StartMenuPlugin::run_in_state() must be called with a GameState");
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    let container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect::all(Val::Px(0.)),
                margin: Rect::all(Val::Px(16.)),
                padding: Rect::all(Val::Px(16.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .insert(Name::new("menu"))
        .insert(Menu {})
        .id();

    for text in &["Continue", "New Game", "Quit"] {
        commands
            .spawn_bundle(ButtonBundle {
                node: Node {
                    size: Vec2::new(300., 80.),
                },
                style: Style {
                    min_size: Size::new(Val::Px(300.), Val::Px(80.)),
                    margin: Rect::all(Val::Px(8.)),
                    padding: Rect::all(Val::Px(8.)),
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                color: Color::rgb(0.2, 0.2, 0.2).into(),
                transform: Transform::from_scale(Vec3::splat(0.5)),
                ..Default::default()
            })
            .insert(Name::new(format!("button:{}", text)))
            .insert(Parent(container))
            .insert(Animator::new(Tween::new(
                EaseFunction::BounceOut,
                TweeningType::Once,
                Duration::from_millis(250),
                TransformScaleLens {
                    start: Vec3::splat(0.1),
                    end: Vec3::splat(1.0),
                },
            )))
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        text.to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 48.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            });
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    commands.entity(query.single()).despawn_recursive();
}

pub fn button_selection(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, entity) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.3, 0.3, 0.3).into();
                // button_action.run() // TODO
            }
            Interaction::Hovered => {
                // NOTE: We dont need to remove the Animator afterwards
                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::BounceOut,
                    TweeningType::Once,
                    Duration::from_millis(250),
                    TransformScaleLens {
                        start: Vec3::splat(0.8),
                        end: Vec3::splat(1.0),
                    },
                )));
                *color = Color::rgb(0.2, 0.2, 0.2).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.1, 0.1, 0.1).into();
            }
        }
    }
}

#[derive(Component)]
struct Menu {}
