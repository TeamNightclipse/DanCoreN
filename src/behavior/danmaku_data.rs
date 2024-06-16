use std::collections::HashMap;

use nalgebra::{Matrix4, UnitQuaternion, Vector3};

use crate::behavior::Behavior;
use crate::form::Form;

pub struct DanmakuSpawnData {
    pub pos: Vector3<f32>,
    pub orientation: UnitQuaternion<f32>,
    pub shot_data: ShotData,
    pub behavior: Vec<Box<dyn Behavior>>,
    pub next_stage: Vec<DanmakuSpawnData>,
    pub parent: Option<i128>,
    pub children: Vec<DanmakuSpawnData>,
    pub family_depth: i16,
}
impl DanmakuSpawnData {
    fn update_children_depth(&mut self) {
        self.children.iter_mut().for_each(|child| {
            child.family_depth = self.family_depth + 1;
            child.update_children_depth()
        })
    }

    pub(crate) fn set_family_depth(
        &mut self,
        global_family_depth_map: &HashMap<i128, i16>,
    ) -> bool {
        if self.family_depth != -1 {
            true
        } else {
            match self.parent {
                None if self.family_depth == 0 => true,
                None => {
                    self.family_depth = 0;
                    true
                }
                Some(parent_id) => global_family_depth_map
                    .get(&parent_id)
                    .map_or(false, |depth| {
                        self.family_depth = *depth;
                        self.update_children_depth();
                        true
                    }),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShotData {
    pub form: &'static Form,
    pub render_properties: HashMap<&'static str, f32>,
    pub main_color: i32,
    pub secondary_color: i32,
    pub damage: f32,
    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
    pub end_time: i16,
}

impl ShotData {
    pub fn new(form: &'static Form) -> ShotData {
        ShotData {
            form,
            render_properties: HashMap::new(),
            main_color: 0xFF0000,
            secondary_color: 0xFFFFFF,
            damage: 2.0,
            size_x: 0.5,
            size_y: 0.5,
            size_z: 0.5,
            end_time: 100,
        }
    }
}

pub struct RenderData<'a> {
    pub form: &'static Form,
    pub render_properties: &'a HashMap<&'static str, f32>,
    pub model_mat: Matrix4<f32>,
    pub model_view_mat: Matrix4<f32>,
    pub main_color: i32,
    pub secondary_color: i32,
    pub ticks_existed: i16,
    pub end_time: i16,
    pub distance_from_camera: f64,
}
