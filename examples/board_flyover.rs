//! A simple example of how to create a `bevy_skybox` and attach it to a camera.
//!
//! An optional positional argument can be used to give the filename of the
//! skybox image to test (in the `assets` folder, including the suffix).
//!
//! ```
//! cargo +nightly run --release --example board_flyover -- sky2.png
//! ```
//!
//! The controls are:
//! - W / A / S / D - Move along the horizontal plane
//! - Shift - Move downward
//! - Space - Move upward
//! - Mouse - Look around

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_skybox::{SkyboxCamera, SkyboxPlugin};
use rand::Rng;
use std::env;

/// Create the window, add the plugins and set up the entities.
fn main() {
    // Get the skybox image.
    let image = env::args().nth(1).unwrap_or("sky1.png".to_owned());
    // Build the window and app.
    App::build()
        .add_resource(WindowDescriptor {
            title: "Skybox Board Flyover".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(SkyboxPlugin::from_image_file(&image))
        .add_startup_system(setup.system())
        .run();
}

/// Set up the camera, skybox and "board" in this example.
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a camera with a the `FlyCamera` controls and a `Skybox` centred on it.
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_translation(Vec3::new(0.0, 20.0, -40.0)))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(FlyCamera::default());
        .with(SkyboxCamera);

    // Add a light source.
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
        ..Default::default()
    });

    // Add the "board" as some foreground.
    let mut rng = rand::thread_rng();
    for i in -20..=20 {
        for j in -20..=20 {
            let br = rng.gen::<f32>() * 0.4 + 0.6;
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
                material: materials.add(Color::rgb(0.6 * br, 1. * br, 0.6 * br).into()),
                transform: Transform::from_translation(Vec3::new(
                    i as f32 * 10.0,
                    0.0,
                    j as f32 * 10.0,
                )),
                ..Default::default()
            });
        }
    }
}
