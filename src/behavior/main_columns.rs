use crate::behavior::danmaku_data::DanmakuSpawnData;
use crate::form::Form;
use enumset::{EnumSet, EnumSetType};
use nalgebra::{Matrix4, UnitQuaternion};
use std::collections::HashMap;

pub struct Columns {
    pub required_columns: EnumSet<DataColumns>,
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
    pub next_stage_add_data: Vec<EnumSet<DataColumns>>,

    pub parent: Vec<i128>,

    pub transform_mats: Vec<Matrix4<f32>>,
    pub family_depth: Vec<i16>,

    pub current_dead: Vec<usize>,
    pub add_spawns: Vec<(DanmakuSpawnData, Option<usize>)>,

    // Behavior specific data
    pub motion_x: Vec<f32>,
    pub motion_y: Vec<f32>,
    pub motion_z: Vec<f32>,

    pub gravity_x: Vec<f32>,
    pub gravity_y: Vec<f32>,
    pub gravity_z: Vec<f32>,

    pub speed_accel: Vec<f32>,

    pub forward_x: Vec<f32>,
    pub forward_y: Vec<f32>,
    pub forward_z: Vec<f32>,

    pub rotation: Vec<UnitQuaternion<f32>>,
}

impl Columns {
    fn sized_vec<A: Clone>(
        contents: A,
        required: EnumSet<DataColumns>,
        max_column_size: usize,
        required_column: DataColumns,
    ) -> Vec<A> {
        if required.contains(required_column) {
            vec![contents; max_column_size]
        } else {
            Vec::new()
        }
    }

    pub fn new(max_column_size: usize, required: EnumSet<DataColumns>) -> Columns {
        Columns {
            required_columns: required,

            id: vec![0; max_column_size],
            pos_x: Self::sized_vec(0.0, required, max_column_size, DataColumns::PosX),
            pos_y: Self::sized_vec(0.0, required, max_column_size, DataColumns::PosY),
            pos_z: Self::sized_vec(0.0, required, max_column_size, DataColumns::PosZ),
            old_pos_x: Self::sized_vec(0.0, required, max_column_size, DataColumns::PosX),
            old_pos_y: Self::sized_vec(0.0, required, max_column_size, DataColumns::PosY),
            old_pos_z: Self::sized_vec(0.0, required, max_column_size, DataColumns::PosZ),
            scale_x: Self::sized_vec(0.0, required, max_column_size, DataColumns::ScaleX),
            scale_y: Self::sized_vec(0.0, required, max_column_size, DataColumns::ScaleX),
            scale_z: Self::sized_vec(0.0, required, max_column_size, DataColumns::ScaleX),
            old_scale_x: Self::sized_vec(0.0, required, max_column_size, DataColumns::ScaleX),
            old_scale_y: Self::sized_vec(0.0, required, max_column_size, DataColumns::ScaleY),
            old_scale_z: Self::sized_vec(0.0, required, max_column_size, DataColumns::ScaleZ),
            orientation: Self::sized_vec(
                UnitQuaternion::identity(),
                required,
                max_column_size,
                DataColumns::Orientation,
            ),
            old_orientation: Self::sized_vec(
                UnitQuaternion::identity(),
                required,
                max_column_size,
                DataColumns::Orientation,
            ),
            main_color: Self::sized_vec(0, required, max_column_size, DataColumns::MainColor),
            secondary_color: Self::sized_vec(
                0,
                required,
                max_column_size,
                DataColumns::SecondaryColor,
            ),
            old_main_color: Self::sized_vec(0, required, max_column_size, DataColumns::MainColor),
            old_secondary_color: Self::sized_vec(
                0,
                required,
                max_column_size,
                DataColumns::SecondaryColor,
            ),
            damage: Self::sized_vec(0.0, required, max_column_size, DataColumns::Damage),
            form: Self::sized_vec(
                &Form::SPHERE,
                required,
                max_column_size,
                DataColumns::Appearance,
            ),
            render_properties: Self::sized_vec(
                HashMap::new(),
                required,
                max_column_size,
                DataColumns::Appearance,
            ),
            ticks_existed: vec![0; max_column_size],
            end_time: vec![0; max_column_size],
            dead: vec![false; max_column_size],
            next_stage: vec![Vec::new(); max_column_size],
            next_stage_add_data: vec![EnumSet::EMPTY; max_column_size],
            parent: vec![-1; max_column_size],
            transform_mats: vec![Matrix4::identity(); max_column_size],
            family_depth: vec![0; max_column_size],
            current_dead: Vec::new(),
            add_spawns: Vec::new(),

            // Behavior specific data
            motion_x: Self::sized_vec(0.0, required, max_column_size, DataColumns::MotionX),
            motion_y: Self::sized_vec(0.0, required, max_column_size, DataColumns::MotionY),
            motion_z: Self::sized_vec(0.0, required, max_column_size, DataColumns::MotionZ),
            gravity_x: Self::sized_vec(0.0, required, max_column_size, DataColumns::GravityX),
            gravity_y: Self::sized_vec(0.0, required, max_column_size, DataColumns::GravityY),
            gravity_z: Self::sized_vec(0.0, required, max_column_size, DataColumns::GravityZ),
            speed_accel: Self::sized_vec(0.0, required, max_column_size, DataColumns::SpeedAccel),

            forward_x: Self::sized_vec(1.0, required, max_column_size, DataColumns::Forward),
            forward_y: Self::sized_vec(1.0, required, max_column_size, DataColumns::Forward),
            forward_z: Self::sized_vec(1.0, required, max_column_size, DataColumns::Forward),
            rotation: Self::sized_vec(
                UnitQuaternion::identity(),
                required,
                max_column_size,
                DataColumns::Rotation,
            ),
        }
    }

