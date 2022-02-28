use crate::game_state::GameState;
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use leafwing_input_manager::{
    plugin::InputManagerPlugin, prelude::ActionState, InputManagerBundle,
};
use leafwing_input_manager::{prelude::InputMap, Actionlike};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};

// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/446ac84cfcd2c76ae5607cca1c871681af09a0d9/src/lib.rs#L98
#[derive(Default)]
pub struct CameraPlugin {
    desired_state: Option<GameState>,
}

impl CameraPlugin {
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

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        if let Some(desired_state) = self.desired_state {
            let p = InputManagerPlugin::<CameraAction, GameState>::run_in_state(desired_state);
            app.add_plugin(p)
                .add_system_set(SystemSet::on_enter(desired_state).with_system(setup))
                .add_system_set(SystemSet::on_update(desired_state).with_system(camera_controller))
                .add_system_set(SystemSet::on_exit(desired_state).with_system(destroy));
        } else {
            //panic!("CameraPlugin::run_in_state() must be called with a GameState");
            app.add_plugin(InputManagerPlugin::<CameraAction>::default())
                .add_startup_system(setup)
                .add_system(camera_controller);
        }
    }
}

fn setup(mut commands: Commands) {
    let scale = 10.0;
    commands
        .spawn_bundle(CameraBundle {
            input_manager: InputManagerBundle {
                input_map: CameraBundle::default_input_map(),
                ..Default::default()
            },
            camera: Camera {
                speed: 25.0 * scale,
                bounds_x: (-5.0 * scale, 5.0 * scale),
                bounds_y: (10.0 * scale, 30.0 * scale),
                bounds_z: (-5.0 * scale, 5.0 * scale),
            },
            look_transform: LookTransformBundle {
                transform: LookTransform {
                    eye: Vec3::new(-5.0 * scale, 15.0 * scale, 5.0 * scale),
                    target: Vec3::new(0.0 * scale, 0.0 * scale, 0.0 * scale),
                },
                smoother: Smoother::new(0.9),
            },
            perspective_camera: PerspectiveCameraBundle {
                ..Default::default()
            },
        })
        .insert_bundle(PickingCameraBundle::default());
}

fn destroy(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    commands.entity(query.single()).despawn_recursive();
}

pub fn camera_controller(
    time: Res<Time>,
    mut camera: Query<(&mut LookTransform, &Transform, &Camera)>,
    actions: Query<&ActionState<CameraAction>>,
    //input: Res<InputBindings>,
) {
    let (mut camera_transform, scene_transform, camera) =
        if let Some((transform, scene_transform, camera)) = camera.iter_mut().next() {
            (transform, scene_transform, camera)
        } else {
            return;
        };

    let actions = actions.single();
    let delta = time.delta_seconds() as f32;
    for direction in CameraAction::DIRECTIONS {
        if actions.pressed(&direction) {
            let increment = direction.scene_direction(scene_transform) * camera.speed * delta;
            let new_position = camera_transform.eye + increment;

            // Check if the new position is in bounds
            if direction.check_in_bounds(scene_transform, new_position, camera) {
                if direction == CameraAction::ZoomIn || direction == CameraAction::ZoomOut {
                    camera_transform.eye += increment;
                } else {
                    camera_transform.eye += increment;
                    camera_transform.target += increment;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Camera {
    speed: f32,
    bounds_x: (f32, f32),
    bounds_y: (f32, f32),
    bounds_z: (f32, f32),
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            speed: 25.0,
            bounds_x: (-f32::INFINITY, f32::INFINITY),
            bounds_y: (-f32::INFINITY, f32::INFINITY),
            bounds_z: (-f32::INFINITY, f32::INFINITY),
        }
    }
}

#[derive(Bundle)]
pub struct CameraBundle {
    #[bundle]
    look_transform: LookTransformBundle,
    #[bundle]
    perspective_camera: PerspectiveCameraBundle,
    #[bundle]
    input_manager: InputManagerBundle<CameraAction>,
    camera: Camera,
}

impl CameraBundle {
    fn default_input_map() -> InputMap<CameraAction> {
        let mut input_map: InputMap<CameraAction> = InputMap::default();

        input_map
            .insert(CameraAction::MoveUp, KeyCode::W)
            .insert(CameraAction::MoveDown, KeyCode::S)
            .insert(CameraAction::MoveLeft, KeyCode::A)
            .insert(CameraAction::MoveRight, KeyCode::D)
            .insert(CameraAction::ZoomIn, KeyCode::Q)
            .insert(CameraAction::ZoomOut, KeyCode::E);

        input_map
    }
}

#[derive(Actionlike, Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum CameraAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    ZoomIn,
    ZoomOut,
}

impl CameraAction {
    const DIRECTIONS: [Self; 6] = [
        Self::MoveUp,
        Self::MoveDown,
        Self::MoveLeft,
        Self::MoveRight,
        Self::ZoomIn,
        Self::ZoomOut,
    ];

    fn scene_direction(self, scene_transform: &Transform) -> Vec3 {
        match self {
            CameraAction::MoveUp => scene_transform.up(),
            CameraAction::MoveDown => scene_transform.down(),
            CameraAction::MoveLeft => scene_transform.left(),
            CameraAction::MoveRight => scene_transform.right(),
            CameraAction::ZoomIn => scene_transform.forward(),
            CameraAction::ZoomOut => scene_transform.back(),
        }
    }

    fn check_in_bounds(self, scene_transform: &Transform, pos: Vec3, camera: &Camera) -> bool {
        let pos = pos.dot(self.scene_direction(scene_transform));
        match self {
            CameraAction::MoveUp => pos < camera.bounds_z.1,
            CameraAction::MoveDown => -pos > camera.bounds_z.0,
            CameraAction::MoveLeft => pos < camera.bounds_x.1,
            CameraAction::MoveRight => -pos > camera.bounds_x.0,
            CameraAction::ZoomIn => pos < -camera.bounds_y.0,
            CameraAction::ZoomOut => pos < camera.bounds_y.1,
        }
    }
}
