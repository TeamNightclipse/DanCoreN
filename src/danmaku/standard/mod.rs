use std::collections::HashMap;
use std::simd::{Simd, SimdElement};

use enumset::{EnumSet, EnumSetType};
use nalgebra::{Matrix4, UnitQuaternion, UnitVector3, Vector3};

use crate::color::ColorHex;
use crate::danmaku::{
    data::{DanmakuSpawnData, RenderData},
    DanmakuData, N,
};
use crate::form::Form;

pub mod behaviors;

pub struct StandardColumns {
    pub required_columns: EnumSet<StandardDataColumns>,
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
    pub next_stage: Vec<Vec<DanmakuSpawnData<StandardSpawnData, StandardDataColumns>>>,
    pub next_stage_add_data: Vec<EnumSet<StandardDataColumns>>,

    pub parent: Vec<i128>,

    pub transform_mats: Vec<Matrix4<f32>>,
    pub family_depth: Vec<i16>,

    pub current_dead: Vec<usize>,
    pub add_spawns: Vec<(
        DanmakuSpawnData<StandardSpawnData, StandardDataColumns>,
        Option<usize>,
    )>,

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

impl DanmakuData for StandardColumns {
    type DataColumns = StandardDataColumns;
    type SpawnData = StandardSpawnData;

    fn new(max_column_size: usize, required: EnumSet<StandardDataColumns>) -> StandardColumns {
        fn sized_vec<A: Clone>(
            contents: A,
            required: EnumSet<StandardDataColumns>,
            max_column_size: usize,
            required_column: StandardDataColumns,
        ) -> Vec<A> {
            if required.contains(required_column) {
                vec![contents; max_column_size]
            } else {
                Vec::new()
            }
        }

        fn sized_simd_always<A: SimdElement>(
            contents: A,
            max_column_size: usize,
        ) -> Vec<Simd<A, N>> {
            let amount = max_column_size.div_ceil(N);
            vec![Simd::splat(contents); amount]
        }

        fn sized_simd<A: SimdElement>(
            contents: A,
            required: EnumSet<StandardDataColumns>,
            max_column_size: usize,
            required_column: StandardDataColumns,
        ) -> Vec<Simd<A, N>> {
            if required.contains(required_column) {
                sized_simd_always(contents, max_column_size)
            } else {
                Vec::new()
            }
        }

        StandardColumns {
            required_columns: required,

            id: vec![0; max_column_size],
            pos_x: sized_simd(0.0, required, max_column_size, StandardDataColumns::PosX),
            pos_y: sized_simd(0.0, required, max_column_size, StandardDataColumns::PosY),
            pos_z: sized_simd(0.0, required, max_column_size, StandardDataColumns::PosZ),
            old_pos_x: sized_simd(0.0, required, max_column_size, StandardDataColumns::PosX),
            old_pos_y: sized_simd(0.0, required, max_column_size, StandardDataColumns::PosY),
            old_pos_z: sized_simd(0.0, required, max_column_size, StandardDataColumns::PosZ),
            scale_x: sized_simd(0.0, required, max_column_size, StandardDataColumns::ScaleX),
            scale_y: sized_simd(0.0, required, max_column_size, StandardDataColumns::ScaleX),
            scale_z: sized_simd(0.0, required, max_column_size, StandardDataColumns::ScaleX),
            old_scale_x: sized_simd(0.0, required, max_column_size, StandardDataColumns::ScaleX),
            old_scale_y: sized_simd(0.0, required, max_column_size, StandardDataColumns::ScaleY),
            old_scale_z: sized_simd(0.0, required, max_column_size, StandardDataColumns::ScaleZ),
            orientation: sized_vec(
                UnitQuaternion::identity(),
                required,
                max_column_size,
                StandardDataColumns::Orientation,
            ),
            old_orientation: sized_vec(
                UnitQuaternion::identity(),
                required,
                max_column_size,
                StandardDataColumns::Orientation,
            ),
            main_color: sized_simd(0, required, max_column_size, StandardDataColumns::MainColor),
            secondary_color: sized_simd(
                0,
                required,
                max_column_size,
                StandardDataColumns::SecondaryColor,
            ),
            old_main_color: sized_simd(
                0,
                required,
                max_column_size,
                StandardDataColumns::MainColor,
            ),
            old_secondary_color: sized_simd(
                0,
                required,
                max_column_size,
                StandardDataColumns::SecondaryColor,
            ),
            damage: sized_simd(0.0, required, max_column_size, StandardDataColumns::Damage),
            form: sized_vec(
                &Form::SPHERE,
                required,
                max_column_size,
                StandardDataColumns::Appearance,
            ),
            render_properties: sized_vec(
                HashMap::new(),
                required,
                max_column_size,
                StandardDataColumns::Appearance,
            ),
            ticks_existed: sized_simd_always(0, max_column_size),
            end_time: sized_simd_always(0, max_column_size),
            dead: vec![false; max_column_size],
            next_stage: vec![Vec::new(); max_column_size],
            next_stage_add_data: vec![EnumSet::EMPTY; max_column_size],
            parent: vec![-1; max_column_size],
            transform_mats: vec![Matrix4::identity(); max_column_size],
            family_depth: vec![0; max_column_size],
            current_dead: Vec::new(),
            add_spawns: Vec::new(),

            // Behavior specific data
            motion_x: sized_simd(0.0, required, max_column_size, StandardDataColumns::MotionX),
            motion_y: sized_simd(0.0, required, max_column_size, StandardDataColumns::MotionY),
            motion_z: sized_simd(0.0, required, max_column_size, StandardDataColumns::MotionZ),
            gravity_x: sized_simd(
                0.0,
                required,
                max_column_size,
                StandardDataColumns::GravityX,
            ),
            gravity_y: sized_simd(
                0.0,
                required,
                max_column_size,
                StandardDataColumns::GravityY,
            ),
            gravity_z: sized_simd(
                0.0,
                required,
                max_column_size,
                StandardDataColumns::GravityZ,
            ),
            speed_accel: sized_simd(
                0.0,
                required,
                max_column_size,
                StandardDataColumns::SpeedAccel,
            ),

            forward_x: sized_simd(1.0, required, max_column_size, StandardDataColumns::Forward),
            forward_y: sized_simd(1.0, required, max_column_size, StandardDataColumns::Forward),
            forward_z: sized_simd(1.0, required, max_column_size, StandardDataColumns::Forward),
            rotation: sized_vec(
                UnitQuaternion::identity(),
                required,
                max_column_size,
                StandardDataColumns::Rotation,
            ),
        }
    }

