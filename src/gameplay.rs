use bevy::prelude::*;

pub const MAX_WORLD_COORD: u32 = 63;
pub const CELL_SIZE: u32 = 32;

pub struct GridWorld {
    pub tick_timer: Timer,
    pub machines: Vec<Machine>,
    pub objects: Vec<GridObject>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MachineType {
    ConveyerBelt,
    Target,
    Cow,
    Milker,
    Grater,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GridObjectType {
    Cheese,
    Milk,
    GratedCheese,
}

pub struct Machine {
    pub dir: i8,
    pub kind: MachineType,
    pub pos: Vec2,
    pub entity: Entity,
}

// TODO WT: Use Bevy UI for this
pub struct MachinePlacementWidget {
    pub dir: i8,
    pub entity: Entity,
    pub selected_machine: Option<MachineType>,
}

pub struct GridObject {
    pub kind: GridObjectType,
    pub entity: Entity,
    pub pos: Vec2,
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

impl GridWorld {
    pub fn create_machine(&mut self, kind: MachineType, sprite: Handle<ColorMaterial>, location: Vec2, rotation: i8, commands: &mut Commands) {
        let entity = commands.spawn(SpriteComponents {
            material: sprite,
            translation: Translation::new(location.x() * CELL_SIZE as f32, location.y() * CELL_SIZE as f32, 1.),
            rotation: Quat::from_rotation_z(rotation as f32 * std::f32::consts::FRAC_PI_2).into(),
            ..Default::default()
        }).current_entity().unwrap();

        self.machines.push(Machine {
            kind,
            pos: location,
            dir: rotation,
            entity,
        });
    }

    pub fn create_object(&mut self, kind: GridObjectType, sprite: Handle<ColorMaterial>, location: Vec2, commands: &mut Commands) {
        let entity = commands.spawn(SpriteComponents {
            material: sprite,
            translation: Translation::new(location.x() * CELL_SIZE as f32, location.y() * CELL_SIZE as f32, 2.),
            ..Default::default()
        })
        .current_entity().unwrap();

        self.objects.push(GridObject {
            kind,
            pos: location,
            entity,
        });
    }

    pub fn remove_machine(&mut self, location: Vec2, commands: &mut Commands) {
        for i in 0..self.machines.len() {
            let machine = &self.machines[i];
            if self.machines[i].pos.x() == location.x() && self.machines[i].pos.y() == location.y() {
                commands.despawn(machine.entity);

                let last = self.machines.len() - 1;

                self.machines.swap(i, last);

                self.machines.pop();

                return;
            }
        }
    }

    pub fn remove_object(&mut self, location: Vec2, commands: &mut Commands) {
        for i in 0..self.objects.len() {
            let object = &self.objects[i];
            if self.objects[i].pos.x() == location.x() && self.objects[i].pos.y() == location.y() {
                commands.despawn(object.entity);

                let last = self.objects.len() - 1;

                self.objects.swap(i, last);

                self.objects.pop();

                return;
            }
        }
    }

    pub fn get_machine_at(&self, location: Vec2) -> Option<&Machine> {
        for obj in self.machines.iter() {
            if obj.pos.x() == location.x() && obj.pos.y() == location.y() {
                return Some(&obj);
            }
        }

        return None
    }

    pub fn get_object_at(&self, location: Vec2) -> Option<&GridObject> {
        for obj in self.objects.iter() {
            if obj.pos.x() == location.x() && obj.pos.y() == location.y() {
                return Some(obj);
            }
        }

        return None
    }

    pub fn get_object_at_mut(&mut self, location: Vec2) -> Option<&mut GridObject> {
        for obj in self.objects.iter_mut() {
            if obj.pos.x() == location.x() && obj.pos.y() == location.y() {
                return Some(obj);
            }
        }

        return None
    }
}