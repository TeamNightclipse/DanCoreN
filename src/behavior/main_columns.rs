use std::collections::HashMap;

use enumset::{EnumSet, EnumSetType};
use nalgebra::{Matrix4, UnitQuaternion};

use crate::behavior::danmaku_data::DanmakuSpawnData;
use crate::form::Form;

pub struct Columns {
    pub required_main_columns: EnumSet<RequiredMainColumns>,
    pub id: Vec<i128>,

    pub pos_x: Vec<f32>,
    pub pos_y: Vec<f32>,
    pub pos_z: Vec<f32>,

    pub old_pos_x: Vec<f32>,
    pub old_pos_y: Vec<f32>,
    pub old_pos_z: Vec<f32>,

    pub scale_x: Vec<f32>,
    pub scale_y: Vec<f32>,
    pub scale_z: Vec<f32>,

    pub old_scale_x: Vec<f32>,
    pub old_scale_y: Vec<f32>,
    pub old_scale_z: Vec<f32>,

    pub orientation: Vec<UnitQuaternion<f32>>,
    pub old_orientation: Vec<UnitQuaternion<f32>>,

    pub main_color: Vec<i32>,
    pub secondary_color: Vec<i32>,

    pub old_main_color: Vec<i32>,
    pub old_secondary_color: Vec<i32>,

    pub damage: Vec<f32>,
    pub form: Vec<&'static Form>,
    pub render_properties: Vec<HashMap<&'static str, f32>>,

    pub ticks_existed: Vec<i16>,
    pub end_time: Vec<i16>,
    pub dead: Vec<bool>,
    pub next_stage: Vec<Vec<DanmakuSpawnData>>,

    pub parent: Vec<i128>,

    pub transform_mats: Vec<Matrix4<f32>>,
    pub family_depth: Vec<i16>,

    pub extra_data: HashMap<&'static str, Vec<f32>>,

    pub current_dead: Vec<usize>,
    add_spawns: Vec<(DanmakuSpawnData, usize)>,
}

impl Columns {
    pub fn new(max_column_size: usize, required: EnumSet<RequiredMainColumns>, extra_data_identifiers: Vec<&'static str>) -> Columns {
        Columns {
            required_main_columns: required,
            
            id: vec![0; max_column_size],
            pos_x: vec![0.0; if required.contains(RequiredMainColumns::PosX) { max_column_size } else { 0 }],
            pos_y: vec![0.0; if required.contains(RequiredMainColumns::PosY) { max_column_size } else { 0 }],
            pos_z: vec![0.0; if required.contains(RequiredMainColumns::PosZ) { max_column_size } else { 0 }],
            old_pos_x: vec![0.0; if required.contains(RequiredMainColumns::PosX) { max_column_size } else { 0 }],
            old_pos_y: vec![0.0; if required.contains(RequiredMainColumns::PosY) { max_column_size } else { 0 }],
            old_pos_z: vec![0.0; if required.contains(RequiredMainColumns::PosZ) { max_column_size } else { 0 }],
            scale_x: vec![0.0; if required.contains(RequiredMainColumns::ScaleX) { max_column_size } else { 0 }],
            scale_y: vec![0.0; if required.contains(RequiredMainColumns::ScaleY) { max_column_size } else { 0 }],
            scale_z: vec![0.0; if required.contains(RequiredMainColumns::ScaleZ) { max_column_size } else { 0 }],
            old_scale_x: vec![0.0; if required.contains(RequiredMainColumns::ScaleX) { max_column_size } else { 0 }],
            old_scale_y: vec![0.0; if required.contains(RequiredMainColumns::ScaleY) { max_column_size } else { 0 }],
            old_scale_z: vec![0.0; if required.contains(RequiredMainColumns::ScaleZ) { max_column_size } else { 0 }],
            orientation: vec![UnitQuaternion::identity(); if required.contains(RequiredMainColumns::Orientation) { max_column_size } else { 0 }],
            old_orientation: vec![UnitQuaternion::identity(); if required.contains(RequiredMainColumns::Orientation) { max_column_size } else { 0 }],
            main_color: vec![0; if required.contains(RequiredMainColumns::MainColor) { max_column_size } else { 0 }],
            secondary_color: vec![0; if required.contains(RequiredMainColumns::SecondaryColor) { max_column_size } else { 0 }],
            old_main_color: vec![0; if required.contains(RequiredMainColumns::MainColor) { max_column_size } else { 0 }],
            old_secondary_color: vec![0; if required.contains(RequiredMainColumns::SecondaryColor) { max_column_size } else { 0 }],
            damage: vec![0.0; if required.contains(RequiredMainColumns::Damage) { max_column_size } else { 0 }],
            form: vec![&Form::SPHERE; if required.contains(RequiredMainColumns::Appearance) { max_column_size } else { 0 }],
            render_properties: vec![HashMap::new(); if required.contains(RequiredMainColumns::Appearance) { max_column_size } else { 0 }],
            ticks_existed: vec![0; max_column_size],
            end_time: vec![0; max_column_size],
            dead: vec![false; max_column_size],
            next_stage: {
                let mut r = Vec::new();
                r.resize_with(max_column_size, Vec::new);
                r
            },
            parent: vec![-1; max_column_size],
            transform_mats: vec![Matrix4::identity(); max_column_size],
            family_depth: vec![0; max_column_size],
            extra_data: extra_data_identifiers.into_iter().map(|id| (id, vec![0.0; max_column_size])).collect(),
            current_dead: Vec::new(),
            add_spawns: Vec::new(),
        }
    }

