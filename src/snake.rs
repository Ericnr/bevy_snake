use crate::collision::Collision;
use crate::wall::Wall;
use bevy::{prelude::*, sprite::collide_aabb::collide};
struct Snake {
    velocity: Vec3,
}

pub struct SnakePlugin;

fn rotate_velocity((x, y): (f32, f32), angle: f32) -> (f32, f32) {
    let (sin, cos) = angle.to_radians().sin_cos();
    ((x * cos) - (y * sin), (x * sin) + (y * cos))
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup_snake.system());
        app.add_system(snake_movement.system());
        app.add_system(snake_controls.system());
        app.add_system(snake_collision.system());
    }
}

fn startup_snake(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    spawn_snake(&mut commands, &mut materials, Vec3::new(0.0, 0.0, 1.0));
}

fn snake_controls(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut snake: Mut<Snake>) {
    let velocity = &mut snake.velocity;

    let mut angle = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        angle += 240.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        angle -= 240.0;
    }
    let x = velocity.x();
    let y = velocity.y();

    let (new_x, new_y) = rotate_velocity((x, y), time.delta_seconds * angle);

    *velocity.x_mut() = new_x;
    *velocity.y_mut() = new_y;
}

fn snake_collision(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_query: Query<(Entity, &Translation, Mut<Snake>)>,
    mut wall_query: Query<(&Translation, &Sprite, &Wall)>,
) {
    for (snake_entity, snake_translation, _) in &mut snake_query.iter() {
        for (wall_translation, sprite, _) in &mut wall_query.iter() {
            if let Some(_) = collide(
                snake_translation.0,
                Vec2::new(12.0, 12.0),
                wall_translation.0,
                sprite.size,
            ) {
                commands.despawn(snake_entity);
                spawn_snake(&mut commands, &mut materials, Vec3::new(0.0, 0.0, 1.0));
            }
        }
    }
}

fn snake_movement(time: Res<Time>, snake: &Snake, mut translation: Mut<Translation>) {
    translation.0 += snake.velocity * time.delta_seconds;
}

fn spawn_snake(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    translation: Vec3,
) -> Entity {
    let wall_material = materials.add(Color::rgb(0.5, 0.5, 0.0).into());

    commands
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(translation),
            sprite: Sprite {
                size: Vec2::new(12.0, 12.0),
            },
            ..Default::default()
        })
        .with(Collision)
        .with(Snake {
            velocity: Vec3::new(40.0, 1.0, 0.0),
        })
        .current_entity()
        .unwrap()
}
