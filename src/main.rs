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
    machine: HashMap<MachineType, Handle<ColorMaterial>>,
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
) {
    world.tick_timer.tick(time.delta_seconds);
    if world.tick_timer.finished {


        world.tick_timer.reset();
    }
}

impl GridWorld {
    fn place_object(&mut self, object: GridObject) -> bool {
        if let Some(_) = self.get_object_at(object.pos) {
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
}

// impl GridWorld {
//     fn
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MachineType {
    ConveyerBelt,
    Target(u8),
}

struct Machine {
    kind: MachineType,
    pos: Vec2,
    dir: u8,
}


struct MachinePlacementWidget {
    dir: u8,
    entity: Entity,
    selected_machine: MachineType,
}

struct GridObject {
    kind: GridObjectType,
    entity: Entity,
    pos: Vec2,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum GridObjectType {
    Cheese = 0x1,
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(GridWorld::default())
        .add_resource(Sprites::default())
        .add_startup_system(init_scene.system())
        .add_startup_system(place_random_cubes.system())
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
    asset_server.load_asset_folder("assets").unwrap();

    sprites.cursor = Some(color_materials.add(asset_server.get_handle("assets/sprites/grid_cell.png").unwrap().into()));
    sprites.grid_object.insert(GridObjectType::Cheese, color_materials.add(asset_server.get_handle("assets/sprites/cheese.png").unwrap().into()));
    
    sprites.machine.insert(MachineType::ConveyerBelt, color_materials.add(asset_server.get_handle("assets/sprites/conveyor.png").unwrap().into()));
    sprites.machine.insert(MachineType::Target(0), color_materials.add(asset_server.get_handle("assets/sprites/target.png").unwrap().into()));

    let widget = commands
        .spawn(SpriteComponents {
            material: sprites.machine[&MachineType::ConveyerBelt].clone(),
            translation: (Vec3::new(-8., 4., 0.) * CELL_SIZE as f32).into(),
            ..Default::default()
        }).current_entity().unwrap();
    
    commands.insert_resource(MachinePlacementWidget {
        dir: 0x0,
        entity: widget,
        selected_machine: MachineType::ConveyerBelt,
    });

    let camera_entity = commands
        .spawn(Camera2dComponents::default())
        .current_entity().unwrap();

    commands.push_children(camera_entity, &[widget]);
    
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

    let bounds_mesh = meshes.add(Mesh::from(shape::Quad { size: Vec2::new(CELL_SIZE as f32 / 2., (MAX_WORLD_COORD as f32 + 1.) * CELL_SIZE as f32), flip: false }));
    let bounds_mat = standard_materials.add(Color::BLUE.into());
    standard_materials.get_mut(&bounds_mat).unwrap().shaded = false;
        
    let max_pos = (MAX_WORLD_COORD * CELL_SIZE) as f32;

    commands
        .spawn(PbrComponents {
            mesh: bounds_mesh.clone(),
            material: bounds_mat.clone(),
            ..Default::default()
        })
        .with(Translation(Vec3::new(- 24., max_pos / 2., 0.)));

    commands
        .spawn(PbrComponents {
            mesh: bounds_mesh.clone(),
            material: bounds_mat.clone(),
            ..Default::default()
        })
        .with(Translation(Vec3::new(max_pos + 24., max_pos / 2., 0.)));

    commands
        .spawn(PbrComponents {
            mesh: bounds_mesh.clone(),
            material: bounds_mat.clone(),
            ..Default::default()
        })
        .with(Translation(Vec3::new(max_pos / 2., - 24., 0.)))
        .with(Rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)));

    commands
        .spawn(PbrComponents {
            mesh: bounds_mesh.clone(),
            material: bounds_mat.clone(),
            ..Default::default()
        })
        .with(Translation(Vec3::new(max_pos / 2., max_pos + 24., 0.)))
        .with(Rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)));

        
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
    // mut commands: Commands,
    mut machine_widget: ResMut<MachinePlacementWidget>,
    sprites: Res<Sprites>,
    kb: Res<Input<KeyCode>>, 
    // cursor: Res<Cursor>, 
    // mut world: ResMut<GridWorld>,
    query: Query<&mut Handle<ColorMaterial>>,
) {
    let selected = 
             if kb.just_pressed(KeyCode::Key1) { Some(MachineType::ConveyerBelt) }
        else if kb.just_pressed(KeyCode::Key2) { Some(MachineType::Target(0))}
        else { None };

    if let Some(selected) = selected {
        if selected != machine_widget.selected_machine {
            machine_widget.selected_machine = selected;

            println!("Selected {:?}", &selected);
            
            if let Ok(mut sprite) = query.get_mut::<Handle<ColorMaterial>>(machine_widget.entity) {
                *sprite = sprites.machine[&selected];
                println!("Changing selection");
            }
        }
    }


    // if kb.just_pressed(KeyCode::Key1) {
    //     // TODO WT: Check that object can be placed here.
    //     let entity = commands.spawn(SpriteComponents {
    //         material: sprites.grid_object.get(&GridObjectType::Cheese).unwrap().clone(),
    //         translation: Translation::new(cursor.pos.x() * CELL_SIZE as f32, cursor.pos.y() * CELL_SIZE as f32, 1.),
    //         rotation: Quat::from_rotation_z(random::<f32>()).into(),
    //         ..Default::default()
    //     }).current_entity().unwrap();

    //     // TODO WT: Make this so the entity can be spawned after the check for whether there's space.

    //     if !world.place_object(GridObject {
    //         kind: GridObjectType::Cheese,
    //         entity,
    //         pos: cursor.pos,
    //     }) {
    //         commands.despawn(entity);
    //     }
    // }
}