    fn required_columns(&self) -> EnumSet<StandardDataColumns> {
        self.required_columns
    }

    fn grab_new_spawns(
        &mut self,
    ) -> Vec<(
        DanmakuSpawnData<StandardSpawnData, StandardDataColumns>,
        Option<usize>,
    )> {
        std::mem::take(&mut self.add_spawns)
    }

    fn resize(&mut self, new_max_size: usize) {
        self.id.resize(new_max_size, 0);

        fn resize_if_required<A: Clone>(
            required_columns: EnumSet<StandardDataColumns>,
            new_max_size: usize,
            required_column: StandardDataColumns,
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
            required_columns: EnumSet<StandardDataColumns>,
            new_max_size: usize,
            required_column: StandardDataColumns,
            vec: &mut Vec<Simd<A, N>>,
            new_obj: A,
        ) {
            if required_columns.contains(required_column) {
                resize_simd(new_max_size, vec, new_obj);
            }
        }

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::PosX,
            &mut self.pos_x,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::PosY,
            &mut self.pos_y,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::PosZ,
            &mut self.pos_z,
            0.0,
        );

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::PosX,
            &mut self.old_pos_x,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::PosY,
            &mut self.old_pos_y,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::PosZ,
            &mut self.old_pos_z,
            0.0,
        );

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::ScaleX,
            &mut self.scale_x,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::ScaleY,
            &mut self.scale_y,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::ScaleZ,
            &mut self.scale_z,
            0.0,
        );

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::ScaleX,
            &mut self.scale_x,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::ScaleY,
            &mut self.scale_y,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::ScaleZ,
            &mut self.scale_z,
            0.0,
        );

        resize_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Orientation,
            &mut self.orientation,
            UnitQuaternion::identity(),
        );
        resize_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Orientation,
            &mut self.old_orientation,
            UnitQuaternion::identity(),
        );

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MainColor,
            &mut self.main_color,
            0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MainColor,
            &mut self.old_main_color,
            0,
        );

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::SecondaryColor,
            &mut self.secondary_color,
            0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::SecondaryColor,
            &mut self.old_secondary_color,
            0,
        );

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Damage,
            &mut self.damage,
            0.0,
        );
        resize_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Appearance,
            &mut self.form,
            &Form::SPHERE,
        );
        resize_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Appearance,
            &mut self.render_properties,
            HashMap::new(),
        );

        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MotionX,
            &mut self.motion_x,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MotionY,
            &mut self.motion_y,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MotionZ,
            &mut self.motion_z,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MotionX,
            &mut self.gravity_x,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MotionY,
            &mut self.gravity_y,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::MotionZ,
            &mut self.gravity_z,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::SpeedAccel,
            &mut self.speed_accel,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Forward,
            &mut self.forward_x,
            1.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Forward,
            &mut self.forward_y,
            0.0,
        );
        resize_simd_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Forward,
            &mut self.forward_z,
            0.0,
        );
        resize_if_required(
            self.required_columns,
            new_max_size,
            StandardDataColumns::Rotation,
            &mut self.rotation,
            UnitQuaternion::identity(),
        );

        resize_simd(new_max_size, &mut self.ticks_existed, 0);
        resize_simd(new_max_size, &mut self.end_time, 0);
        self.dead.resize(new_max_size, false);
        self.next_stage.resize(new_max_size, Vec::new());
        self.next_stage_add_data
            .resize(new_max_size, EnumSet::EMPTY);

        self.parent.resize(new_max_size, -1);
        self.transform_mats
            .resize(new_max_size, Matrix4::identity());

        self.family_depth.resize(new_max_size, 0);
    }

    fn compact(&mut self, new_max_size: usize) {
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

        let dead = &self.dead;

        [&mut self.id, &mut self.parent]
            .iter_mut()
            .for_each(|d| compact_vec(d, dead, new_max_size, -1));
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
        .for_each(|d| compact_simd(d, dead, new_max_size, 0.0));

        [
            &mut self.forward_x,
            &mut self.forward_y,
            &mut self.forward_z,
        ]
        .iter_mut()
        .for_each(|d| compact_simd(d, dead, new_max_size, 1.0));

        [
            &mut self.orientation,
            &mut self.old_orientation,
            &mut self.rotation,
        ]
        .iter_mut()
        .for_each(|d| compact_vec(d, dead, new_max_size, UnitQuaternion::identity()));

        [
            &mut self.main_color,
            &mut self.old_main_color,
            &mut self.secondary_color,
            &mut self.old_secondary_color,
        ]
        .iter_mut()
        .for_each(|d| compact_simd(d, dead, new_max_size, 0));

        compact_vec(&mut self.form, dead, new_max_size, &Form::SPHERE);
        compact_vec(
            &mut self.render_properties,
            dead,
            new_max_size,
            HashMap::new(),
        );

        [&mut self.ticks_existed, &mut self.end_time]
            .iter_mut()
            .for_each(|d| compact_simd(d, dead, new_max_size, 0));

        compact_vec(&mut self.family_depth, dead, new_max_size, 0);

        compact_vec(&mut self.next_stage, dead, new_max_size, Vec::new());
        compact_vec(
            &mut self.next_stage_add_data,
            dead,
            new_max_size,
            EnumSet::new(),
        );
        compact_vec(
            &mut self.transform_mats,
            dead,
            new_max_size,
            Matrix4::identity(),
        );

        let _ = &mut self.dead.retain(|d| *d);
        self.dead.resize(new_max_size, false);
        let _ = &mut self.current_dead.clear();
    }

    fn id(&mut self) -> &mut Vec<i128> {
        &mut self.id
    }

    fn dead(&mut self) -> &mut Vec<bool> {
        &mut self.dead
    }

    fn current_dead_len(&self) -> usize {
        self.current_dead.len()
    }

    fn add_danmaku_at_idx(
        &mut self,
        i: usize,
        danmaku: DanmakuSpawnData<StandardSpawnData, StandardDataColumns>,
        id: i128,
    ) -> Vec<DanmakuSpawnData<StandardSpawnData, StandardDataColumns>> {
        fn transfer_data_simd<A: SimdElement>(
            required_columns: EnumSet<StandardDataColumns>,
            i: usize,
            required: StandardDataColumns,
            vec: &mut [Simd<A, N>],
            data: A,
        ) {
            if required_columns.contains(required) {
                vec[i.div_ceil(N)][i % N] = data;
            }
        }

        fn transfer_data<A>(
            required_columns: EnumSet<StandardDataColumns>,
            i: usize,
            required: StandardDataColumns,
            vec: &mut [A],
            data: A,
        ) {
            if required_columns.contains(required) {
                vec[i] = data;
            }
        }

        self.id[i] = id;

        let render_properties = danmaku.render_properties;

        for d in danmaku.behavior_data {
            match d {
                StandardSpawnData::PosX(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::PosX,
                        &mut self.pos_x,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::PosX,
                        &mut self.old_pos_x,
                        v,
                    );
                }
                StandardSpawnData::PosY(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::PosY,
                        &mut self.pos_y,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::PosY,
                        &mut self.old_pos_y,
                        v,
                    );
                }
                StandardSpawnData::PosZ(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::PosZ,
                        &mut self.pos_z,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::PosZ,
                        &mut self.old_pos_z,
                        v,
                    );
                }
                StandardSpawnData::Orientation(v) => {
                    transfer_data(
                        self.required_columns,
                        i,
                        StandardDataColumns::Orientation,
                        &mut self.orientation,
                        v,
                    );
                    transfer_data(
                        self.required_columns,
                        i,
                        StandardDataColumns::Orientation,
                        &mut self.old_orientation,
                        v,
                    );
                }
                StandardSpawnData::Appearance { form } => {
                    transfer_data(
                        self.required_columns,
                        i,
                        StandardDataColumns::Appearance,
                        &mut self.form,
                        form,
                    );
                    transfer_data(
                        self.required_columns,
                        i,
                        StandardDataColumns::Appearance,
                        &mut self.render_properties,
                        render_properties.clone(),
                    );
                }
                StandardSpawnData::MainColor(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::MainColor,
                        &mut self.main_color,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::MainColor,
                        &mut self.old_main_color,
                        v,
                    );
                }
                StandardSpawnData::SecondaryColor(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::SecondaryColor,
                        &mut self.secondary_color,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::SecondaryColor,
                        &mut self.old_secondary_color,
                        v,
                    );
                }
                StandardSpawnData::Damage(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::Damage,
                        &mut self.damage,
                        v,
                    );
                }
                StandardSpawnData::SizeX(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::ScaleX,
                        &mut self.scale_x,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::ScaleX,
                        &mut self.old_scale_x,
                        v,
                    );
                }
                StandardSpawnData::SizeY(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::ScaleY,
                        &mut self.scale_y,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::ScaleY,
                        &mut self.old_scale_y,
                        v,
                    );
                }
                StandardSpawnData::SizeZ(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::ScaleZ,
                        &mut self.scale_z,
                        v,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::ScaleZ,
                        &mut self.old_scale_z,
                        v,
                    );
                }
                StandardSpawnData::MotionX(v) => transfer_data_simd(
                    self.required_columns,
                    i,
                    StandardDataColumns::MotionX,
                    &mut self.motion_x,
                    v,
                ),
                StandardSpawnData::MotionY(v) => transfer_data_simd(
                    self.required_columns,
                    i,
                    StandardDataColumns::MotionY,
                    &mut self.motion_x,
                    v,
                ),
                StandardSpawnData::MotionZ(v) => transfer_data_simd(
                    self.required_columns,
                    i,
                    StandardDataColumns::MotionZ,
                    &mut self.motion_x,
                    v,
                ),
                StandardSpawnData::GravityX(v) => transfer_data_simd(
                    self.required_columns,
                    i,
                    StandardDataColumns::GravityX,
                    &mut self.motion_x,
                    v,
                ),
                StandardSpawnData::GravityY(v) => transfer_data_simd(
                    self.required_columns,
                    i,
                    StandardDataColumns::GravityY,
                    &mut self.motion_x,
                    v,
                ),
                StandardSpawnData::GravityZ(v) => transfer_data_simd(
                    self.required_columns,
                    i,
                    StandardDataColumns::GravityZ,
                    &mut self.motion_x,
                    v,
                ),
                StandardSpawnData::SpeedAccel(v) => transfer_data_simd(
                    self.required_columns,
                    i,
                    StandardDataColumns::SpeedAccel,
                    &mut self.speed_accel,
                    v,
                ),
                StandardSpawnData::Forward(v) => {
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::Forward,
                        &mut self.forward_x,
                        v.x,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::Forward,
                        &mut self.forward_z,
                        v.y,
                    );
                    transfer_data_simd(
                        self.required_columns,
                        i,
                        StandardDataColumns::Forward,
                        &mut self.forward_z,
                        v.z,
                    );
                }
                StandardSpawnData::Rotation(v) => transfer_data(
                    self.required_columns,
                    i,
                    StandardDataColumns::Rotation,
                    &mut self.rotation,
                    v,
                ),
            }
        }

        self.ticks_existed[i.div_ceil(N)][i % N] = 0;
        self.end_time[i.div_ceil(N)][i % N] = danmaku.end_time;
        self.dead[i] = false;
        self.next_stage[i] = danmaku.next_stage;
        self.next_stage_add_data[i] = danmaku.next_stage_add_data;
        self.parent[i] = danmaku.parent.unwrap_or(-1);
        self.family_depth[i] = danmaku.family_depth;

        self.transform_mats[i].fill_with_identity();

        danmaku.children
    }

    fn compute_transform_mats(&mut self, current_size: usize, partial_ticks: f32) {
        let required_main_columns = self.required_columns;

        #[inline]
        fn lerp_if_used(
            partial_ticks: f32,
            used: bool,
            i: usize,
            old: &[Simd<f32, N>],
            new: &[Simd<f32, N>],
        ) -> f32 {
            if used {
                nalgebra_glm::lerp_scalar(
                    old[i.div_ceil(N)][i % N],
                    new[i.div_ceil(N)][i % N],
                    partial_ticks,
                )
            } else {
                0.0
            }
        }

        if required_main_columns.contains(StandardDataColumns::Appearance) {
            let requires_scale_x = required_main_columns.contains(StandardDataColumns::ScaleX);
            let requires_scale_y = required_main_columns.contains(StandardDataColumns::ScaleY);
            let requires_scale_z = required_main_columns.contains(StandardDataColumns::ScaleZ);
            let requires_pos_x = required_main_columns.contains(StandardDataColumns::PosX);
            let requires_pos_y = required_main_columns.contains(StandardDataColumns::PosY);
            let requires_pos_z = required_main_columns.contains(StandardDataColumns::PosZ);
            let requires_orientation =
                required_main_columns.contains(StandardDataColumns::Orientation);

            let mut temp = Matrix4::identity();

            let pos_x = &self.pos_x;
            let pos_y = &self.pos_y;
            let pos_z = &self.pos_z;
            let old_pos_x = &self.old_pos_x;
            let old_pos_y = &self.old_pos_y;
            let old_pos_z = &self.old_pos_z;

            let scale_x = &self.scale_x;
            let scale_y = &self.scale_y;
            let scale_z = &self.scale_z;
            let old_scale_x = &self.old_scale_x;
            let old_scale_y = &self.old_scale_y;
            let old_scale_z = &self.old_scale_z;

            let orientation = &self.orientation;
            let old_orientation = &self.old_orientation;

            let dead = &self.dead;

            for i in 0..current_size {
                if !dead[i] {
                    temp.fill_with_identity();

                    temp.append_nonuniform_scaling_mut(&Vector3::new(
                        lerp_if_used(partial_ticks, requires_scale_x, i, old_scale_x, scale_x),
                        lerp_if_used(partial_ticks, requires_scale_y, i, old_scale_y, scale_y),
                        lerp_if_used(partial_ticks, requires_scale_z, i, old_scale_z, scale_z),
                    ));

                    if requires_pos_x || requires_pos_y || requires_pos_z {
                        temp.append_translation_mut(&Vector3::new(
                            lerp_if_used(partial_ticks, requires_pos_x, i, old_pos_x, pos_x),
                            lerp_if_used(partial_ticks, requires_pos_y, i, old_pos_y, pos_y),
                            lerp_if_used(partial_ticks, requires_pos_z, i, old_pos_z, pos_z),
                        ));
                    }

                    let orientation_mat = if requires_orientation {
                        old_orientation
                            .get(i)
                            .unwrap_or(&UnitQuaternion::identity())
                            .slerp(
                                orientation.get(i).unwrap_or(&UnitQuaternion::identity()),
                                partial_ticks,
                            )
                            .to_homogeneous()
                    } else {
                        orientation
                            .get(i)
                            .unwrap_or(&UnitQuaternion::identity())
                            .to_homogeneous()
                    };

                    self.transform_mats[i] = orientation_mat * temp;
                }
            }
        }
    }

    fn compute_and_get_render_data(
        &mut self,
        current_size: usize,
        partial_ticks: f32,
    ) -> Vec<(i128, RenderData)> {
        self.compute_transform_mats(current_size, partial_ticks);

        let form = &self.form;
        let render_properties = &self.render_properties;
        let transform_mats = &self.transform_mats;
        let main_color = &self.main_color;
        let old_main_color = &self.old_main_color;
        let secondary_color = &self.secondary_color;
        let old_secondary_color = &self.old_secondary_color;
        let ticks_existed = &self.ticks_existed;
        let end_time = &self.end_time;
        let dead = &self.dead;
        let id = &self.id;

        let has_main_color = self
            .required_columns
            .contains(StandardDataColumns::MainColor);
        let has_secondary_color = self
            .required_columns
            .contains(StandardDataColumns::SecondaryColor);

        if self
            .required_columns
            .contains(StandardDataColumns::Appearance)
        {
            (0..current_size)
                .filter(|i| !dead.get(*i).unwrap_or(&false))
                .map(|i| (id.get(i).unwrap_or(&0), i))
                .map(|(id, i)| {
                    let lerp_color = |has_color: bool,
                                      new: &Vec<Simd<i32, N>>,
                                      old: &Vec<Simd<i32, N>>|
                     -> ColorHex {
                        if has_color {
                            ColorHex(new[i.div_ceil(N)][i % N]).lerp_through_hsv(
                                ColorHex(old[i.div_ceil(N)][i % N]),
                                partial_ticks,
                            )
                        } else {
                            ColorHex(0)
                        }
                    };

                    let main_color = lerp_color(has_main_color, main_color, old_main_color);
                    let secondary_color =
                        lerp_color(has_secondary_color, secondary_color, old_secondary_color);

                    (
                        *id,
                        RenderData {
                            form: form.get(i).unwrap(),
                            render_properties: render_properties.get(i).unwrap(),
                            model_mat: *transform_mats.get(i).unwrap_or(&Matrix4::identity()),
                            main_color: main_color.0,
                            secondary_color: secondary_color.0,
                            ticks_existed: ticks_existed[i.div_ceil(N)][i & N],
                            end_time: end_time[i.div_ceil(N)][i & N],
                        },
                    )
                })
                .collect()
        } else {
            vec![]
        }
    }
}

#[derive(Clone, Debug)]
pub enum StandardSpawnData {
    PosX(f32),
    PosY(f32),
    PosZ(f32),
    Orientation(UnitQuaternion<f32>),
    Appearance { form: &'static Form },
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
    Rotation(UnitQuaternion<f32>),
}

#[derive(Debug, Hash, EnumSetType)]
pub enum StandardDataColumns {
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
