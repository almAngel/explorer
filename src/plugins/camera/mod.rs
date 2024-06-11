use bevy::prelude::*;

use crate::components::{playable::Playable, singleton::Singleton};

pub struct CustomCamera;

impl Plugin for CustomCamera {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {

    commands.spawn(Camera3dBundle {
        ..default()
    })
    .insert(Singleton);

    /* PROFILE VIEW CAMERA */

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0., 0., -100.).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });

}

fn follow_player(playable: Query<&GlobalTransform, With<Playable>>, mut camera: Query<&mut Transform, With<Singleton>>) {

    if let Ok(mut c) = camera.get_single_mut() {
        let p = playable.single();

        c.translation = Transform::from_xyz(p.translation().x, 40., p.translation().z - 40.).translation;
        c.look_at(p.translation(), Vec3::Y);
    }

}