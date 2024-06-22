use std::collections::HashMap;

use enumset::{EnumSet, EnumSetType};
use nalgebra::Matrix4;

use crate::form::Form;

#[derive(Clone)]
pub struct DanmakuSpawnData<SpawnData, DataColumns: EnumSetType> {
    pub end_time: i16,
    pub behavior_data: Vec<SpawnData>,
    pub render_properties: HashMap<&'static str, f32>,
    pub behaviors: Vec<&'static str>,
    pub next_stage_add_data: EnumSet<DataColumns>,
    pub next_stage: Vec<DanmakuSpawnData<SpawnData, DataColumns>>,
    pub parent: Option<i128>,
    pub children: Vec<DanmakuSpawnData<SpawnData, DataColumns>>,
    pub family_depth: i16,
}
impl<SD, DC: EnumSetType> DanmakuSpawnData<SD, DC> {
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