    pub fn grab_new_spawns(&mut self) -> Vec<(DanmakuSpawnData, Option<usize>)> {
        std::mem::take(&mut self.add_spawns)
    }

    fn resize_if_required<A: Clone>(
        required_columns: EnumSet<DataColumns>,
        new_max_size: usize,
        required_column: DataColumns,
        vec: &mut Vec<A>,
        new_obj: A,
    ) {
        if required_columns.contains(required_column) {
            vec.resize(new_max_size, new_obj)
        }
    }

    pub fn resize(&mut self, new_max_size: usize) {
        self.id.resize(new_max_size, 0);

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosX,
            &mut self.pos_x,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosY,
            &mut self.pos_y,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosZ,
            &mut self.pos_z,
            0.0,
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosX,
            &mut self.old_pos_x,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosY,
            &mut self.old_pos_y,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosZ,
            &mut self.old_pos_z,
            0.0,
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleX,
            &mut self.scale_x,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleY,
            &mut self.scale_y,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleZ,
            &mut self.scale_z,
            0.0,
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleX,
            &mut self.scale_x,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleY,
            &mut self.scale_y,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleZ,
            &mut self.scale_z,
            0.0,
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Orientation,
            &mut self.orientation,
            UnitQuaternion::identity(),
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Orientation,
            &mut self.old_orientation,
            UnitQuaternion::identity(),
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MainColor,
            &mut self.main_color,
            0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MainColor,
            &mut self.old_main_color,
            0,
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::SecondaryColor,
            &mut self.secondary_color,
            0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::SecondaryColor,
            &mut self.old_secondary_color,
            0,
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Damage,
            &mut self.damage,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Appearance,
            &mut self.form,
            &Form::SPHERE,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Appearance,
            &mut self.render_properties,
            HashMap::new(),
        );

        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionX,
            &mut self.motion_x,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionY,
            &mut self.motion_y,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionZ,
            &mut self.motion_z,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionX,
            &mut self.gravity_x,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionY,
            &mut self.gravity_y,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionZ,
            &mut self.gravity_z,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::SpeedAccel,
            &mut self.speed_accel,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Forward,
            &mut self.forward_x,
            1.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Forward,
            &mut self.forward_y,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Forward,
            &mut self.forward_z,
            0.0,
        );
        Self::resize_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Rotation,
            &mut self.rotation,
            UnitQuaternion::identity(),
        );

        self.ticks_existed.resize(new_max_size, 0);
        self.end_time.resize(new_max_size, 0);
        self.dead.resize(new_max_size, false);
        self.next_stage.resize(new_max_size, Vec::new());
        self.next_stage_add_data.resize(new_max_size, EnumSet::EMPTY);

        self.parent.resize(new_max_size, -1);
        self.transform_mats
            .resize(new_max_size, Matrix4::identity());
        self.family_depth.resize(new_max_size, 0);
    }

    fn compact_vec<A>(vec: &mut Vec<A>, remove: &[bool]) {
        let mut j = 0;
        vec.retain(|_| {
            j += 1;
            *remove.get(j - 1).unwrap_or(&false)
        })
    }

    pub fn compact(&mut self) {
        let dead = &self.dead;

        [&mut self.id, &mut self.parent]
            .iter_mut()
            .for_each(|d| Self::compact_vec(d, dead));
        [
            &mut self.pos_x,
            &mut self.pos_y,
            &mut self.pos_z,
            &mut self.old_pos_x,
            &mut self.old_pos_y,
            &mut self.old_pos_z,
            &mut self.scale_x,
            &mut self.scale_y,
            &mut self.scale_z,
            &mut self.old_scale_x,
            &mut self.old_scale_y,
            &mut self.old_scale_z,
            &mut self.damage,
            &mut self.motion_x,
            &mut self.motion_y,
            &mut self.motion_z,
            &mut self.gravity_x,
            &mut self.gravity_y,
            &mut self.gravity_z,
            &mut self.speed_accel,
            &mut self.forward_x,
            &mut self.forward_y,
            &mut self.forward_z,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        [
            &mut self.orientation,
            &mut self.old_orientation,
            &mut self.rotation,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        [
            &mut self.main_color,
            &mut self.old_main_color,
            &mut self.secondary_color,
            &mut self.old_secondary_color,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        Self::compact_vec(&mut self.form, dead);
        Self::compact_vec(&mut self.render_properties, dead);

        [
            &mut self.ticks_existed,
            &mut self.end_time,
            &mut self.family_depth,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        Self::compact_vec(&mut self.next_stage, dead);
        Self::compact_vec(&mut self.next_stage_add_data, dead);
        Self::compact_vec(&mut self.transform_mats, dead);

        let _ = &mut self.dead.retain(|d| *d);
        let _ = &mut self.current_dead.clear();
    }
}

#[derive(Debug, Hash, EnumSetType)]
pub enum DataColumns {
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

    MotionX,
    MotionY,
    MotionZ,
    GravityX,
    GravityY,
    GravityZ,
    SpeedAccel,

    Rotation,
    Forward,
}
