use crate::danmaku::{
    handlers::TopDanmakuBehaviorsHandler,
    standard::{StandardColumns, StandardDataColumns, StandardSpawnData},
    Behavior, N,
};

use enumset::EnumSet;
use multiversion::multiversion;
use nalgebra::{UnitVector3, Vector3};
use std::simd::{cmp::SimdPartialOrd, num::SimdInt, Simd};

pub const MOTION1_BEHAVIOR_ID: &str = "motion1";
pub fn motion1_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let motion_z = &mut columns.motion_z[0..size.div_ceil(N)];
        let pos_z = &mut columns.pos_z[0..size.div_ceil(N)];
        let old_pos_z = &mut columns.old_pos_z[0..size.div_ceil(N)];

        old_pos_z[0..size.div_ceil(N)].copy_from_slice(&pos_z[0..size.div_ceil(N)]);

        for i in 0..size {
            pos_z[i] += motion_z[i]
        }
    }

    Behavior {
        identifier: MOTION1_BEHAVIOR_ID,
        required_columns: StandardDataColumns::PosZ | StandardDataColumns::MotionZ,
        act,
    }
}

pub const GRAVITY1_BEHAVIOR_ID: &str = "gravity1";
pub fn gravity1_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let ticks_existed = &columns.ticks_existed[0..size.div_ceil(N)];
        let mot = &mut columns.motion_y[0..size.div_ceil(N)];
        let gravity = &mut columns.gravity_y[0..size.div_ceil(N)];

        for i in 0..size {
            mot[i] += gravity[i] * ticks_existed[i].cast::<f32>();
        }
    }

    Behavior {
        identifier: GRAVITY1_BEHAVIOR_ID,
        required_columns: StandardDataColumns::MotionY | StandardDataColumns::GravityY,
        act,
    }
}

pub const ACCELERATION1_BEHAVIOR_ID: &str = "acceleration1";
pub fn acceleration1_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let speed_accel = &mut columns.speed_accel[0..size.div_ceil(N)];
        let motion = &mut columns.motion_z[0..size.div_ceil(N)];

        for i in 0..size.div_ceil(N) {
            motion[i] += speed_accel[i];
        }
    }

    Behavior {
        identifier: ACCELERATION1_BEHAVIOR_ID,
        required_columns: StandardDataColumns::MotionZ | StandardDataColumns::SpeedAccel,
        act,
    }
}

pub const ROTATE_ORIENTATION_BEHAVIOR_ID: &str = "rotate_orientation";
pub fn rotate_orientation_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let orientation = &mut columns.orientation[0..size];
        let old_orientation = &mut columns.old_orientation[0..size];
        let rotation = &mut columns.rotation[0..size];

        for i in 0..size {
            old_orientation[0] = orientation[i];
            orientation[i] *= rotation[i];
        }
    }

    Behavior {
        identifier: ROTATE_ORIENTATION_BEHAVIOR_ID,
        required_columns: StandardDataColumns::Rotation | StandardDataColumns::Orientation,
        act,
    }
}

pub const ROTATE_FORWARD_BEHAVIOR_ID: &str = "rotate_forward";
pub fn rotate_forward_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let forward_x = &mut columns.forward_x[0..size.div_ceil(N)];
        let forward_y = &mut columns.forward_y[0..size.div_ceil(N)];
        let forward_z = &mut columns.forward_z[0..size.div_ceil(N)];

        let rotation = &mut columns.rotation[0..size];

        for i in 0..size.div_ceil(N) {
            for j in 0..N {
                let forward = UnitVector3::new_normalize(Vector3::new(
                    forward_x[i][j],
                    forward_y[i][j],
                    forward_z[i][j],
                ));
                let new_forward = rotation[i * N + j] * forward;
                forward_x[i][j] = new_forward.x;
                forward_y[i][j] = new_forward.y;
                forward_z[i][j] = new_forward.z;
            }
        }
    }

    Behavior {
        identifier: ROTATE_FORWARD_BEHAVIOR_ID,
        required_columns: StandardDataColumns::Rotation | StandardDataColumns::Forward,
        act,
    }
}

