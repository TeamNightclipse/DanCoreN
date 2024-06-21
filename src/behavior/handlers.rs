use crate::behavior::danmaku_data::{BehaviorData, DanmakuSpawnData, RenderData};
use crate::behavior::main_columns::{Columns, DataColumns, N_F32};
use crate::behavior::Behavior;
use crate::color::ColorHex;
use enumset::EnumSet;
use nalgebra::{Matrix4, UnitQuaternion, Vector3};
use priority_queue::PriorityQueue;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
use std::simd::{Simd, SimdElement};

pub struct TopDanmakuBehaviorsHandler {
    handlers: HashMap<Vec<&'static str>, DanmakuBehaviorHandler>,
    behaviors: HashMap<&'static str, Rc<Behavior>>,

    global_family_depth_map: HashMap<i128, i16>,
    global_parent_map: HashMap<i128, i128>,

    next_identifier: i64,
}

impl TopDanmakuBehaviorsHandler {
    pub fn new() -> TopDanmakuBehaviorsHandler {
        TopDanmakuBehaviorsHandler {
            handlers: HashMap::new(),
            behaviors: HashMap::new(),
            global_family_depth_map: HashMap::new(),
            global_parent_map: HashMap::new(),

            next_identifier: 0,
        }
    }

    pub fn register_behavior(&mut self, behavior: Behavior) {
        self.behaviors
            .insert(behavior.identifier, Rc::new(behavior));
    }

    fn add_single_danmaku(
        &mut self,
        d: DanmakuSpawnData,
        preferred_idx: Option<(usize, i64)>,
    ) -> Vec<DanmakuSpawnData> {
        let handler = match self.handlers.get_mut(&d.behaviors) {
            Some(t) => t,
            None => {
                let behaviors = d
                    .behaviors
                    .iter()
                    .map(|b| Rc::clone(self.behaviors.get(b).unwrap()))
                    .collect();

                self.next_identifier += 1;
                self.handlers.insert(
                    d.behaviors.clone(),
                    DanmakuBehaviorHandler::new(self.next_identifier, behaviors, false),
                );

                self.handlers.get_mut(&d.behaviors).unwrap()
            }
        };

        handler.add_danmaku_with_preffered_index(
            d,
            preferred_idx
                .filter(|(_, original_handler_identifier)| {
                    *original_handler_identifier == handler.identifier
                })
                .map(|(idx, _)| idx),
            &mut self.global_family_depth_map,
            &mut self.global_parent_map,
        )
    }

    pub fn add_danmaku(&mut self, danmaku: Vec<DanmakuSpawnData>) {
        let mut pending = danmaku;

        while let Some(d) = pending
            .pop()
            .into_iter()
            .filter_map(|mut d| {
                if d.set_family_depth(&self.global_family_depth_map) {
                    Some(d)
                } else {
                    None
                }
            })
            .next()
        {
            pending.append(&mut self.add_single_danmaku(d, None));
        }
    }

    pub fn tick(&mut self) {
        let mut with_idx: Vec<(DanmakuSpawnData, usize, i64)> = vec![];
        let mut simple = vec![];

        for h in self.handlers.values_mut() {
            for (d, idx) in h.tick() {
                match idx {
                    None => simple.push(d),
                    Some(i) => with_idx.push((d, i, h.identifier)),
                }
            }
        }

        while let Some((d, idx, handler_id)) = with_idx.pop() {
            simple.append(&mut self.add_single_danmaku(d, Some((idx, handler_id))));
        }

        self.add_danmaku(simple)
    }

