// Koules.rs is a Rust implementation of Koules Game
// Copyright (C) 2022  Alexey Zakhlestin
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use bevy::prelude::*;
use bevy_mouse_tracking_plugin::MousePosPlugin;
use bevy_mouse_tracking_plugin::MousePosWorld;
use bevy_prototype_lyon::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const BBALL_RADIUS: f32 = 16.0;
const SPEED: f32 = 2.0;

// Resources
struct WinSize {
    width: f32,
    height: f32,
}
// end Resources

// Components
#[derive(Component)]
struct Player {
    pub direction: f32,
}

impl Player {
    pub fn new() -> Self {
        Player { direction: 0.0 }
    }
}
// end Components

pub struct KoulesPlugin;

impl Plugin for KoulesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.02, 0.02, 0.02)))
            .insert_resource(WindowDescriptor {
                title: "Koules".to_string(),
                width: 800.0,
                height: 600.0,
                ..Default::default()
            })
            .insert_resource(WinSize {
                // we'll write here updated values later
                width: 800.0,
                height: 600.0,
            });
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 }) // needed for lyon?
        .add_plugin(KoulesPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePosPlugin::Orthographic)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_startup_stage("setup_game_actors", SystemStage::single(init_ball))
        .add_system(keyboard_movement)
        .add_system(mouse_movement)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn init_ball(mut commands: Commands, win_size: Res<WinSize>) {
    let shape = shapes::Circle {
        radius: BBALL_RADIUS,
        ..shapes::Circle::default()
    };

    let player = Player::new();
    let transform = Transform::default();

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GRAY),
                outline_mode: StrokeMode::new(Color::WHITE, 1.0),
            },
            transform,
        ))
        .insert(player);
}

fn keyboard_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut transform, mut player) = query.single_mut();

    if keyboard_input.pressed(KeyCode::Right) {
        player.direction += 3.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        player.direction -= 3.0;
    }

    let direction = if keyboard_input.pressed(KeyCode::Up) {
        1.0
    } else if keyboard_input.pressed(KeyCode::Down) {
        -1.0
    } else {
        0.0
    };

    let x_movement = player.direction.to_radians().sin() * SPEED * direction;
    let y_movement = player.direction.to_radians().cos() * SPEED * direction;

    transform.translation.x += x_movement;
    transform.translation.y += y_movement;
}

fn mouse_movement(
    mouse_input: Res<Input<MouseButton>>,
    mouse: Res<MousePosWorld>,
    mut query: Query<(&mut Transform, With<Player>)>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        let mut transform = query.single_mut().0;

        let dif_x: f32 = mouse.x - transform.translation.x;
        let dif_y: f32 = mouse.y - transform.translation.y;

        let mut velocity = Vec2::default();
        velocity.x = if dif_x > 0. {
            1.0
        } else if dif_x < 0. {
            -1.0
        } else {
            0.
        };
        velocity.y = if dif_y > 0. {
            1.0
        } else if dif_y < 0. {
            -1.0
        } else {
            0.
        };

        transform.translation.x += velocity.x * SPEED;
        transform.translation.y += velocity.y * SPEED;
    }
}
