pub mod danmaku_data;
pub mod main_columns;
pub mod handlers;
pub mod standard_behaviors;

use enumset::EnumSet;
use main_columns::{Columns, RequiredMainColumns};

pub trait Behavior {
    fn identifier(&self) -> &'static str;
    fn required_main_columns(&self) -> EnumSet<RequiredMainColumns>;
    fn extra_columns(&self) -> Vec<&'static str>;

    fn transfer_extra_data(&self, columns: &mut Columns, idx: usize);

    fn act(&self, columns: &mut Columns, size: usize);
}

pub trait BehaviorNoOp {
    fn no_op_data() -> Self;
}
