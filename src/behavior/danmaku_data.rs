use std::collections::HashMap;
use enumset::EnumSet;

use nalgebra::{Matrix4, UnitQuaternion, UnitVector3};
use crate::behavior::main_columns::DataColumns;

use crate::form::Form;

#[derive(Clone, Debug)]
pub enum BehaviorData {
    PosX(f32),
    PosY(f32),
    PosZ(f32),
    Orientation(UnitQuaternion<f32>),
    Appearance {
        form: &'static Form,
    },
    MainColor(i32),
    SecondaryColor(i32),
    Damage(f32),
    SizeX(f32),
    SizeY(f32),
    SizeZ(f32),
    
    MotionX(f32),
    MotionY(f32),
    MotionZ(f32),

    GravityX(f32),
    GravityY(f32),
    GravityZ(f32),
    
    SpeedAccel(f32),
    Forward(UnitVector3<f32>),
    Rotation(UnitQuaternion<f32>)
}

#[derive(Clone)]
pub struct DanmakuSpawnData {
    pub end_time: i16,
    pub behavior_data: Vec<BehaviorData>,
    pub render_properties: HashMap<&'static str, f32>,
    pub behaviors: Vec<&'static str>,
    pub next_stage_add_data: EnumSet<DataColumns>,
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

pub struct RenderData<'a> {
    pub form: &'static Form,
    pub render_properties: &'a HashMap<&'static str, f32>,
    pub model_mat: Matrix4<f32>,
    pub main_color: i32,
    pub secondary_color: i32,
    pub ticks_existed: i16,
    pub end_time: i16,
}