    pub fn grab_new_spawns(&mut self) -> Vec<(DanmakuSpawnData, usize)> {
        std::mem::take(&mut self.add_spawns)
    }
    
    pub fn add_spawns(&mut self, mut spawns: Vec<(DanmakuSpawnData, usize)>) {
        self.add_spawns.append(&mut spawns)
    }
    
    pub fn resize(&mut self, new_max_size: usize) {
        self.id.resize(new_max_size, 0);
        
        self.pos_x.resize(new_max_size, 0.0);
        self.pos_y.resize(new_max_size, 0.0);
        self.pos_z.resize(new_max_size, 0.0);

        self.old_pos_x.resize(new_max_size, 0.0);
        self.old_pos_y.resize(new_max_size, 0.0);
        self.old_pos_z.resize(new_max_size, 0.0);

        self.scale_x.resize(new_max_size, 0.0);
        self.scale_y.resize(new_max_size, 0.0);
        self.scale_z.resize(new_max_size, 0.0);

        self.old_scale_x.resize(new_max_size, 0.0);
        self.old_scale_y.resize(new_max_size, 0.0);
        self.old_scale_z.resize(new_max_size, 0.0);
        
        self.orientation.resize(new_max_size, UnitQuaternion::identity());
        self.old_orientation.resize(new_max_size, UnitQuaternion::identity());
        
        self.main_color.resize(new_max_size, 0);
        self.secondary_color.resize(new_max_size, 0);

        self.old_main_color.resize(new_max_size, 0);
        self.old_secondary_color.resize(new_max_size, 0);
        
        self.damage.resize(new_max_size, 0.0);
        self.form.resize(new_max_size, &Form::SPHERE);
        self.render_properties.resize(new_max_size, HashMap::new());
        
        self.ticks_existed.resize(new_max_size, 0);
        self.end_time.resize(new_max_size, 0);
        self.dead.resize(new_max_size, false);
        self.next_stage.resize_with(new_max_size, Vec::new);
        
        self.parent.resize(new_max_size, -1);
        self.transform_mats.resize(new_max_size, Matrix4::identity());
        self.family_depth.resize(new_max_size, 0);
        
        self.extra_data.values_mut().for_each(|v| v.resize(new_max_size, 0.0))
    }
}

#[derive(Debug, Hash, EnumSetType)]
pub enum RequiredMainColumns {
    PosX,
    PosY,
    PosZ,
    ScaleX,
    ScaleY,
    ScaleZ,
    Orientation,
    MainColor,
    SecondaryColor,
    Damage,
    Appearance,
}