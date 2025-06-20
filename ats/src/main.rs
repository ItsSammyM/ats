// mod tilemap;
mod mouse_world_position;
mod selection_box;
mod lerp;

use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::core_pipeline::core_2d::Camera2d;
use bevy_ecs_tilemap::TilemapPlugin;
use crate::lerp::{lerp_value, Lerp, LerpPoint, Lerpable};
use crate::mouse_world_position::{MouseWorldPosition, MouseWorldPositionPlugin};
use crate::selection_box::{Selectable, Selected, SelectionBoxPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TilemapPlugin,
            MouseWorldPositionPlugin,
            SelectionBoxPlugin
        ))
        .add_systems(Startup, (
            setup_camera,
            spawn_unit
            // tilemap::startup_tilemap,
        ))
        .add_systems(Update, (
            lerp_value::<Position>,
            position_to_transform,
            command_unit,
        ))
        .run();
}


fn command_unit(
    mouse_pos: Res<MouseWorldPosition>,
    keys: Res<ButtonInput<MouseButton>>,
    mut units: Query<(&Position, &mut Lerp<Position>), With<Selectable>>,
    selected: Res<Selected>
){
    if !keys.just_pressed(MouseButton::Right) {return};
    let Some(mouse_pos) = mouse_pos.get() else {return};

    for entity in selected.entities(){
        let Ok((old_position, mut lerp)) = units.get_mut(*entity) else {continue};

        let new_position = Position(*mouse_pos);
        let distance = new_position.0.distance(old_position.0);


        lerp.insert_point_delete_later(
            old_position.clone(),
            Instant::now()
        );
        lerp.insert_point_delete_later(
            new_position,
            Instant::now() + Duration::from_secs_f32(0.001 * distance)
        );
    }
}

fn setup_camera(
    mut commands: Commands
){
    commands.spawn((
        Camera2d,
    ));
}

fn spawn_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let image = asset_server.load("tiles.png");

    commands.spawn_batch(vec![
        UnitBundle::new(image.clone()),
        UnitBundle::new(image),
    ]);
}
#[derive(Bundle)]
struct UnitBundle{
    sprite: Sprite,
    transform: Transform,
    position: Position,
    lerp_position: Lerp<Position>,
    selectable: Selectable
}
impl UnitBundle{
    fn new(image: Handle<Image>)->Self{
        Self{
            sprite: Sprite{
                image,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            position: Position::default(),
            lerp_position: Lerp::<Position>::new_inner(
                vec![
                    LerpPoint::new(Position(Vec2::new(0.0,0.0)), Instant::now()),
                    LerpPoint::new(
                        Position(Vec2::new(rand::random::<f32>()*500.0, rand::random::<f32>()*500.0)),
                        Instant::now() + Duration::from_secs(1)
                    ),
                ].into(),
            ),
            selectable: Selectable
        }
    }
}


#[derive(Component, Clone, Debug, Default)]
struct Position(Vec2);
impl Lerpable for Position{
    fn current_value(start: &Self, stop: &Self, percentage: f32)->Self {
        Position(
            start.0 * (1.0 - percentage) + stop.0 * percentage
        )
    }
}
fn position_to_transform(
    q: Query<(&mut Transform, &Position)>
){
    for (mut transform, pos) in q{
        transform.translation = pos.0.extend(0.0);
    }
}