use enumset::EnumSet;
use crate::behavior::{Behavior, BehaviorNoOp};
use crate::behavior::main_columns::{Columns, RequiredMainColumns};

pub struct SimpleMotionBehavior {
    pub forward_speed: f32
}

impl Behavior for SimpleMotionBehavior {
    fn identifier(&self) -> &'static str {
        "simple_motion"
    }

    fn required_main_columns(&self) -> EnumSet<RequiredMainColumns> {
        EnumSet::only(RequiredMainColumns::PosZ)
    }

    fn extra_columns(&self) -> Vec<&'static str> {
        vec!["motion-z"]
    }

    fn transfer_extra_data(&self, columns: &mut Columns, idx: usize) {
        columns.extra_data.get_mut("motion-z").unwrap()[idx] = self.forward_speed;
    }

    fn act(&self, columns: &mut Columns, size: usize) {
        
        let motion_z = columns.extra_data.get("motion-z").unwrap();
        let pos_z = &mut columns.pos_z;
        let old_pos_z = &mut columns.old_pos_z;

        old_pos_z[0..size].copy_from_slice(&pos_z[0..size]);

        for i in 0..size {
            pos_z[i] += motion_z[i]
        }
    }
}

impl BehaviorNoOp for SimpleMotionBehavior {
    fn no_op_data() -> Self {
        SimpleMotionBehavior {
            forward_speed: 0.0
        }
    }
}