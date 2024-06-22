use crate::behavior::danmaku_data::DanmakuSpawnData;
use crate::form::Form;
use enumset::{EnumSet, EnumSetType};
use nalgebra::{Matrix4, UnitQuaternion};
use std::collections::HashMap;
use std::simd::{Simd, SimdElement};
use target_features::CURRENT_TARGET;

pub const N: usize = if let Some(size) = CURRENT_TARGET.suggested_simd_width::<f32>() {
    size
} else {
    // If SIMD isn't supported natively, we use a vector of 1 element.
    // This is effectively a scalar value.
    1
};

pub struct Columns {
    pub required_columns: EnumSet<DataColumns>,
    pub id: Vec<i128>,

    pub pos_x: Vec<Simd<f32, N>>,
    pub pos_y: Vec<Simd<f32, N>>,
    pub pos_z: Vec<Simd<f32, N>>,

    pub old_pos_x: Vec<Simd<f32, N>>,
    pub old_pos_y: Vec<Simd<f32, N>>,
    pub old_pos_z: Vec<Simd<f32, N>>,

    pub scale_x: Vec<Simd<f32, N>>,
    pub scale_y: Vec<Simd<f32, N>>,
    pub scale_z: Vec<Simd<f32, N>>,

    pub old_scale_x: Vec<Simd<f32, N>>,
    pub old_scale_y: Vec<Simd<f32, N>>,
    pub old_scale_z: Vec<Simd<f32, N>>,

    pub orientation: Vec<UnitQuaternion<f32>>,
    pub old_orientation: Vec<UnitQuaternion<f32>>,

    pub main_color: Vec<Simd<i32, N>>,
    pub secondary_color: Vec<Simd<i32, N>>,

    pub old_main_color: Vec<Simd<i32, N>>,
    pub old_secondary_color: Vec<Simd<i32, N>>,

    pub damage: Vec<Simd<f32, N>>,
    pub form: Vec<&'static Form>,
    pub render_properties: Vec<HashMap<&'static str, f32>>,

    pub ticks_existed: Vec<Simd<i16, N>>,
    pub end_time: Vec<Simd<i16, N>>,
    pub dead: Vec<bool>,
    pub next_stage: Vec<Vec<DanmakuSpawnData>>,
    pub next_stage_add_data: Vec<EnumSet<DataColumns>>,

    pub parent: Vec<i128>,

    pub transform_mats: Vec<Matrix4<f32>>,
    pub family_depth: Vec<i16>,

    pub current_dead: Vec<usize>,
    pub add_spawns: Vec<(DanmakuSpawnData, Option<usize>)>,

    // Behavior specific data
    pub motion_x: Vec<Simd<f32, N>>,
    pub motion_y: Vec<Simd<f32, N>>,
    pub motion_z: Vec<Simd<f32, N>>,

    pub gravity_x: Vec<Simd<f32, N>>,
    pub gravity_y: Vec<Simd<f32, N>>,
    pub gravity_z: Vec<Simd<f32, N>>,

    pub speed_accel: Vec<Simd<f32, N>>,

    pub forward_x: Vec<Simd<f32, N>>,
    pub forward_y: Vec<Simd<f32, N>>,
    pub forward_z: Vec<Simd<f32, N>>,

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

    fn sized_simd_always<A: SimdElement>(contents: A, max_column_size: usize) -> Vec<Simd<A, N>> {
        let amount = max_column_size.div_ceil(N);
        vec![Simd::splat(contents); amount]
    }

    fn sized_simd<A: SimdElement>(
        contents: A,
        required: EnumSet<DataColumns>,
        max_column_size: usize,
        required_column: DataColumns,
    ) -> Vec<Simd<A, N>> {
        if required.contains(required_column) {
            Self::sized_simd_always(contents, max_column_size)
        } else {
            Vec::new()
        }
    }

