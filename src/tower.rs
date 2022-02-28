use crate::game_state::GameState;
use bevy::prelude::*;

// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/446ac84cfcd2c76ae5607cca1c871681af09a0d9/src/lib.rs#L98
#[derive(Default)]
pub struct TowerPlugin {
    desired_state: Option<GameState>,
}

impl TowerPlugin {
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

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TowerAssets>();
        if let Some(desired_state) = self.desired_state {
            app //.add_system_set(SystemSet::on_enter(desired_state).with_system(setup))
                .add_system_set(SystemSet::on_update(desired_state).with_system(aim_towers))
                .add_system_set(SystemSet::on_exit(desired_state).with_system(destroy));
        } else {
            panic!("TowerPlugin::run_in_state() must be called with a GameState");
        }
    }
}

#[derive(Default, Clone)]
pub struct TowerAssets {
    pub cannon_mesh: Handle<Mesh>,
    pub body_mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

// TODO: Use GlobalTransform instead
pub fn spawn_tower(commands: &mut Commands, position: Vec3, tower_assets: &ResMut<TowerAssets>) {
    let scale = 1.8;
    let offset = 3.5;
    commands
        .spawn_bundle(TowerBundle {
            properties: Tower::default(),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn_bundle(PbrBundle {
                mesh: tower_assets.cannon_mesh.clone(),
                material: tower_assets.material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, offset + (1.0 * scale), 0.0))
                    .with_scale(Vec3::new(scale, scale, scale)),
                ..Default::default()
            })
            .insert(TowerCannon::default());
            p.spawn_bundle(PbrBundle {
                mesh: tower_assets.body_mesh.clone(),
                material: tower_assets.material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, offset + 1.0, 0.0))
                    .with_scale(Vec3::new(scale, scale, scale)),
                ..Default::default()
            })
            .insert(TowerBody::default());
        });
}

fn aim_towers(time: Res<Time>, mut towers: Query<&mut Transform, With<TowerCannon>>) {
    for mut transform in (&mut towers).iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Tower>>) {
    commands.entity(query.single()).despawn_recursive();
}

#[derive(Bundle, Default)]
pub struct TowerBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub properties: Tower,
}

#[derive(Component, Default)]
pub struct TowerBody {}

#[derive(Component, Default)]
pub struct TowerCannon {}

#[derive(Component, Default)]
pub struct Tower {
    kind: TowerKind,
}

pub enum TowerKind {
    Cannon,
}

impl Default for TowerKind {
    fn default() -> Self {
        TowerKind::Cannon
    }
}
