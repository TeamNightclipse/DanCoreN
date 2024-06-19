use crate::behavior::danmaku_data::BehaviorData;
use crate::behavior::main_columns::DataColumns;
use crate::behavior::Behavior;
use enumset::EnumSet;
use nalgebra::{UnitVector3, Vector3};

pub const MOTION1_BEHAVIOR_ID: &str = "motion1";
pub fn motion1_behavior() -> Behavior {
    Behavior {
        identifier: MOTION1_BEHAVIOR_ID,
        required_columns: DataColumns::PosZ | DataColumns::MotionZ,
        act: |columns, size| {
            let motion_z = &mut columns.motion_z[0..size];
            let pos_z = &mut columns.pos_z[0..size];
            let old_pos_z = &mut columns.old_pos_z[0..size];

            old_pos_z[0..size].copy_from_slice(&pos_z[0..size]);

            for i in 0..size {
                pos_z[i] += motion_z[i]
            }
        },
    }
}

pub const GRAVITY1_BEHAVIOR_ID: &str = "gravity1";
pub fn gravity1_behavior() -> Behavior {
    Behavior {
        identifier: GRAVITY1_BEHAVIOR_ID,
        required_columns: DataColumns::MotionY | DataColumns::GravityY,
        act: |columns, size| {
            let ticks_existed = &columns.ticks_existed[0..size];
            let mot = &mut columns.motion_y[0..size];
            let gravity = &mut columns.gravity_y[0..size];

            for i in 0..size {
                mot[i] = mot[i] + gravity[i] * ticks_existed[i] as f32;
            }
        },
    }
}

pub const ACCELERATION1_BEHAVIOR_ID: &str = "acceleration1";
pub fn acceleration1_behavior() -> Behavior {
    Behavior {
        identifier: ACCELERATION1_BEHAVIOR_ID,
        required_columns: DataColumns::MotionZ | DataColumns::SpeedAccel,
        act: |columns, size| {
            let speed_accel = &mut columns.speed_accel[0..size];
            let motion = &mut columns.motion_z[0..size];

            for i in 0..size {
                motion[i] = motion[i] + speed_accel[i];
            }
        },
    }
}

pub const ROTATE_ORIENTATION_BEHAVIOR_ID: &str = "rotate_orientation";
pub fn rotate_orientation_behavior() -> Behavior {
    Behavior {
        identifier: ROTATE_ORIENTATION_BEHAVIOR_ID,
        required_columns: DataColumns::Rotation | DataColumns::Orientation,
        act: |columns, size| {
            let orientation = &mut columns.orientation[0..size];
            let old_orientation = &mut columns.old_orientation[0..size];
            let rotation = &mut columns.rotation[0..size];

            for i in 0..size {
                old_orientation[0] = orientation[i];
                orientation[i] *= rotation[i];
            }
        },
    }
}

pub const ROTATE_FORWARD_BEHAVIOR_ID: &str = "rotate_forward";
pub fn rotate_forward_behavior() -> Behavior {
    Behavior {
        identifier: ROTATE_FORWARD_BEHAVIOR_ID,
        required_columns: DataColumns::Rotation | DataColumns::Forward,
        act: |columns, size| {
            let forward_x = &mut columns.forward_x[0..size];
            let forward_y = &mut columns.forward_y[0..size];
            let forward_z = &mut columns.forward_z[0..size];

            let rotation = &mut columns.rotation[0..size];

            for i in 0..size {
                let forward = UnitVector3::new_normalize(Vector3::new(
                    forward_x[i],
                    forward_y[i],
                    forward_z[i],
                ));
                let new_forward = rotation[i] * forward;
                forward_x[i] = new_forward.x;
                forward_y[i] = new_forward.y;
                forward_z[i] = new_forward.z;
            }
        },
    }
}

