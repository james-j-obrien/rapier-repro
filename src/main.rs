use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(collisions)
        .add_system(spawn_sensor)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, mut rapier: ResMut<RapierConfiguration>) {
    rapier.gravity = Vec2::ZERO;

    /* Create the rigid body. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        // .insert(Velocity::linear(Vec2::new(-20., 0.)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0., 0.0, 0.0)));
}

fn spawn_sensor(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    /* Create the sensor. */
    if keys.pressed(KeyCode::Space) {
        commands
            .spawn()
            .insert(Sensor)
            .insert(Collider::ball(10.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
    }
}

pub fn collisions(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flags) => {
                println!("Start: {:?}, {:?}", e1, e2);
            }
            CollisionEvent::Stopped(e1, e2, _flags) => {
                println!("Stop: {:?}, {:?}", e1, e2);
            }
        }
    }
}
