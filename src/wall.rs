use crate::collision::Collision;
use bevy::prelude::*;

pub struct WallsPlugin;

pub struct Wall;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_walls.system());
    }
}

fn setup_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    let wall_thickness = 20.0;

    let bounds = Vec2::new(400.0, 300.0);

    make_wall(
        &mut commands,
        &mut materials,
        Vec3::new(bounds.x(), 0.0, 0.0),
        Vec2::new(wall_thickness, (2.0 * bounds.y()) + wall_thickness),
    );

    make_wall(
        &mut commands,
        &mut materials,
        Vec3::new(-bounds.x(), 0.0, 0.0),
        Vec2::new(wall_thickness, (2.0 * bounds.y()) + wall_thickness),
    );

    make_wall(
        &mut commands,
        &mut materials,
        Vec3::new(0.0, bounds.y(), 0.0),
        Vec2::new((2.0 * bounds.x()) + wall_thickness, wall_thickness),
    );

    make_wall(
        &mut commands,
        &mut materials,
        Vec3::new(0.0, -bounds.y(), 0.0),
        Vec2::new((2.0 * bounds.x()) + wall_thickness, wall_thickness),
    );
}

fn make_wall(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    translation: Vec3,
    size: Vec2,
) -> Entity {
    let wall_material = materials.add(Color::rgb(0.0, 0.0, 0.5).into());

    commands
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(translation),
            sprite: Sprite { size },
            ..Default::default()
        })
        .with(Collision)
        .with(Wall)
        .current_entity()
        .unwrap()
}
