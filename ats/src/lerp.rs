use std::{collections::VecDeque, time::Instant};
use bevy::{ecs::component::Mutable, prelude::*};

// struct LerpPlugin;
// impl Plugin for LerpPlugin{
//     fn build(&self, app: &mut bevy::app::App) {
//         app
//             .add_systems(schedule, systems)
//     }
// }

pub trait Lerpable: Clone{
    fn current_value(start: &Self, stop: &Self, percentage: f32)->Self;
}
#[derive(Component)]
pub struct Lerp<T: Lerpable>{
    //assumed to be sorted by increasing time
    points: VecDeque<LerpPoint<T>> 
}
#[derive(Debug)]
pub struct LerpPoint<T>{
    val: T, time: Instant
}

pub fn lerp_value<T>(
    mut lerp: Query<(&mut T, &Lerp<T>)>
)
where
    T: Lerpable,
    T: Component<Mutability = Mutable>
{
    for(mut val, lerp) in lerp.iter_mut() {
        if let Some(new) = lerp.current_value(Instant::now()) {
            *val = new;
        }
    }
}
impl<T> LerpPoint<T>{
    pub fn new(val: T, time: Instant)->Self{
        Self{val, time}
    }
}
impl<T: Lerpable> Lerp<T>{
    pub fn insert_point_delete_later(&mut self, val: T, time: Instant){
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
    pub fn new_inner(inner: VecDeque<LerpPoint<T>>)->Self{
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