use bevy::prelude::*;
use bevy::platform::collections::HashSet;

use crate::mouse_world_position::MouseWorldPosition;


pub struct SelectionBoxPlugin;
impl Plugin for SelectionBoxPlugin{
    fn build(&self, app: &mut bevy::app::App) {
        app
            .insert_resource(Selected::default())
            .add_systems(Startup, setup_select)
            .add_systems(Update, (
                move_selection_box,
                select_units
            ))
        ;
    }
}

#[derive(Resource, Default)]
pub struct Selected(HashSet<Entity>);
impl Selected{
    pub fn entities(&self)->&HashSet<Entity>{
        &self.0
    }
    // fn contains(&self, entity: &Entity)->bool{
    //     self.0.contains(entity)
    // }
}

#[derive(Component)]
struct SelectBoxMain;

#[derive(Component)]
struct SelectBoxCorner;

#[derive(Component)]
pub struct Selectable;

fn setup_select(
    mut commands: Commands
){
    commands.spawn((
        SelectBoxMain,
        children![
            (
                SelectBoxCorner,
                Transform::from_xyz(0.0,0.0,0.0)
            ),
            (
                SelectBoxCorner,
                Transform::from_xyz(0.0,0.0,0.0)
            )
        ]
    ));
}


fn move_selection_box(
    q_select_box_main: Query<&Children, With<SelectBoxMain>>,
    mut q_select_box_corner: Query<&mut Transform, With<SelectBoxCorner>>,
    keys: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MouseWorldPosition>,
){
    let Some(mouse_pos) = mouse_pos.get() else {return};
    let mouse_pos = mouse_pos.extend(0.0);
    let Ok(children) = q_select_box_main.single() else {return};

    if keys.just_pressed(MouseButton::Left){
        let Some(corner) = children.first() else {return};
        let Ok(mut transform) = q_select_box_corner.get_mut(*corner) else {return};
        transform.translation = mouse_pos;
    }
    if keys.just_released(MouseButton::Left){
        let Some(corner) = children.first() else {return};
        let Ok(mut transform) = q_select_box_corner.get_mut(*corner) else {return};
        transform.translation = mouse_pos;
    }
}
fn select_units(
    q_select_box_main: Query<&Children, With<SelectBoxMain>>,
    q_select_box_corner: Query<&Transform, With<SelectBoxCorner>>,
    q_units: Query<(Entity, &Transform), With<Selectable>>,
    mut selected: ResMut<Selected>

){
    let Ok(children) = q_select_box_main.single() else {return};
    let Some(first_corner) = children.first() else {return};
    let Some(second_corner) = children.last() else {return};
    let Ok(first_corner_transform) = q_select_box_corner.get(*first_corner) else {return};
    let Ok(second_corner_transform) = q_select_box_corner.get(*second_corner) else {return};

    for (entity, unit_transform) in q_units{
        if selected.0.contains(&entity) {continue};
        if
            (
                unit_transform.translation.x < first_corner_transform.translation.x &&
                unit_transform.translation.y < first_corner_transform.translation.y &&
                unit_transform.translation.x > second_corner_transform.translation.x &&
                unit_transform.translation.y > second_corner_transform.translation.y
            ) || (
                unit_transform.translation.x < first_corner_transform.translation.x &&
                unit_transform.translation.y < first_corner_transform.translation.y &&
                unit_transform.translation.x > second_corner_transform.translation.x &&
                unit_transform.translation.y > second_corner_transform.translation.y
            ) || (
                unit_transform.translation.x < first_corner_transform.translation.x &&
                unit_transform.translation.y > first_corner_transform.translation.y &&
                unit_transform.translation.x > second_corner_transform.translation.x &&
                unit_transform.translation.y < second_corner_transform.translation.y
            ) || (
                unit_transform.translation.x > first_corner_transform.translation.x &&
                unit_transform.translation.y < first_corner_transform.translation.y &&
                unit_transform.translation.x < second_corner_transform.translation.x &&
                unit_transform.translation.y > second_corner_transform.translation.y
            )
        {
            selected.0.insert(entity);
        }
    }
}