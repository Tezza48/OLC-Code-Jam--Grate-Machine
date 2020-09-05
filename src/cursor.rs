use bevy::prelude::*;
use crate::{ CELL_SIZE, MAX_WORLD_COORD };

pub struct Cursor {
    pub pos: Vec2,
    pub cursor: Entity,
    pub camera: Entity,
}

pub fn update_cursor(kb: Res<Input<KeyCode>>, mut cursor: ResMut<Cursor>, query: Query<&mut Translation>) {
    let mut delta = Vec2::zero();
    if kb.just_pressed(KeyCode::W) { *delta.y_mut() += 1.; }
    if kb.just_pressed(KeyCode::S) { *delta.y_mut() -= 1.; }
    if kb.just_pressed(KeyCode::D) { *delta.x_mut() += 1.; }
    if kb.just_pressed(KeyCode::A) { *delta.x_mut() -= 1.; }

    if kb.pressed(KeyCode::LShift) { delta *= 10.; }

    cursor.pos += delta;

    cursor.pos = cursor.pos.floor().max(Vec2::zero()).min(Vec2::new(MAX_WORLD_COORD as f32, MAX_WORLD_COORD as f32));

    if let Ok(mut translation) = query.get_mut::<Translation>(cursor.cursor) {
        translation.set_x(cursor.pos.x() * CELL_SIZE as f32);
        translation.set_y(cursor.pos.y() * CELL_SIZE as f32);
    }

    if let Ok(mut translation) = query.get_mut::<Translation>(cursor.camera) {
        translation.set_x(cursor.pos.x() * CELL_SIZE as f32);
        translation.set_y(cursor.pos.y() * CELL_SIZE as f32);
    }
}