    pub fn render_data(&mut self, partial_ticks: f32) -> Vec<RenderData> {
        let mut local_render_data: HashMap<i128, RenderData> = self
            .handlers
            .values_mut()
            .flat_map(|h| h.compute_and_get_render_data(partial_ticks))
            .collect();

        let mut remaining_relationships: PriorityQueue<_, i16> = self
            .global_parent_map
            .iter()
            .map(|(child, parent)| {
                let depth = *self.global_family_depth_map.get(child).unwrap_or(&0);
                ((child, parent), depth)
            })
            .collect();

        while let Some(((child_id, parent_id), _)) = remaining_relationships.pop() {
            let parent_opt = local_render_data.get(parent_id).map(|p| p.model_mat);

            if let Entry::Occupied(mut o) = local_render_data.entry(*child_id) {
                match parent_opt {
                    Some(parent) => {
                        o.get_mut().model_mat = parent * o.get().model_mat;
                    }
                    None => {
                        o.remove();
                    }
                };
            }
        }

        local_render_data.into_values().collect()
    }

    pub fn cleanup(&mut self) {
        self.handlers.retain(|_, h| h.always_keep || h.count() > 0);
        // TODO: Scale down
    }
}

struct DanmakuBehaviorHandler {
    always_keep: bool,
    identifier: i64,
    next_dan_identifier: i64,

    size_exp: u8,
    current_size: usize,

    behaviors: Vec<Rc<Behavior>>,
    columns: Columns,
}

impl DanmakuBehaviorHandler {
    fn new(
        identifier: i64,
        behaviors: Vec<Rc<Behavior>>,
        always_keep: bool,
    ) -> DanmakuBehaviorHandler {
        let required_main_columns: EnumSet<DataColumns> =
            behaviors.iter().map(|b| b.required_columns).collect();

        let size_exp = 7;
        let max_size = 1 << size_exp;

        DanmakuBehaviorHandler {
            always_keep,
            identifier,
            next_dan_identifier: 0,

            size_exp,
            current_size: 0,

            behaviors,
            columns: Columns::new(max_size, required_main_columns),
        }
    }

    fn current_max_size(&self) -> usize {
        1 << self.size_exp
    }

    fn dead(&self) -> usize {
        self.columns.current_dead.len()
    }

    fn count(&self) -> usize {
        self.current_size - self.dead()
    }

    fn should_resize_up_soon(&self) -> bool {
        if self.size_exp > 30 {
            return false;
        }

        let max = self.current_max_size();
        self.current_size as f64 + (max as f64 * 0.1) > max as f64
    }

    fn should_resize_down_soon(&self) -> bool {
        if self.size_exp < 8 {
            return false;
        }
        let step_down_max_size = 1 << (self.size_exp - 1);
        let surplus_if_step_down = step_down_max_size - self.current_size;
        surplus_if_step_down as f64 > (step_down_max_size as f64 * 0.1)
    }

    fn must_resize_before_add(&self, length: usize) -> bool {
        self.current_size + length >= self.current_max_size()
    }

    fn transfer_data_simd<A: SimdElement>(
        required_columns: EnumSet<DataColumns>,
        i: usize,
        required: DataColumns,
        vec: &mut [Simd<A, N_F32>],
        data: A,
    ) {
        if required_columns.contains(required) {
            vec[i.div_ceil(N_F32)][i % N_F32] = data;
        }
    }

    fn transfer_data<A>(
        required_columns: EnumSet<DataColumns>,
        i: usize,
        required: DataColumns,
        vec: &mut [A],
        data: A,
    ) {
        if required_columns.contains(required) {
            vec[i] = data;
        }
    }

