use bevy::prelude::*;

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut tower_assets: ResMut<super::tower::TowerAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>, // TODO: Remove
) {
    tower_assets.cannon_mesh = asset_server.load("models/basic_tower.glb#Mesh0/Primitive0");
    tower_assets.body_mesh = asset_server.load("models/basic_tower.glb#Mesh1/Primitive0");
    tower_assets.material = materials.add(Color::rgb(0.1, 0.2, 0.2).into());
}
