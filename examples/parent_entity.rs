use bevy::prelude::*;
use bevy_tiled_prototype::{MapRoot, TiledMapCenter};

// this example demonstrates moving the map mesh entities using
// the MapRoot marker on a passed-in parent element

const SCALE: f32 = 0.25;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tiled_prototype::TiledMapPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(move_parent_entity.system())
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let's pass in a parent to append map tiles to, otherwise one will be created
    let parent = commands
        .spawn((
            Transform {
                ..Default::default()
            },
            GlobalTransform {
                ..Default::default()
            },
            Visible {
                ..Default::default()
            },
        ))
        .current_entity();

    commands
        .spawn(bevy_tiled_prototype::TiledMapBundle {
            map_asset: asset_server.load("ortho-map.tmx"),
            parent_option: parent,
            center: TiledMapCenter(true),
            origin: Transform::from_scale(Vec3::new(4.0, 4.0, 1.0)),
            ..Default::default()
        })
        .spawn(OrthographicCameraBundle::new_2d());
}

fn move_parent_entity(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&MapRoot, &mut Visible, &mut Transform)>,
) {
    for (_, mut _visible, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(SCALE, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(SCALE, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, SCALE, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, SCALE, 0.0);
        }

        // this won't work right now :'-( -- https://github.com/bevyengine/bevy/issues/838
        // if keyboard_input.just_released(KeyCode::Space) {
        //     visible.is_visible = !visible.is_visible;
        // }

        transform.translation += time.delta_seconds() * direction * 1000.;
        transform.scale = Vec3::splat(SCALE);
    }
}
