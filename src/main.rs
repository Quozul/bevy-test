mod utils;
mod bullet;

use bevy::{prelude::*, window::PresentMode};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game sample".to_string(),
            width: 800.,
            height: 600.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_on_input)
        .add_system(shoot_bullet)
        .add_system(bullet::move_bullet)
        .run();
}

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct CanShoot;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
        .insert(Movable)
        .insert(CanShoot);
}

fn move_on_input(
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    let dt = time.delta_seconds();
    let speed = if key.pressed(KeyCode::LShift) {
        dt * 200.0
    } else {
        dt * 100.0
    };

    for mut transform in query.iter_mut() {
        if key.pressed(KeyCode::Q) {
            transform.translation.x -= speed;
        } else if key.pressed(KeyCode::D) {
            transform.translation.x += speed;
        }

        if key.pressed(KeyCode::S) {
            transform.translation.y -= speed;
        } else if key.pressed(KeyCode::Z) {
            transform.translation.y += speed;
        }
    }
}

fn shoot_bullet(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut query: Query<&Transform, With<CanShoot>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.pressed(MouseButton::Left) {
        let world_pos = utils::get_world_mouse(windows, q_camera);

        for transform in query.iter_mut() {
            let angle = (world_pos.y - transform.translation.y).atan2(world_pos.x - transform.translation.x);
            let x = angle.cos();
            let y = angle.sin();

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_translation(transform.translation),
                ..default()
            })
                .insert(bullet::Bullet { x, y, life: 2.0 });
        }
    }
}