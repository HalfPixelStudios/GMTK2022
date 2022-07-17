use bevy::prelude::*;

use crate::game::{Game, Levels};

#[derive(Debug, Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow;

pub struct Cursor(pub Vec2);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(Cursor(Vec2::ZERO))
            .add_system(cursor_system)
            .add_system(camera_controller);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle {
        orthographic_projection: OrthographicProjection {
            scale: 0.5,
            ..default()
        },
        ..OrthographicCameraBundle::new_2d()
    })
    .insert(MainCamera);
}

// from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn cursor_system(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor: ResMut<Cursor>,
) {
    let (camera, transform) = query.single();

    let win = windows.get_primary().unwrap();

    if let Some(pos) = win.cursor_position() {
        let window_size = Vec2::new(win.width() as f32, win.height() as f32);
        let ndc = (pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor.0 = world_pos.truncate();
    }
}

fn camera_controller(
    mut game: ResMut<Game>,
    mut levels: ResMut<Levels>,
    mut camera_query: Query<(&mut Camera, &mut Transform), (With<MainCamera>)>,
) {
    let (mut camera, mut cam_transform) = camera_query.single_mut();
    let level = levels.levels.get(game.level).unwrap();
    if (level.room_center - cam_transform.translation.y).abs() > 70. {
        cam_transform.translation.y +=
            (level.room_center - cam_transform.translation.y).signum() * 0.8;
    } else {
        cam_transform.translation.y = lerp(cam_transform.translation.y, level.room_center, 0.05);
    }
}

fn lerp(x: f32, y: f32, by: f32) -> f32 {
    x * (1. - by) + y * by
}
