#![allow(unused)]

mod player;
use bevy::prelude::*;
use player::PlayerPlugin;
use wasm_bindgen::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1. / 60.;

// Entity, Component, Resource

// region: Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}
struct WinSize {
    w: f32,
    h: f32,
}
// endregion: Resources

// region: Components
struct Player;
struct PlayerReadyFire(bool);
struct Laser;

struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}
// endregion: Components
#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    app.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 600.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup.system())
        .add_plugin(PlayerPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create the main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser: materials.add(asset_server.load(LASER_SPRITE).into()),
    });

    // position window
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
}