pub const MOTION3_BEHAVIOR_ID: &str = "motion3";
pub fn motion3_behavior() -> Behavior {
    Behavior {
        identifier: MOTION3_BEHAVIOR_ID,
        required_columns: DataColumns::PosX
            | DataColumns::PosY
            | DataColumns::PosZ
            | DataColumns::MotionX
            | DataColumns::MotionY
            | DataColumns::MotionZ,
        act: |columns, size| {
            let motion_x = &mut columns.motion_x[0..size];
            let motion_y = &mut columns.motion_y[0..size];
            let motion_z = &mut columns.motion_z[0..size];
            let pos_x = &mut columns.pos_x[0..size];
            let pos_y = &mut columns.pos_y[0..size];
            let pos_z = &mut columns.pos_z[0..size];
            let old_pos_x = &mut columns.old_pos_x[0..size];
            let old_pos_y = &mut columns.old_pos_y[0..size];
            let old_pos_z = &mut columns.old_pos_z[0..size];

            old_pos_x[0..size].copy_from_slice(&pos_x[0..size]);
            old_pos_y[0..size].copy_from_slice(&pos_y[0..size]);
            old_pos_z[0..size].copy_from_slice(&pos_z[0..size]);

            for i in 0..size {
                pos_x[i] += motion_x[i]
            }

            for i in 0..size {
                pos_y[i] += motion_y[i]
            }

            for i in 0..size {
                pos_z[i] += motion_z[i]
            }
        },
    }
}

pub const GRAVITY3_BEHAVIOR_ID: &str = "gravity3";
pub fn gravity3_behavior() -> Behavior {
    Behavior {
        identifier: GRAVITY3_BEHAVIOR_ID,
        required_columns: DataColumns::MotionY | DataColumns::GravityY,
        act: |columns, size| {
            let ticks_existed = &columns.ticks_existed[0..size];

            let motion_x = &mut columns.motion_x[0..size];
            let motion_y = &mut columns.motion_y[0..size];
            let motion_z = &mut columns.motion_z[0..size];
            let gravity_x = &mut columns.gravity_x[0..size];
            let gravity_y = &mut columns.gravity_y[0..size];
            let gravity_z = &mut columns.gravity_z[0..size];

            for i in 0..size {
                motion_x[i] += gravity_x[i] * ticks_existed[i] as f32;
            }

            for i in 0..size {
                motion_y[i] += gravity_y[i] * ticks_existed[i] as f32;
            }

            for i in 0..size {
                motion_z[i] += gravity_z[i] * ticks_existed[i] as f32;
            }
        },
    }
}

pub const ACCELERATION3_BEHAVIOR_ID: &str = "acceleration3";
pub fn acceleration3_behavior() -> Behavior {
    Behavior {
        identifier: ACCELERATION3_BEHAVIOR_ID,
        required_columns: DataColumns::SpeedAccel
            | DataColumns::MotionX
            | DataColumns::MotionY
            | DataColumns::MotionZ
            | DataColumns::Forward,
        act: |columns, size| {
            let speed_accel = &mut columns.speed_accel[0..size];

            let forward_x = &mut columns.forward_x[0..size];
            let forward_y = &mut columns.forward_y[0..size];
            let forward_z = &mut columns.forward_z[0..size];
            let motion_x = &mut columns.motion_x[0..size];
            let motion_y = &mut columns.motion_y[0..size];
            let motion_z = &mut columns.motion_z[0..size];

            for i in 0..size {
                motion_x[i] += forward_x[i] * speed_accel[i];
            }

            for i in 0..size {
                motion_y[i] += forward_y[i] * speed_accel[i];
            }

            for i in 0..size {
                motion_z[i] += forward_z[i] * speed_accel[i];
            }
        },
    }
}

