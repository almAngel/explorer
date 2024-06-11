use bevy::prelude::*;

use bevy_rapier3d::{
    control::{self, 
        CharacterAutostep, 
        KinematicCharacterController}, 
        dynamics::{AdditionalMassProperties, GravityScale, LockedAxes, RigidBody, Sleeping}, 
        geometry::Collider
};

use crate::components::playable::Playable;

pub struct Player;

const SPEED: f32 = 20.;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands) {

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Playable)
        .insert(GravityScale(6.))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(Sleeping::disabled())
        .insert(SpatialBundle {
            transform: Transform::from_xyz(5., 5., 0.),
            ..default()
        })
        .insert(Collider::cuboid(2., 4., 2.))
        .insert(KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: control::CharacterLength::Absolute(2.5),
                min_width: control::CharacterLength::Absolute(0.5),
                ..default()
            }),
            ..default()
        });

}

fn move_player(time: Res<Time>, keyboard_input: Res<ButtonInput<KeyCode>>, mut q_controller: Query<&mut KinematicCharacterController, With<Playable>>) {

    if let Ok(mut controller) = q_controller.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0., 0., 1.);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(-1., 0., 0.);   
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0., 0., -1.);   
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(1., 0., 0.);
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        controller.translation = Some(direction * SPEED * time.delta_seconds());
    }

}