use crate::{game_state::GameState, tower::TowerAssets};
use bevy::prelude::*;
use bevy_mod_picking::*;

// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/446ac84cfcd2c76ae5607cca1c871681af09a0d9/src/lib.rs#L98
#[derive(Default)]
pub struct MapPlugin {
    desired_state: Option<GameState>,
}

impl MapPlugin {
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

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);

        if let Some(desired_state) = self.desired_state {
            app.insert_resource(PickingPluginsState {
                enable_picking: false,
                ..Default::default()
            });
            app.add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new().with_system(pick_block),
            );
            // app.add_system_set(SystemSet::on_update(desired_state).with_system(pick_block));
            app.add_system_set(
                SystemSet::on_enter(desired_state)
                    .with_system(setup)
                    .with_system(enable_picking),
            )
            .add_system_set(
                SystemSet::on_exit(desired_state)
                    .with_system(disable_picking)
                    .with_system(destroy),
            );
        } else {
            panic!("MapPlugin::run_in_state() must be called with a GameState");
        }
    }
}

fn enable_picking(
    mut state: ResMut<PickingPluginsState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut picking_materials: ResMut<
        MeshButtonMaterials<StandardMaterial, StandardMaterialPickingColors>,
    >,
) {
    state.enable_picking = true;
    picking_materials.pressed = materials.add(Color::rgba(1.0, 0.0, 0.0, 1.0).into());
    //    picking_materials.pressed = materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into());
    dbg!(state.enable_picking);
}

fn disable_picking(mut state: ResMut<PickingPluginsState>) {
    state.enable_picking = false;
}

fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_chunk(commands, meshes, materials, 10, 10, 4, 5.0);
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Chunk>>) {
    commands.entity(query.single()).despawn_recursive();
}

fn spawn_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    length: usize,
    width: usize,
    height: usize,
    block_size: f32,
) {
    commands
        .spawn_bundle(ChunkBundle {
            properties: Chunk {
                length,
                width,
                height,
            },
            ..Default::default()
        })
        .with_children(|p| {
            for l in 0..length {
                for w in 0..width {
                    let height = rand::random::<usize>() % (height - 1) + 1;
                    for h in 0..height {
                        let mut block = p.spawn_bundle(BlockBundle {
                            properties: Block {
                                x: l,
                                y: h,
                                z: w,
                                kind: BlockKind::Stone,
                                has_tower: false,
                            },
                            pbr: PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: block_size })),
                                material: materials.add(
                                    Color::rgb(l as f32 / 10.0, h as f32 / 10.0, w as f32 / 10.0)
                                        .into(),
                                ),
                                transform: Transform::from_translation(Vec3::new(
                                    l as f32 * block_size,
                                    h as f32 * block_size,
                                    w as f32 * block_size,
                                )),
                                ..Default::default()
                            },
                        });
                        if h == height - 1 {
                            block.insert_bundle(PickableBundle::default());
                        }
                    }
                }
            }
        });
}

#[derive(Bundle, Default)]
pub struct ChunkBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub properties: Chunk,
}

#[derive(Component, Default)]
pub struct Chunk {
    pub length: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Bundle)]
pub struct BlockBundle {
    pub properties: Block,
    #[bundle]
    pub pbr: PbrBundle,
}

#[derive(Component)]
pub struct Block {
    x: usize,
    y: usize,
    z: usize,
    kind: BlockKind,
    has_tower: bool,
}

pub enum BlockKind {
    Dirt,
    Stone,
}

fn spawn_tower_on_block(
    commands: &mut Commands,
    position: Vec3,
    tower_assets: &ResMut<TowerAssets>,
) {
    super::tower::spawn_tower(commands, position, tower_assets);
}

pub fn pick_block(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tower_assets: ResMut<TowerAssets>,
    mut events: EventReader<PickingEvent>,
    mut query: Query<(
        &mut Transform,
        &mut Block,
        &mut Handle<StandardMaterial>,
        &PickableButton<StandardMaterial>,
    )>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            if let Ok((transform, mut block, mut material, button)) = query.get_mut(*e) {
                if !block.has_tower {
                    //selection.set_selected(false);

                    // TODO: Fix once every block has its own texture
                    *material = button.initial.clone().unwrap();
                    *material = materials.add(Color::rgb(0.0, 0.0, 1.0).into());
                    commands.entity(*e).remove_bundle::<PickableBundle>();
                    spawn_tower_on_block(&mut commands, transform.translation, &tower_assets);
                    block.has_tower = true;
                }
            }
        }
    }
}