pub const MOTION3_BEHAVIOR_ID: &str = "motion3";
pub fn motion3_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let motion_x = &mut columns.motion_x[0..size.div_ceil(N)];
        let motion_y = &mut columns.motion_y[0..size.div_ceil(N)];
        let motion_z = &mut columns.motion_z[0..size.div_ceil(N)];
        let pos_x = &mut columns.pos_x[0..size.div_ceil(N)];
        let pos_y = &mut columns.pos_y[0..size.div_ceil(N)];
        let pos_z = &mut columns.pos_z[0..size.div_ceil(N)];
        let old_pos_x = &mut columns.old_pos_x[0..size.div_ceil(N)];
        let old_pos_y = &mut columns.old_pos_y[0..size.div_ceil(N)];
        let old_pos_z = &mut columns.old_pos_z[0..size.div_ceil(N)];

        old_pos_x[0..size].copy_from_slice(&pos_x[0..size.div_ceil(N)]);
        old_pos_y[0..size].copy_from_slice(&pos_y[0..size.div_ceil(N)]);
        old_pos_z[0..size].copy_from_slice(&pos_z[0..size.div_ceil(N)]);

        for i in 0..size.div_ceil(N) {
            pos_x[i] += motion_x[i]
        }

        for i in 0..size.div_ceil(N) {
            pos_y[i] += motion_y[i]
        }

        for i in 0..size.div_ceil(N) {
            pos_z[i] += motion_z[i]
        }
    }

    Behavior {
        identifier: MOTION3_BEHAVIOR_ID,
        required_columns: StandardDataColumns::PosX
            | StandardDataColumns::PosY
            | StandardDataColumns::PosZ
            | StandardDataColumns::MotionX
            | StandardDataColumns::MotionY
            | StandardDataColumns::MotionZ,
        act,
    }
}

pub const GRAVITY3_BEHAVIOR_ID: &str = "gravity3";
pub fn gravity3_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let ticks_existed = &columns.ticks_existed[0..size.div_ceil(N)];

        let motion_x = &mut columns.motion_x[0..size.div_ceil(N)];
        let motion_y = &mut columns.motion_y[0..size.div_ceil(N)];
        let motion_z = &mut columns.motion_z[0..size.div_ceil(N)];
        let gravity_x = &mut columns.gravity_x[0..size.div_ceil(N)];
        let gravity_y = &mut columns.gravity_y[0..size.div_ceil(N)];
        let gravity_z = &mut columns.gravity_z[0..size.div_ceil(N)];

        for i in 0..size.div_ceil(N) {
            motion_x[i] += gravity_x[i] * ticks_existed[i].cast::<f32>();
        }

        for i in 0..size.div_ceil(N) {
            motion_y[i] += gravity_y[i] * ticks_existed[i].cast::<f32>();
        }

        for i in 0..size.div_ceil(N) {
            motion_z[i] += gravity_z[i] * ticks_existed[i].cast::<f32>();
        }
    }

    Behavior {
        identifier: GRAVITY3_BEHAVIOR_ID,
        required_columns: StandardDataColumns::MotionY | StandardDataColumns::GravityY,
        act,
    }
}

pub const ACCELERATION3_BEHAVIOR_ID: &str = "acceleration3";
pub fn acceleration3_behavior() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let speed_accel = &mut columns.speed_accel[0..size.div_ceil(N)];

        let forward_x = &mut columns.forward_x[0..size.div_ceil(N)];
        let forward_y = &mut columns.forward_y[0..size.div_ceil(N)];
        let forward_z = &mut columns.forward_z[0..size.div_ceil(N)];
        let motion_x = &mut columns.motion_x[0..size.div_ceil(N)];
        let motion_y = &mut columns.motion_y[0..size.div_ceil(N)];
        let motion_z = &mut columns.motion_z[0..size.div_ceil(N)];

        for i in 0..size.div_ceil(N) {
            motion_x[i] += forward_x[i] * speed_accel[i];
        }

        for i in 0..size.div_ceil(N) {
            motion_y[i] += forward_y[i] * speed_accel[i];
        }

        for i in 0..size.div_ceil(N) {
            motion_z[i] += forward_z[i] * speed_accel[i];
        }
    }

    Behavior {
        identifier: ACCELERATION3_BEHAVIOR_ID,
        required_columns: StandardDataColumns::SpeedAccel
            | StandardDataColumns::MotionX
            | StandardDataColumns::MotionY
            | StandardDataColumns::MotionZ
            | StandardDataColumns::Forward,
        act,
    }
}

