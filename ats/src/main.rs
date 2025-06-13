// mod tilemap;
mod mouse_world_position;
mod selection_box;

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use bevy::prelude::*;
use bevy::core_pipeline::core_2d::Camera2d;
use bevy_ecs_tilemap::TilemapPlugin;
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
            lerp_position,
            command_unit,
        ))
        .run();
}


fn command_unit(
    mouse_pos: Res<MouseWorldPosition>,
    keys: Res<ButtonInput<MouseButton>>,
    mut units: Query<&mut Lerp<Position>, With<Selectable>>,
    selected: Res<Selected>
){
    if !keys.just_pressed(MouseButton::Right) {return};
    let Some(mouse_pos) = mouse_pos.get() else {return};

    for entity in selected.entities(){
        let Ok(mut lerp) = units.get_mut(*entity) else {continue};

        lerp.insert_point_delete_later(
            Position(*mouse_pos),
            Instant::now() + Duration::from_secs(10)
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
                    LerpPoint::<Position>::new(Position(Vec2::new(0.0,0.0)), Instant::now()),
                    LerpPoint::<Position>::new(
                        Position(Vec2::new(rand::random::<f32>()*200.0, rand::random::<f32>()*200.0)),
                        Instant::now() + Duration::from_secs(1)
                    ),
                ].into(),
            ),
            selectable: Selectable
        }
    }
}
fn lerp_position(
    lerp: Query<(&mut Transform, &mut Position, &Lerp<Position>)>
){
    for (mut transform, mut pos, lerp) in lerp{
        
        if let Some(val) = lerp.current_value(Instant::now()){
            *pos = val;
            transform.translation = pos.0.extend(0.0);
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

#[derive(Component)]
struct Lerp<T: Lerpable + Clone>{
    //assumed to be sorted by increasing time
    points: VecDeque<LerpPoint<T>> 
}
#[derive(Debug)]
struct LerpPoint<T>{
    val: T, time: Instant
}
impl<T> LerpPoint<T>{
    fn new(val: T, time: Instant)->Self{
        Self{val, time}
    }
}
impl<T: Lerpable + Clone> Lerp<T>{
    fn insert_point_delete_later(&mut self, val: T, time: Instant){
        self.points.retain(|p|p.time < time);
        self.points.push_back(LerpPoint { val, time });
    }
    // fn new_one_point(start_val: T, start_time: Instant)->Self{
    //     Self {
    //         points: vec![
    //             LerpPoint{
    //                 val: start_val,
    //                 time: start_time,
    //             }
    //         ].into()
    //     }
    // }
    fn new_inner(inner: VecDeque<LerpPoint<T>>)->Self{
        Self{points: inner}
    }
    fn current_value(&self, current_time: Instant)->Option<T>{

        if let Some(first) = self.points.front() {
            if current_time <= first.time {
                return None
            }
        }
        if let Some(last) = self.points.back() {
            if current_time >= last.time {
                return Some(last.val.clone());
            }
        }
        for (first, second) in
            self.points.iter().zip(self.points.iter().skip(1))
        {
            if first.time <= current_time && current_time <= second.time{
                let full = second.time - first.time;
                let percentage = if !full.is_zero() {
                    let partial = current_time - first.time;
                    partial.as_secs_f32() / full.as_secs_f32()
                }else{
                    0.0
                }.min(1.0);
                return Some(<T as Lerpable>::current_value(
                    &first.val,
                    &second.val,
                    percentage
                ));
            }
        }
        println!("current_time: {:?}, inner: {:?}", current_time, self.points.iter().map(|p|p.time).collect::<Vec<_>>());
        unreachable!("panic if current_time is, less than first, in between first and last, or after last")
    }
}
trait Lerpable{
    fn current_value(start: &Self, stop: &Self, percentage: f32)->Self;
}