    pub fn new(max_column_size: usize, required: EnumSet<DataColumns>) -> Columns {
        Columns {
            required_columns: required,

            id: vec![0; max_column_size],
            pos_x: Self::sized_simd(0.0, required, max_column_size, DataColumns::PosX),
            pos_y: Self::sized_simd(0.0, required, max_column_size, DataColumns::PosY),
            pos_z: Self::sized_simd(0.0, required, max_column_size, DataColumns::PosZ),
            old_pos_x: Self::sized_simd(0.0, required, max_column_size, DataColumns::PosX),
            old_pos_y: Self::sized_simd(0.0, required, max_column_size, DataColumns::PosY),
            old_pos_z: Self::sized_simd(0.0, required, max_column_size, DataColumns::PosZ),
            scale_x: Self::sized_simd(0.0, required, max_column_size, DataColumns::ScaleX),
            scale_y: Self::sized_simd(0.0, required, max_column_size, DataColumns::ScaleX),
            scale_z: Self::sized_simd(0.0, required, max_column_size, DataColumns::ScaleX),
            old_scale_x: Self::sized_simd(0.0, required, max_column_size, DataColumns::ScaleX),
            old_scale_y: Self::sized_simd(0.0, required, max_column_size, DataColumns::ScaleY),
            old_scale_z: Self::sized_simd(0.0, required, max_column_size, DataColumns::ScaleZ),
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
            main_color: Self::sized_simd(0, required, max_column_size, DataColumns::MainColor),
            secondary_color: Self::sized_simd(
                0,
                required,
                max_column_size,
                DataColumns::SecondaryColor,
            ),
            old_main_color: Self::sized_simd(0, required, max_column_size, DataColumns::MainColor),
            old_secondary_color: Self::sized_simd(
                0,
                required,
                max_column_size,
                DataColumns::SecondaryColor,
            ),
            damage: Self::sized_simd(0.0, required, max_column_size, DataColumns::Damage),
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
            ticks_existed: Self::sized_simd_always(0, max_column_size),
            end_time: Self::sized_simd_always(0, max_column_size),
            dead: vec![false; max_column_size],
            next_stage: vec![Vec::new(); max_column_size],
            next_stage_add_data: vec![EnumSet::EMPTY; max_column_size],
            parent: vec![-1; max_column_size],
            transform_mats: vec![Matrix4::identity(); max_column_size],
            family_depth: vec![0; max_column_size],
            current_dead: Vec::new(),
            add_spawns: Vec::new(),

            // Behavior specific data
            motion_x: Self::sized_simd(0.0, required, max_column_size, DataColumns::MotionX),
            motion_y: Self::sized_simd(0.0, required, max_column_size, DataColumns::MotionY),
            motion_z: Self::sized_simd(0.0, required, max_column_size, DataColumns::MotionZ),
            gravity_x: Self::sized_simd(0.0, required, max_column_size, DataColumns::GravityX),
            gravity_y: Self::sized_simd(0.0, required, max_column_size, DataColumns::GravityY),
            gravity_z: Self::sized_simd(0.0, required, max_column_size, DataColumns::GravityZ),
            speed_accel: Self::sized_simd(0.0, required, max_column_size, DataColumns::SpeedAccel),

            forward_x: Self::sized_simd(1.0, required, max_column_size, DataColumns::Forward),
            forward_y: Self::sized_simd(1.0, required, max_column_size, DataColumns::Forward),
            forward_z: Self::sized_simd(1.0, required, max_column_size, DataColumns::Forward),
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

    fn resize_simd<A: Clone + SimdElement>(
        new_max_size: usize,
        vec: &mut Vec<Simd<A, N>>,
        new_obj: A,
    ) {
        let chunks = new_max_size.div_ceil(N);
        vec.resize(chunks, Simd::splat(new_obj))
    }

    fn resize_simd_if_required<A: Clone + SimdElement>(
        required_columns: EnumSet<DataColumns>,
        new_max_size: usize,
        required_column: DataColumns,
        vec: &mut Vec<Simd<A, N>>,
        new_obj: A,
    ) {
        if required_columns.contains(required_column) {
            Self::resize_simd(new_max_size, vec, new_obj);
        }
    }

    pub fn resize(&mut self, new_max_size: usize) {
        self.id.resize(new_max_size, 0);

        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosX,
            &mut self.pos_x,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosY,
            &mut self.pos_y,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosZ,
            &mut self.pos_z,
            0.0,
        );

        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosX,
            &mut self.old_pos_x,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosY,
            &mut self.old_pos_y,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::PosZ,
            &mut self.old_pos_z,
            0.0,
        );

        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleX,
            &mut self.scale_x,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleY,
            &mut self.scale_y,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleZ,
            &mut self.scale_z,
            0.0,
        );

        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleX,
            &mut self.scale_x,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::ScaleY,
            &mut self.scale_y,
            0.0,
        );
        Self::resize_simd_if_required(
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

        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MainColor,
            &mut self.main_color,
            0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MainColor,
            &mut self.old_main_color,
            0,
        );

        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::SecondaryColor,
            &mut self.secondary_color,
            0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::SecondaryColor,
            &mut self.old_secondary_color,
            0,
        );

        Self::resize_simd_if_required(
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

        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionX,
            &mut self.motion_x,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionY,
            &mut self.motion_y,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionZ,
            &mut self.motion_z,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionX,
            &mut self.gravity_x,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionY,
            &mut self.gravity_y,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::MotionZ,
            &mut self.gravity_z,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::SpeedAccel,
            &mut self.speed_accel,
            0.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Forward,
            &mut self.forward_x,
            1.0,
        );
        Self::resize_simd_if_required(
            self.required_columns,
            new_max_size,
            DataColumns::Forward,
            &mut self.forward_y,
            0.0,
        );
        Self::resize_simd_if_required(
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

        Self::resize_simd(new_max_size, &mut self.ticks_existed, 0);
        Self::resize_simd(new_max_size, &mut self.end_time, 0);
        self.dead.resize(new_max_size, false);
        self.next_stage.resize(new_max_size, Vec::new());
        self.next_stage_add_data
            .resize(new_max_size, EnumSet::EMPTY);

        self.parent.resize(new_max_size, -1);
        self.transform_mats
            .resize(new_max_size, Matrix4::identity());

        self.family_depth.resize(new_max_size, 0);
    }

    fn compact_vec<A: Clone>(vec: &mut Vec<A>, remove: &[bool], new_max_size: usize, value: A) {
        let mut j = 0;
        vec.retain(|_| {
            j += 1;
            let to_remove = *remove.get(j - 1).unwrap_or(&false);
            !to_remove
        });
        vec.resize(new_max_size, value);
    }

    fn compact_simd<A: SimdElement + Clone>(
        vec: &mut Vec<Simd<A, N>>,
        remove: &[bool],
        new_max_size: usize,
        value: A,
    ) {
        let mut new_vec = vec![value; new_max_size];
        let mut stored_so_far = 0;
        vec.iter().enumerate().for_each(|(idx, v)| {
            let from = idx / N;
            let slice = &remove[from..from + N];
            let mut arr = [false; N];
            let len = arr.len();
            arr.copy_from_slice(&slice[..len]);

            let mask = !std::simd::Mask::from_array(arr);
            v.store_select(&mut new_vec[stored_so_far..stored_so_far + N], mask);
            stored_so_far += slice.iter().filter(|v| !*v).count();
        });

        vec.resize(new_max_size.div_ceil(N), Simd::splat(value));
        for i in 0..new_max_size.div_ceil(N) {
            vec[i] = Simd::load_or(&new_vec[i * N..(i + 1) * N], Simd::splat(value));
        }
    }

    pub fn compact(&mut self, new_max_size: usize) {
        let dead = &self.dead;

        [&mut self.id, &mut self.parent]
            .iter_mut()
            .for_each(|d| Self::compact_vec(d, dead, new_max_size, -1));
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
        ]
        .iter_mut()
        .for_each(|d| Self::compact_simd(d, dead, new_max_size, 0.0));

        [
            &mut self.forward_x,
            &mut self.forward_y,
            &mut self.forward_z,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_simd(d, dead, new_max_size, 1.0));

        [
            &mut self.orientation,
            &mut self.old_orientation,
            &mut self.rotation,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead, new_max_size, UnitQuaternion::identity()));

        [
            &mut self.main_color,
            &mut self.old_main_color,
            &mut self.secondary_color,
            &mut self.old_secondary_color,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_simd(d, dead, new_max_size, 0));

        Self::compact_vec(&mut self.form, dead, new_max_size, &Form::SPHERE);
        Self::compact_vec(
            &mut self.render_properties,
            dead,
            new_max_size,
            HashMap::new(),
        );

        [&mut self.ticks_existed, &mut self.end_time]
            .iter_mut()
            .for_each(|d| Self::compact_simd(d, dead, new_max_size, 0));

        Self::compact_vec(&mut self.family_depth, dead, new_max_size, 0);

        Self::compact_vec(&mut self.next_stage, dead, new_max_size, Vec::new());
        Self::compact_vec(
            &mut self.next_stage_add_data,
            dead,
            new_max_size,
            EnumSet::new(),
        );
        Self::compact_vec(
            &mut self.transform_mats,
            dead,
            new_max_size,
            Matrix4::identity(),
        );

        let _ = &mut self.dead.retain(|d| *d);
        self.dead.resize(new_max_size, false);
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
