pub mod danmaku_data;
pub mod handlers;
pub mod columns;
pub mod standard_behaviors;

use enumset::EnumSet;
use columns::{Columns, DataColumns};

pub struct Behavior {
    pub identifier: &'static str,
    pub required_columns: EnumSet<DataColumns>,
    pub act: fn(&mut Columns, usize),
}
