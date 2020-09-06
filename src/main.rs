mod cursor;
mod gameplay;

use std::collections::HashMap;
use bevy::{
    prelude::*,
    // input::mouse::{MouseWheel, MouseMotion},
    // render::camera::{OrthographicProjection, Camera}, math::Mat2,
};
// use rand::prelude::*;
use gameplay::*;
use cursor::*;

const LERP_SPEED: f32 = 5.0;

#[derive(Default)]
struct Sprites {
    cursor: Handle<ColorMaterial>,
    grid_object: HashMap<GridObjectType, Handle<ColorMaterial>>,
    machine: HashMap<MachineType, Handle<ColorMaterial>>,
    delete: Handle<ColorMaterial>,
}

fn tick_world(
    mut commands: Commands,
    time: Res<Time>,
    sprites: Res<Sprites>,
    mut world: ResMut<GridWorld>,
    query: Query<(&mut Handle<ColorMaterial>, &mut Rotation)>,
    lerp_query: Query<&mut Translation>,
) {
    world.tick_timer.tick(time.delta_seconds);

    let mut changes = Vec::new();
    let mut transmutes = Vec::new();
    let mut removals = Vec::new();

    if world.tick_timer.finished {
        // machines which create stuff

        // machines which move/change objects
        for (i, object) in world.objects.iter().enumerate() {
            if let Some(machine) = world.get_machine_at(object.pos) {
                let mut do_move = false;
                match machine.kind {
                    MachineType::ConveyerBelt => {

                        // object.pos += delta;
                        // object.pos = object.pos.max(Vec2::zero()).min(Vec2::splat(MAX_WORLD_COORD as f32));
                        do_move = true;
                    }
                    MachineType::Target => {
                        removals.push(object.pos);
                        do_move = false;
                    }
                    MachineType::Cow => {
                        do_move = false;
                    }
                    MachineType::Milker => {

                    }
                    MachineType::Grater => {
                        if object.kind == GridObjectType::Cheese {
                            transmutes.push((object.pos, GridObjectType::GratedCheese));
                        }
                        do_move = true;
                    }
                }
                
                if do_move {
                    let mut delta = if machine.dir == 0 { Vec2::new(1., 0.) } 
                    else if machine.dir == 1 { Vec2::new(0., 1.) }
                    else if machine.dir == 2 { Vec2::new(-1., 0.) }
                    else { Vec2::new(0., -1.) };
    
                    changes.push((i, delta));
                }
            }
        }

        for transmute in transmutes.iter_mut() {
            if let Some(object) = world.get_object_at_mut(transmute.0) {
                object.kind = transmute.1;
                if let Ok(mut sprite) = query.get_mut::<Handle<ColorMaterial>>(object.entity) {
                    *sprite = sprites.grid_object[&GridObjectType::GratedCheese];
                }
            }
        }

        for removal in removals.iter() {
            world.remove_object(*removal, &mut commands);
        }

        for change in changes.iter() {
            if let Some(_) = world.get_object_at(world.objects[change.0].pos + change.1) {
                continue;
            }
            world.objects[change.0].pos += change.1;
            world.objects[change.0].pos = world.objects[change.0].pos.max(Vec2::zero()).min(Vec2::splat(MAX_WORLD_COORD as f32));
        }

        world.tick_timer.reset();
    }

    for object in world.objects.iter_mut() {
        if let Ok(mut translation) = lerp_query.get_mut::<Translation>(object.entity) {
            let target = object.pos * CELL_SIZE as f32;
            translation.0 = Vec3::lerp(translation.0, Vec3::new(target.x(), target.y(), translation.0.z()), time.delta_seconds * LERP_SPEED);
            // translation.0 = Vec3::new(target.x(), target.y(), translation.0.z());
        }
    }
}

// impl GridWorld {
//     fn
// }

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(GridWorld::default())
        .add_startup_system(init_scene.system())
        .add_system(update_cursor.system())
        .add_system(debug_place_item.system())
        .add_system(tick_world.system())
        .run();
}

