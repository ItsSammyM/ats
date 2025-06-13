use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct MouseWorldPositionPlugin;
impl Plugin for MouseWorldPositionPlugin{
    fn build(&self, app: &mut bevy::app::App) {
        app
            .insert_resource(MouseWorldPosition::default())
            .add_systems(Update, set_mouse_world_position)
        ;
    }
}

#[derive(Resource, Default)]
pub struct MouseWorldPosition(Option<Vec2>);
impl MouseWorldPosition{
    pub fn get(&self)->&Option<Vec2>{
        &self.0
    }
}
fn set_mouse_world_position(
    mut mouse_pos: ResMut<MouseWorldPosition>,
    camera: Query<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
){
    *mouse_pos = MouseWorldPosition((move ||{
        let window = q_windows.single().ok()?;
        let cursor_pos = window.cursor_position()?;
        let (camera, transform) = camera.single().ok()?;
        let pos = camera.viewport_to_world_2d(
            transform,
            cursor_pos
        ).ok()?;
        Some(pos)
    })());
}