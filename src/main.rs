use bevy::gltf::{Gltf, GltfNode};
use bevy::prelude::*;

fn main() {
    // TODO(cretz): Remove
    env_logger::Builder::from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info")).init();
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(debug_asset_loaded.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct Character(Handle<Gltf>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::info!("in setup");
    // Load the character
    commands.insert_resource(Character(asset_server.load("ybot/ybot.gltf")));
}

fn debug_asset_loaded(
    mut ev_asset: EventReader<AssetEvent<Gltf>>,
    gltf_assets: ResMut<Assets<Gltf>>,
    gltf_node_assets: ResMut<Assets<GltfNode>>,
    character: Res<Character>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if *handle == character.0 {
                log::info!("loaded character");
                let gltf = gltf_assets.get(handle).unwrap();
                for (name, value) in &gltf.named_nodes {
                    log::info!(
                        "node {} at {:?}",
                        name,
                        gltf_node_assets.get(value).unwrap().transform,
                    );
                }
            }
        }
    }
}
