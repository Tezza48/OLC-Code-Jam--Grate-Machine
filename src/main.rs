mod cursor;
use cursor::*;

use std::collections::HashMap;
use bevy::{
    prelude::*,
    input::mouse::{MouseWheel, MouseMotion},
    render::camera::{OrthographicProjection, Camera}, math::Mat2,
};
use rand::prelude::*;

pub const MAX_WORLD_COORD: u32 = 63;
pub const CELL_SIZE: u32 = 32;
const NUM_SCATTERED_QUADS: u32 = 256;

#[derive(Default)]
struct Sprites {
    cursor: Option<Handle<ColorMaterial>>,
    grid_object: HashMap<GridObjectType, Handle<ColorMaterial>>,
}

struct GridWorld {
    tick_timer: Timer,
    machines: Vec<Machine>,
    objects: Vec<GridObject>,
}

impl Default for GridWorld {
    fn default() -> Self {
        Self {
            tick_timer: Timer::from_seconds(1.0, true),
            machines: Default::default(),
            objects: Default::default(),
        }
    }
}

fn tick_world(
    time: Res<Time>,
    mut world: ResMut<GridWorld>,
    query: Query<&mut Translation>,
) {
    world.tick_timer.tick(time.delta_seconds);
    if world.tick_timer.finished {
        // Tick the world.

        let mut to_move = Vec::new();

        // Check which can be moved
        for i in 0..world.objects.len() {
            let object = &world.objects[i];
            let find = world.get_object_at(object.pos - Vec2::new(0., 1.));
            match find {
                Some(obj) => {
                    if *obj != *object {
                        continue;
                    }
                }
            }
        }

        // Move the ones which can be moved.
        for i in 0..to_move.len() {
            let mut object = &mut world.objects[i];

            object.pos -= Vec2::new(0., 1.);
            object.pos = object.pos.max(Vec2::zero()).min(Vec2::new(MAX_WORLD_COORD as f32, MAX_WORLD_COORD as f32));

            if let Ok(mut translation) = query.get_mut::<Translation>(object.entity) {
                translation.set_x(object.pos.x() * CELL_SIZE as f32);
                translation.set_y(object.pos.y() * CELL_SIZE as f32);
            }
        }

        world.tick_timer.reset();
    }
}

impl GridWorld {
    fn place_object(&mut self, object: GridObject) -> bool {
        if !self.is_location_empty(object.pos) {
            return false;
        }
        
        self.objects.push(object);

        true
    }

    fn get_object_at(&self, location: Vec2) -> Option<&GridObject> {
        for obj in self.objects.iter() {
            if obj.pos.x() == location.x() && obj.pos.y() == location.y() {
                return Some(&obj);
            }
        }

        return None
    }

    fn is_location_empty(&self, location: Vec2) -> bool {
        if  location.x() < 0. ||
            location.x() > MAX_WORLD_COORD as f32 ||
            location.y() < 0. ||
            location.y() > MAX_WORLD_COORD as f32
        {
            return false;
        }

        for obj in self.objects.iter() {
            if obj.pos.x() == location.x() && obj.pos.y() == location.y() {
                return false;
            }
        }

        true
    }
}

// impl GridWorld {
//     fn
// }

enum MachineType {
    ConveyerBelt,
    Target(u8)
}

struct Machine {
    kind: MachineType,
    pos: Vec2,
    dir: u8,
}


struct GridObject {
    kind: GridObjectType,
    entity: Entity,
    pos: Vec2,
}

#[derive(PartialEq, Eq, Hash)]
enum GridObjectType {
    Cheese = 0x1,
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(GridWorld::default())
        .add_resource(Sprites::default())
        .add_startup_system(init_scene.system())
        // .add_startup_system(place_random_cubes.system())
        .add_system(update_cursor.system())
        .add_system(debug_place_item.system())
        .add_system(tick_world.system())
        .run();
}