fn init_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    asset_server.load_asset_folder("assets").unwrap();

    let mut sprites = Sprites::default();

    sprites.cursor = color_materials.add(asset_server.get_handle("assets/sprites/grid_cell.png").unwrap().into());
    sprites.grid_object.insert(GridObjectType::Cheese, color_materials.add(asset_server.get_handle("assets/sprites/cheese.png").unwrap().into()));
    sprites.grid_object.insert(GridObjectType::Milk, color_materials.add(asset_server.get_handle("assets/sprites/milk.png").unwrap().into()));
    sprites.grid_object.insert(GridObjectType::GratedCheese, color_materials.add(asset_server.get_handle("assets/sprites/grated.png").unwrap().into()));
    

    sprites.machine.insert(MachineType::ConveyerBelt, color_materials.add(asset_server.get_handle("assets/sprites/conveyor.png").unwrap().into()));
    sprites.machine.insert(MachineType::Target, color_materials.add(asset_server.get_handle("assets/sprites/target.png").unwrap().into()));
    sprites.machine.insert(MachineType::Cow, color_materials.add(asset_server.get_handle("assets/sprites/cow.png").unwrap().into()));
    sprites.machine.insert(MachineType::Grater, color_materials.add(asset_server.get_handle("assets/sprites/grater.png").unwrap().into()));
    sprites.machine.insert(MachineType::Milker, color_materials.add(asset_server.get_handle("assets/sprites/milking_parlour.png").unwrap().into()));
    
    sprites.delete = color_materials.add(asset_server.get_handle("assets/sprites/delete.png").unwrap().into());

    let widget = commands
        .spawn(SpriteComponents {
            material: sprites.machine[&MachineType::ConveyerBelt].clone(),
            translation: (Vec3::new(-8., 4., 0.) * CELL_SIZE as f32).into(),
            ..Default::default()
        }).current_entity().unwrap();
    
    commands.insert_resource(MachinePlacementWidget {
        dir: 0x0,
        entity: widget,
        selected_machine: Some(MachineType::ConveyerBelt),
    });

    let camera_entity = commands
        .spawn(Camera2dComponents::default())
        .current_entity().unwrap();

    commands.push_children(camera_entity, &[widget]);
    
    let cursor_entity = commands
        .spawn(SpriteComponents {
            material: sprites.cursor.clone(),
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

        
    commands.insert_resource(sprites);

        
    // Grid background

    // Conveyer belt sprite

    // click and drag the map

    // scroll zoom


}

// TODO WT: Make these event triggered, probably a better idea
fn debug_place_item(
    mut commands: Commands,
    mut machine_widget: ResMut<MachinePlacementWidget>,
    sprites: Res<Sprites>,
    kb: Res<Input<KeyCode>>, 
    cursor: Res<Cursor>, 
    mut world: ResMut<GridWorld>,
    query: Query<(&mut Handle<ColorMaterial>, &mut Rotation)>,
) {
    let mut made_selection = false;
    let selected = 
        if kb.just_pressed(KeyCode::Key1) {
            made_selection = true;
            Some(MachineType::ConveyerBelt)
        }
        else if kb.just_pressed(KeyCode::Key2) {
            made_selection = true;
            Some(MachineType::Target)
        }
        else if kb.just_pressed(KeyCode::Key3) {
            made_selection = true;
            Some(MachineType::Cow)
        }
        else if kb.just_pressed(KeyCode::Key4) {
            made_selection = true;
            Some(MachineType::Milker)
        }
        else if kb.just_pressed(KeyCode::Key5) {
            made_selection = true;
            Some(MachineType::Grater)
        }
        else if kb.pressed(KeyCode::Back) { 
            made_selection = true;
            None
        } else { None };

    let mut update_rotation = false;
    if kb.just_pressed(KeyCode::Q) {
        machine_widget.dir = (machine_widget.dir + 1) % 4;
        update_rotation = true;
    }

    if kb.just_pressed(KeyCode::E) {
        machine_widget.dir = (machine_widget.dir - 1) % 4;
        update_rotation = true;
    }

    if update_rotation {
        if let Ok(mut rot) = query.get_mut::<Rotation>(machine_widget.entity) {
            rot.0 = Quat::from_rotation_z(machine_widget.dir as f32 * std::f32::consts::FRAC_PI_2);
        }
    }

    if made_selection {
        if let Some(selected_type) = selected {
            machine_widget.selected_machine.replace(selected_type);
    
            println!("Selected {:?}", &selected);
            
            if let Ok(mut sprite) = query.get_mut::<Handle<ColorMaterial>>(machine_widget.entity) {
                *sprite = sprites.machine[&selected_type];
            }
        } else {
            machine_widget.selected_machine = None;

            if let Ok(mut sprite) = query.get_mut::<Handle<ColorMaterial>>(machine_widget.entity) {
                *sprite = sprites.delete;
            }
        }
    }

    if kb.just_pressed(KeyCode::Return) {
        let machine_at = world.get_machine_at(cursor.pos);
        if  let Some(kind) = machine_widget.selected_machine {
            if let None = machine_at {
                world.create_machine(
                    kind,
                    sprites.machine[&kind],
                    cursor.pos,
                    machine_widget.dir,
                    &mut commands);
            }
        } else {
            if let Some(_) = machine_at {
                world.remove_machine(cursor.pos, &mut commands);
            }
        }
    }

    // TODO WT: Remove this debug bit
    if kb.just_pressed(KeyCode::Space) {
        // Spawn cheese at cursor
        let object_at = world.get_object_at(cursor.pos);
        if let None = object_at {
            world.create_object(
                GridObjectType::Cheese, 
                sprites.grid_object[&GridObjectType::Cheese],
                cursor.pos,
                &mut commands);
        }
    }
}