pub const MANDATORY_END_BEHAVIOR_ID: &str = "mandatory_end";
pub fn mandatory_end() -> Behavior<StandardColumns> {
    #[multiversion(targets = "simd")]
    fn act(columns: &mut StandardColumns, size: usize) {
        let ticks_existed = &mut columns.ticks_existed[0..size.div_ceil(N)];
        let end_time = &mut columns.end_time[0..size.div_ceil(N)];
        let next_stage = &mut columns.next_stage[0..size];
        let next_stage_add_data = &mut columns.next_stage_add_data[0..size];
        let dead = &mut columns.dead[0..size];

        let pos_x = &mut columns.pos_x;
        let pos_y = &mut columns.pos_y;
        let pos_z = &mut columns.pos_z;

        let scale_x = &mut columns.scale_x;
        let scale_y = &mut columns.scale_y;
        let scale_z = &mut columns.scale_z;

        let motion_x = &mut columns.motion_x;
        let motion_y = &mut columns.motion_y;
        let motion_z = &mut columns.motion_z;

        let forward_x = &mut columns.forward_x;
        let forward_y = &mut columns.forward_y;
        let forward_z = &mut columns.forward_z;

        let gravity_x = &mut columns.gravity_x;
        let gravity_y = &mut columns.gravity_y;
        let gravity_z = &mut columns.gravity_z;

        let damage = &mut columns.damage;

        let orientation = &mut columns.orientation;
        let rotation = &mut columns.rotation;

        let speed_accel = &mut columns.speed_accel;

        let main_color = &mut columns.main_color;
        let secondary_color = &mut columns.secondary_color;

        let add_spawns = &mut columns.add_spawns;

        for i in 0..size.div_ceil(N) {
            ticks_existed[i] += Simd::splat(1);
        }

        for i in 0..size.div_ceil(N) {
            let this_dead = ticks_existed[i].simd_gt(end_time[i]).to_array();

            for j in 0..N {
                let idx = i * N + j;
                let add_data = next_stage_add_data[idx];

                let value_or_simd = |vec: &Vec<Simd<f32, N>>, required| {
                    if (columns.required_columns & add_data).contains(required) {
                        vec[i][j]
                    } else {
                        0.0
                    }
                };
                let is_dead = this_dead[j];

                if is_dead && !columns.current_dead.contains(&idx) {
                    columns.current_dead.push(idx);
                    let mut next_stages = std::mem::take(&mut next_stage[idx]);
                    next_stages.iter_mut().for_each(|next| {
                        next.behavior_data.iter_mut().for_each(|data| match data {
                            StandardSpawnData::PosX(ref mut v) => {
                                *v += value_or_simd(pos_x, StandardDataColumns::PosY)
                            }
                            StandardSpawnData::PosY(ref mut v) => {
                                *v += value_or_simd(pos_y, StandardDataColumns::PosY)
                            }
                            StandardSpawnData::PosZ(ref mut v) => {
                                *v += value_or_simd(pos_z, StandardDataColumns::PosY)
                            }
                            StandardSpawnData::Orientation(ref mut v) => {
                                if columns
                                    .required_columns
                                    .contains(StandardDataColumns::Orientation)
                                {
                                    *v = orientation[idx] * *v
                                }
                            }
                            StandardSpawnData::Appearance { .. } => {}
                            StandardSpawnData::MainColor(ref mut v) => {
                                if columns
                                    .required_columns
                                    .contains(StandardDataColumns::MainColor)
                                {
                                    *v = main_color[i][j]
                                }
                            }
                            StandardSpawnData::SecondaryColor(ref mut v) => {
                                if columns
                                    .required_columns
                                    .contains(StandardDataColumns::SecondaryColor)
                                {
                                    *v = secondary_color[i][j]
                                }
                            }
                            StandardSpawnData::Damage(ref mut v) => {
                                *v += value_or_simd(damage, StandardDataColumns::Damage)
                            }
                            StandardSpawnData::SizeX(ref mut v) => {
                                *v += value_or_simd(scale_x, StandardDataColumns::ScaleX)
                            }
                            StandardSpawnData::SizeY(ref mut v) => {
                                *v += value_or_simd(scale_y, StandardDataColumns::ScaleY)
                            }
                            StandardSpawnData::SizeZ(ref mut v) => {
                                *v += value_or_simd(scale_z, StandardDataColumns::ScaleZ)
                            }
                            StandardSpawnData::MotionX(ref mut v) => {
                                *v += value_or_simd(motion_x, StandardDataColumns::MotionX)
                            }
                            StandardSpawnData::MotionY(ref mut v) => {
                                *v += value_or_simd(motion_y, StandardDataColumns::MotionY)
                            }
                            StandardSpawnData::MotionZ(ref mut v) => {
                                *v += value_or_simd(motion_z, StandardDataColumns::MotionZ)
                            }
                            StandardSpawnData::GravityX(ref mut v) => {
                                *v += value_or_simd(gravity_x, StandardDataColumns::GravityX)
                            }
                            StandardSpawnData::GravityY(ref mut v) => {
                                *v += value_or_simd(gravity_y, StandardDataColumns::GravityY)
                            }
                            StandardSpawnData::GravityZ(ref mut v) => {
                                *v += value_or_simd(gravity_z, StandardDataColumns::GravityZ)
                            }
                            StandardSpawnData::SpeedAccel(ref mut v) => {
                                *v += value_or_simd(speed_accel, StandardDataColumns::SpeedAccel)
                            }
                            StandardSpawnData::Forward(ref mut v) => {
                                if columns
                                    .required_columns
                                    .contains(StandardDataColumns::Forward)
                                {
                                    *v = UnitVector3::new_normalize(Vector3::new(
                                        forward_x[i][j],
                                        forward_y[i][j],
                                        forward_z[i][j],
                                    ))
                                }
                            }
                            StandardSpawnData::Rotation(ref mut v) => {
                                if columns
                                    .required_columns
                                    .contains(StandardDataColumns::Orientation)
                                {
                                    *v = rotation[idx] * *v
                                }
                            }
                        })
                    });

                    if next_stages.len() == 1 {
                        add_spawns
                            .append(&mut next_stages.into_iter().map(|d| (d, Some(idx))).collect());
                    } else {
                        add_spawns
                            .append(&mut next_stages.into_iter().map(|d| (d, None)).collect());
                    }
                }

                dead[idx] = dead[idx] || is_dead
            }
        }
    }

    Behavior {
        identifier: MANDATORY_END_BEHAVIOR_ID,
        required_columns: EnumSet::EMPTY,
        act,
    }
}

pub trait StandardTopHandlerExt {
    fn register_standard_behaviors(&mut self);
}

impl StandardTopHandlerExt for TopDanmakuBehaviorsHandler<StandardColumns> {
    fn register_standard_behaviors(&mut self) {
        self.register_behavior(motion1_behavior());
        self.register_behavior(gravity1_behavior());
        self.register_behavior(acceleration1_behavior());
        self.register_behavior(rotate_orientation_behavior());
        self.register_behavior(rotate_forward_behavior());
        self.register_behavior(motion3_behavior());
        self.register_behavior(gravity3_behavior());
        self.register_behavior(acceleration3_behavior());
        self.register_behavior(mandatory_end());
    }
}