fn init_scene(
    mut commands: Commands,
    mut sprites: ResMut<Sprites>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    asset_server.load_asset_folder("assets");

    sprites.cursor = Some(color_materials.add(asset_server.get_handle("assets/sprites/grid_cell.png").unwrap().into()));
    sprites.grid_object.insert(GridObjectType::Cheese, color_materials.add(asset_server.get_handle("assets/sprites/cheese.png").unwrap().into()));

    let camera_entity = commands
        .spawn(Camera2dComponents::default())
        .current_entity().unwrap();
    
    let cursor_entity = commands
        .spawn(SpriteComponents {
            material: sprites.cursor.unwrap().clone(),
            ..Default::default()
        })
        .with(Translation::default())
        .current_entity().unwrap();

    commands
        .insert_resource(Cursor {
            pos: Vec2::default(),
            camera: camera_entity,
            cursor: cursor_entity,
        });

    // let bounds_mesh = meshes.add(Mesh::from(shape::Quad { size: Vec2::new(CELL_SIZE / 2., (MAX_WORLD_COORD + 1.) * CELL_SIZE), flip: false }));
    // let bounds_mat = standard_materials.add(Color::BLUE.into());
    // standard_materials.get_mut(&bounds_mat).unwrap().shaded = false;
        
    // commands
    //     .spawn(PbrComponents {
    //         mesh: bounds_mesh.clone(),
    //         material: bounds_mat.clone(),
    //         ..Default::default()
    //     })
    //     .with(Translation(Vec3::new(- 24., MAX_WORLD_COORD * CELL_SIZE / 2., 0.)));

    // commands
    //     .spawn(PbrComponents {
    //         mesh: bounds_mesh.clone(),
    //         material: bounds_mat.clone(),
    //         ..Default::default()
    //     })
    //     .with(Translation(Vec3::new(MAX_WORLD_COORD * CELL_SIZE + 24., MAX_WORLD_COORD * CELL_SIZE / 2., 0.)));

    // commands
    //     .spawn(PbrComponents {
    //         mesh: bounds_mesh.clone(),
    //         material: bounds_mat.clone(),
    //         ..Default::default()
    //     })
    //     .with(Translation(Vec3::new(MAX_WORLD_COORD * CELL_SIZE / 2., - 24., 0.)))
    //     .with(Rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)));

    // commands
    //     .spawn(PbrComponents {
    //         mesh: bounds_mesh.clone(),
    //         material: bounds_mat.clone(),
    //         ..Default::default()
    //     })
    //     .with(Translation(Vec3::new(MAX_WORLD_COORD * CELL_SIZE / 2., MAX_WORLD_COORD * CELL_SIZE + 24., 0.)))
    //     .with(Rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)));

        
    // Grid background

    // Conveyer belt sprite

    // click and drag the map

    // scroll zoom


}

fn place_random_cubes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mesh_handle = meshes.add(shape::Quad {
        size: Vec2::new(8., 8.),
        flip: false,
    }.into());

    let mat = materials.add(Color::GREEN.into());

    for _ in (0..NUM_SCATTERED_QUADS).enumerate() {
        commands
            .spawn(PbrComponents {
                mesh: mesh_handle.clone(),
                material: mat.clone(),
                ..Default::default()
            })
            .with(
                Translation::new(
                    rand::random::<f32>() * (CELL_SIZE * MAX_WORLD_COORD) as f32,
                    rand::random::<f32>() * (CELL_SIZE * MAX_WORLD_COORD) as f32,
                    10.0
                )
            );
    }
}

// TODO WT: Make these event triggered, probably a better idea
fn debug_place_item(
    mut commands: Commands,
    sprites: Res<Sprites>,
    kb: Res<Input<KeyCode>>, 
    cursor: Res<Cursor>, 
    mut world: ResMut<GridWorld>
) {
    if kb.just_pressed(KeyCode::Key1) {
        // TODO WT: Check that object can be placed here.
        let entity = commands.spawn(SpriteComponents {
            material: sprites.grid_object.get(&GridObjectType::Cheese).unwrap().clone(),
            translation: Translation::new(cursor.pos.x() * CELL_SIZE as f32, cursor.pos.y() * CELL_SIZE as f32, 1.),
            rotation: Quat::from_rotation_z(random::<f32>()).into(),
            ..Default::default()
        }).current_entity().unwrap();

        // TODO WT: Make this so the entity can be spawned after the check for whether there's space.

        if !world.place_object(GridObject {
            kind: GridObjectType::Cheese,
            entity,
            pos: cursor.pos,
        }) {
            commands.despawn(entity);
        }
    }
}