    fn add_danmaku_with_preffered_index(
        &mut self,
        mut danmaku: DanmakuSpawnData,
        idx: Option<usize>,
        global_family_depth_map: &mut HashMap<i128, i16>,
        global_parent_map: &mut HashMap<i128, i128>,
    ) -> Vec<DanmakuSpawnData> {
        let idx_with_filter = idx.filter(|i| *self.columns.dead.get(*i).unwrap_or(&false));
        let i = idx_with_filter.unwrap_or(self.current_size);

        if self.must_resize_before_add(if idx_with_filter.is_some() { 0 } else { 1 }) {
            self.resize(true)
        }

        self.current_size += 1;

        let this_id = ((self.identifier as i128) << 64) + (self.next_dan_identifier as i128);
        self.next_dan_identifier += 1;

        self.columns.id[i] = this_id;

        let render_properties = danmaku.render_properties;

        for d in danmaku.behavior_data {
            match d {
                BehaviorData::PosX(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::PosX,
                        &mut self.columns.pos_x,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::PosX,
                        &mut self.columns.old_pos_x,
                        v,
                    );
                }
                BehaviorData::PosY(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::PosY,
                        &mut self.columns.pos_y,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::PosY,
                        &mut self.columns.old_pos_y,
                        v,
                    );
                }
                BehaviorData::PosZ(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::PosZ,
                        &mut self.columns.pos_z,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::PosZ,
                        &mut self.columns.old_pos_z,
                        v,
                    );
                }
                BehaviorData::Orientation(v) => {
                    Self::transfer_data(
                        self.columns.required_columns,
                        i,
                        DataColumns::Orientation,
                        &mut self.columns.orientation,
                        v,
                    );
                    Self::transfer_data(
                        self.columns.required_columns,
                        i,
                        DataColumns::Orientation,
                        &mut self.columns.old_orientation,
                        v,
                    );
                }
                BehaviorData::Appearance { form } => {
                    Self::transfer_data(
                        self.columns.required_columns,
                        i,
                        DataColumns::Appearance,
                        &mut self.columns.form,
                        form,
                    );
                    Self::transfer_data(
                        self.columns.required_columns,
                        i,
                        DataColumns::Appearance,
                        &mut self.columns.render_properties,
                        render_properties.clone(),
                    );
                }
                BehaviorData::MainColor(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::MainColor,
                        &mut self.columns.main_color,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::MainColor,
                        &mut self.columns.old_main_color,
                        v,
                    );
                }
                BehaviorData::SecondaryColor(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::SecondaryColor,
                        &mut self.columns.secondary_color,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::SecondaryColor,
                        &mut self.columns.old_secondary_color,
                        v,
                    );
                }
                BehaviorData::Damage(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::Damage,
                        &mut self.columns.damage,
                        v,
                    );
                }
                BehaviorData::SizeX(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::ScaleX,
                        &mut self.columns.scale_x,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::ScaleX,
                        &mut self.columns.old_scale_x,
                        v,
                    );
                }
                BehaviorData::SizeY(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::ScaleY,
                        &mut self.columns.scale_y,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::ScaleY,
                        &mut self.columns.old_scale_y,
                        v,
                    );
                }
                BehaviorData::SizeZ(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::ScaleZ,
                        &mut self.columns.scale_z,
                        v,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::ScaleZ,
                        &mut self.columns.old_scale_z,
                        v,
                    );
                }
                BehaviorData::MotionX(v) => Self::transfer_data_simd(
                    self.columns.required_columns,
                    i,
                    DataColumns::MotionX,
                    &mut self.columns.motion_x,
                    v,
                ),
                BehaviorData::MotionY(v) => Self::transfer_data_simd(
                    self.columns.required_columns,
                    i,
                    DataColumns::MotionY,
                    &mut self.columns.motion_x,
                    v,
                ),
                BehaviorData::MotionZ(v) => Self::transfer_data_simd(
                    self.columns.required_columns,
                    i,
                    DataColumns::MotionZ,
                    &mut self.columns.motion_x,
                    v,
                ),
                BehaviorData::GravityX(v) => Self::transfer_data_simd(
                    self.columns.required_columns,
                    i,
                    DataColumns::GravityX,
                    &mut self.columns.motion_x,
                    v,
                ),
                BehaviorData::GravityY(v) => Self::transfer_data_simd(
                    self.columns.required_columns,
                    i,
                    DataColumns::GravityY,
                    &mut self.columns.motion_x,
                    v,
                ),
                BehaviorData::GravityZ(v) => Self::transfer_data_simd(
                    self.columns.required_columns,
                    i,
                    DataColumns::GravityZ,
                    &mut self.columns.motion_x,
                    v,
                ),
                BehaviorData::SpeedAccel(v) => Self::transfer_data_simd(
                    self.columns.required_columns,
                    i,
                    DataColumns::SpeedAccel,
                    &mut self.columns.speed_accel,
                    v,
                ),
                BehaviorData::Forward(v) => {
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::Forward,
                        &mut self.columns.forward_x,
                        v.x,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::Forward,
                        &mut self.columns.forward_z,
                        v.y,
                    );
                    Self::transfer_data_simd(
                        self.columns.required_columns,
                        i,
                        DataColumns::Forward,
                        &mut self.columns.forward_z,
                        v.z,
                    );
                }
                BehaviorData::Rotation(v) => Self::transfer_data(
                    self.columns.required_columns,
                    i,
                    DataColumns::Rotation,
                    &mut self.columns.rotation,
                    v,
                ),
            }
        }

        self.columns.ticks_existed[i.div_ceil(N_F32)][i % N_F32] = 0;
        self.columns.end_time[i.div_ceil(N_F32)][i % N_F32] = danmaku.end_time;
        self.columns.dead[i] = false;
        self.columns.next_stage[i] = danmaku.next_stage;
        self.columns.next_stage_add_data[i] = danmaku.next_stage_add_data;
        self.columns.parent[i] = danmaku.parent.unwrap_or(-1);
        self.columns.family_depth[i] = danmaku.family_depth;

        danmaku.parent.iter().for_each(|parent_id| {
            global_parent_map.insert(this_id, *parent_id);
        });
        global_family_depth_map.insert(this_id, danmaku.family_depth);

        self.columns.transform_mats[i].fill_with_identity();

        for c in &mut danmaku.children.iter_mut() {
            c.parent = Some(this_id);
        }

        danmaku.children
    }

    fn tick(&mut self) -> Vec<(DanmakuSpawnData, Option<usize>)> {
        for behavior in self.behaviors.iter() {
            (behavior.act)(&mut self.columns, self.current_size);
        }

        self.columns.grab_new_spawns()
    }

    #[inline]
    fn lerp_if_used(
        partial_ticks: f32,
        used: bool,
        i: usize,
        old: &[Simd<f32, N_F32>],
        new: &[Simd<f32, N_F32>],
    ) -> f32 {
        if used {
            nalgebra_glm::lerp_scalar(
                old[i.div_ceil(N_F32)][i % N_F32],
                new[i.div_ceil(N_F32)][i % N_F32],
                partial_ticks,
            )
        } else {
            0.0
        }
    }

    fn compute_transform_mats(&mut self, partial_ticks: f32) {
        let required_main_columns = self.columns.required_columns;

        if required_main_columns.contains(DataColumns::Appearance) {
            let requires_scale_x = required_main_columns.contains(DataColumns::ScaleX);
            let requires_scale_y = required_main_columns.contains(DataColumns::ScaleY);
            let requires_scale_z = required_main_columns.contains(DataColumns::ScaleZ);
            let requires_pos_x = required_main_columns.contains(DataColumns::PosX);
            let requires_pos_y = required_main_columns.contains(DataColumns::PosY);
            let requires_pos_z = required_main_columns.contains(DataColumns::PosZ);
            let requires_orientation = required_main_columns.contains(DataColumns::Orientation);

            let mut temp = Matrix4::identity();

            let pos_x = &self.columns.pos_x;
            let pos_y = &self.columns.pos_y;
            let pos_z = &self.columns.pos_z;
            let old_pos_x = &self.columns.old_pos_x;
            let old_pos_y = &self.columns.old_pos_y;
            let old_pos_z = &self.columns.old_pos_z;

            let scale_x = &self.columns.scale_x;
            let scale_y = &self.columns.scale_y;
            let scale_z = &self.columns.scale_z;
            let old_scale_x = &self.columns.old_scale_x;
            let old_scale_y = &self.columns.old_scale_y;
            let old_scale_z = &self.columns.old_scale_z;

            let orientation = &self.columns.orientation;
            let old_orientation = &self.columns.old_orientation;

            let dead = &self.columns.dead;

            for i in 0..self.current_size {
                if !dead[i] {
                    temp.fill_with_identity();

                    temp.append_nonuniform_scaling_mut(&Vector3::new(
                        Self::lerp_if_used(
                            partial_ticks,
                            requires_scale_x,
                            i,
                            old_scale_x,
                            scale_x,
                        ),
                        Self::lerp_if_used(
                            partial_ticks,
                            requires_scale_y,
                            i,
                            old_scale_y,
                            scale_y,
                        ),
                        Self::lerp_if_used(
                            partial_ticks,
                            requires_scale_z,
                            i,
                            old_scale_z,
                            scale_z,
                        ),
                    ));

                    if requires_pos_x || requires_pos_y || requires_pos_z {
                        temp.append_translation_mut(&Vector3::new(
                            Self::lerp_if_used(partial_ticks, requires_pos_x, i, old_pos_x, pos_x),
                            Self::lerp_if_used(partial_ticks, requires_pos_y, i, old_pos_y, pos_y),
                            Self::lerp_if_used(partial_ticks, requires_pos_z, i, old_pos_z, pos_z),
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

                    self.columns.transform_mats[i] = orientation_mat * temp;
                }
            }
        }
    }

    fn compute_and_get_render_data(&mut self, partial_ticks: f32) -> Vec<(i128, RenderData)> {
        self.compute_transform_mats(partial_ticks);

        let form = &self.columns.form;
        let render_properties = &self.columns.render_properties;
        let transform_mats = &self.columns.transform_mats;
        let main_color = &self.columns.main_color;
        let old_main_color = &self.columns.old_main_color;
        let secondary_color = &self.columns.secondary_color;
        let old_secondary_color = &self.columns.old_secondary_color;
        let ticks_existed = &self.columns.ticks_existed;
        let end_time = &self.columns.end_time;
        let dead = &self.columns.dead;
        let id = &self.columns.id;
        
        let has_main_color = self.columns.required_columns.contains(DataColumns::MainColor);
        let has_secondary_color = self.columns.required_columns.contains(DataColumns::SecondaryColor);

        if self
            .columns
            .required_columns
            .contains(DataColumns::Appearance)
        {
            (0..self.current_size)
                .filter(|i| !dead.get(*i).unwrap_or(&false))
                .map(|i| (id.get(i).unwrap_or(&0), i))
                .map(|(id, i)| {
                    let lerp_color = |has_color: bool, new: &Vec<Simd<i32, N_F32>>, old: &Vec<Simd<i32, N_F32>>| -> ColorHex {
                        if has_color {
                            ColorHex(new[i.div_ceil(N_F32)][i % N_F32])
                                .lerp_through_hsv(ColorHex(old[i.div_ceil(N_F32)][i % N_F32]), partial_ticks)
                        } else {
                            ColorHex(0)
                        }
                    };
                    
                    let main_color = lerp_color(has_main_color, main_color, old_main_color);
                    let secondary_color = lerp_color(has_secondary_color, secondary_color, old_secondary_color);
                    
                    (
                        *id,
                        RenderData {
                            form: form.get(i).unwrap(),
                            render_properties: render_properties.get(i).unwrap(),
                            model_mat: *transform_mats.get(i).unwrap_or(&Matrix4::identity()),
                            main_color: main_color.0,
                            secondary_color: secondary_color.0,
                            ticks_existed: ticks_existed[i.div_ceil(N_F32)][i & N_F32],
                            end_time: end_time[i.div_ceil(N_F32)][i & N_F32],
                        },
                    )
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn resize(&mut self, force_up: bool) {
        if force_up || self.should_resize_up_soon() {
            self.size_exp += 1;
            self.columns.resize(self.current_max_size());
        } else if self.should_resize_down_soon() {
            let dead = self.dead();
            self.size_exp -= 1;
            self.columns.compact(self.current_max_size());
            self.current_size -= dead;
        } else {
            // Something weird is going on. Cancel the resizing
            return;
        }
    }
}
