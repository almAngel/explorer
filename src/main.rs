mod components;
mod plugins {
    pub mod camera;
    pub mod player;
}

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use plugins::{ player::Player, camera::CustomCamera };


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((Player, CustomCamera))
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(50.0, 0., 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0., 0., 0.)));

    commands
        .spawn(Collider::cuboid(2., 1., 2.))
        .insert(RigidBody::Fixed)
        .insert(ColliderDebugColor(Color::BLUE))
        .insert(TransformBundle::from(Transform::from_xyz(0., 1., 0.)));

    commands
        .spawn(Collider::cuboid(2., 2., 2.))
        .insert(RigidBody::Fixed)
        .insert(ColliderDebugColor(Color::BLUE))
        .insert(TransformBundle::from(Transform::from_xyz(-4., 2., 0.)));
    
    commands
        .spawn(Collider::cuboid(2., 3., 2.))
        .insert(RigidBody::Fixed)
        .insert(ColliderDebugColor(Color::BLUE))
        .insert(TransformBundle::from(Transform::from_xyz(-8., 3., 0.)));

}
