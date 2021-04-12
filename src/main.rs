use bevy::gltf::{Gltf, GltfNode};
use bevy::pbr::AmbientLight;
use bevy::prelude::*;

fn main() {
    // TODO(cretz): Remove
    env_logger::Builder::from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    )
    .init();
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(debug_asset_loaded.system())
        .add_system(rotator_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .run();
}

struct Character(Handle<Gltf>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::info!("in setup");
    // Load the character (for debug only at this point)
    commands.insert_resource(Character(asset_server.load("ybot/ybot-fixed.gltf")));
    // Load character scene, with scale increased and rotated
    let mut init_transform = Transform::from_scale(Vec3::new(30.0, 30.0, 30.0));
    init_transform.rotate(Quat::from_rotation_x(f32::to_radians(-90.0)));
    commands
        .spawn_bundle((
            init_transform,
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("ybot/ybot-fixed.gltf#Scene0"));
        });
    // Add camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    });
    // Add rotating light
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 3.0),
            ..Default::default()
        })
        .insert(Rotates);
}

struct Rotates;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4.0 * std::f32::consts::PI / 20.0) * time.delta_seconds(),
        )) * *transform;
    }
}

fn debug_asset_loaded(
    mut commands: Commands,
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
                log::info!("default scene {:?}", gltf.default_scene);
                commands.spawn_scene(gltf.default_scene.clone().unwrap());
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