pub const MANDATORY_END_BEHAVIOR_ID: &str = "mandatory_end";
pub fn mandatory_end() -> Behavior {
    Behavior {
        identifier: MANDATORY_END_BEHAVIOR_ID,
        required_columns: EnumSet::EMPTY,
        act: |columns, size| {
            let ticks_existed = &mut columns.ticks_existed[0..size];
            let end_time = &mut columns.end_time[0..size];
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

            let orientation = &mut columns.orientation;
            let rotation = &mut columns.rotation;
            
            let speed_accel = &mut columns.speed_accel;

            let main_color = &mut columns.main_color;
            let secondary_color = &mut columns.secondary_color;

            let add_spawns = &mut columns.add_spawns;

            for i in 0..size {
                ticks_existed[i] += 1;
            }

            for i in 0..size {
                let add_data = next_stage_add_data[i];

                let value_or = |vec: &mut Vec<f32>, required| {
                    if (columns.required_columns & add_data).contains(required) {
                        vec[i]
                    } else {
                        0.0
                    }
                };

                let this_dead = ticks_existed[i] > end_time[i];
                if this_dead && !columns.current_dead.contains(&i) {
                    columns.current_dead.push(i);
                    let mut next_stages = std::mem::take(&mut next_stage[i]);
                    next_stages.iter_mut().for_each(|next| {
                        next.behavior_data.iter_mut().for_each(|data| match data {
                            BehaviorData::PosX(ref mut v) => {
                                *v += value_or(pos_x, DataColumns::PosY)
                            }
                            BehaviorData::PosY(ref mut v) => {
                                *v += value_or(pos_y, DataColumns::PosY)
                            }
                            BehaviorData::PosZ(ref mut v) => {
                                *v += value_or(pos_z, DataColumns::PosY)
                            }
                            BehaviorData::Orientation(ref mut v) => {
                                if columns.required_columns.contains(DataColumns::Orientation) {
                                    *v = orientation[i] * *v
                                }
                            }
                            BehaviorData::Appearance { .. } => {}
                            BehaviorData::MainColor(ref mut v) => {
                                if columns.required_columns.contains(DataColumns::MainColor) {
                                    *v = main_color[i]
                                }
                            }
                            BehaviorData::SecondaryColor(ref mut v) => {
                                if columns.required_columns.contains(DataColumns::SecondaryColor) {
                                    *v = secondary_color[i]
                                }
                            }
                            BehaviorData::Damage(ref mut v) => {
                                *v += value_or(motion_z, DataColumns::Damage)
                            }
                            BehaviorData::SizeX(ref mut v) => {
                                *v += value_or(scale_x, DataColumns::ScaleX)
                            }
                            BehaviorData::SizeY(ref mut v) => {
                                *v += value_or(scale_y, DataColumns::ScaleY)
                            }
                            BehaviorData::SizeZ(ref mut v) => {
                                *v += value_or(scale_z, DataColumns::ScaleZ)
                            }
                            BehaviorData::MotionX(ref mut v) => {
                                *v += value_or(motion_x, DataColumns::MotionX)
                            }
                            BehaviorData::MotionY(ref mut v) => {
                                *v += value_or(motion_y, DataColumns::MotionY)
                            }
                            BehaviorData::MotionZ(ref mut v) => {
                                *v += value_or(motion_z, DataColumns::MotionZ)
                            }
                            BehaviorData::GravityX(ref mut v) => {
                                *v += value_or(gravity_x, DataColumns::GravityX)
                            }
                            BehaviorData::GravityY(ref mut v) => {
                                *v += value_or(gravity_y, DataColumns::GravityY)
                            }
                            BehaviorData::GravityZ(ref mut v) => {
                                *v += value_or(gravity_z, DataColumns::GravityZ)
                            }
                            BehaviorData::SpeedAccel(ref mut v) => {
                                *v += value_or(speed_accel, DataColumns::SpeedAccel)
                            }
                            BehaviorData::Forward(ref mut v) => {
                                if columns.required_columns.contains(DataColumns::Forward) {
                                    *v = UnitVector3::new_normalize(Vector3::new(
                                        forward_x[i],
                                        forward_y[i],
                                        forward_z[i],
                                    ))
                                }
                            }
                            BehaviorData::Rotation(ref mut v) => {
                                if columns.required_columns.contains(DataColumns::Orientation) {
                                    *v = rotation[i] * *v
                                }
                            }
                        })
                    });

                    if next_stages.len() == 1 {
                        add_spawns
                            .append(&mut next_stages.into_iter().map(|d| (d, Some(i))).collect());
                    } else {
                        add_spawns
                            .append(&mut next_stages.into_iter().map(|d| (d, None)).collect());
                    }
                }

                dead[i] = dead[i] || this_dead
            }
        },
    }
}
