use enumset::{EnumSet, EnumSetType};
use target_features::CURRENT_TARGET;

use crate::danmaku::data::{DanmakuSpawnData, RenderData};

pub mod data;
pub mod handlers;
pub mod standard;

pub const N: usize = if let Some(size) = CURRENT_TARGET.suggested_simd_width::<f32>() {
    size
} else {
    // If SIMD isn't supported natively, we use a vector of 1 element.
    // This is effectively a scalar value.
    1
};

pub trait DanmakuData {
    type DataColumns: EnumSetType;
    type SpawnData;

    fn new(new_column_size: usize, required: EnumSet<Self::DataColumns>) -> Self;

    fn required_columns(&self) -> EnumSet<Self::DataColumns>;

    fn grab_new_spawns(
        &mut self,
    ) -> Vec<(
        DanmakuSpawnData<Self::SpawnData, Self::DataColumns>,
        Option<usize>,
    )>;

    fn resize(&mut self, new_max_size: usize);

    fn compact(&mut self, new_max_size: usize);

    fn id(&mut self) -> &mut Vec<i128>;
    fn dead(&mut self) -> &mut Vec<bool>;
    fn current_dead_len(&self) -> usize;

    fn add_danmaku_at_idx(
        &mut self,
        idx: usize,
        danmaku: DanmakuSpawnData<Self::SpawnData, Self::DataColumns>,
        id: i128,
    ) -> Vec<DanmakuSpawnData<Self::SpawnData, Self::DataColumns>>;

    fn compute_transform_mats(&mut self, current_size: usize, partial_ticks: f32);
    fn compute_and_get_render_data(
        &mut self,
        current_size: usize,
        partial_ticks: f32,
    ) -> Vec<(i128, RenderData)>;
}

pub struct Behavior<C: DanmakuData> {
    pub identifier: &'static str,
    pub required_columns: EnumSet<C::DataColumns>,
    pub act: fn(&mut C, usize),
}
