use enumset::EnumSet;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::behavior::{Behavior, BehaviorNoOp};
use priority_queue::PriorityQueue;

use crate::behavior::danmaku_data::{DanmakuSpawnData, RenderData};
use crate::behavior::main_columns::{Columns, RequiredMainColumns};
use crate::color::ColorHex;
use itertools::Itertools;
use nalgebra::{Matrix4, UnitQuaternion, Vector3};

pub struct TopDanmakuBehaviorsHandler {
    handlers: HashMap<Vec<&'static str>, DanmakuBehaviorHandler>,
    no_op_behaviors: HashMap<&'static str, fn() -> Box<dyn Behavior>>,

    global_family_depth_map: HashMap<i128, i16>,
    global_parent_map: HashMap<i128, i128>,

    next_identifier: i64,
}

impl TopDanmakuBehaviorsHandler {
    pub fn new() -> TopDanmakuBehaviorsHandler {
        TopDanmakuBehaviorsHandler {
            handlers: HashMap::new(),
            no_op_behaviors: HashMap::new(),
            global_family_depth_map: HashMap::new(),
            global_parent_map: HashMap::new(),

            next_identifier: 0,
        }
    }

    pub fn register_behavior<A: Behavior + BehaviorNoOp + 'static>(&mut self) {
        let no_op = A::no_op_data();
        self.no_op_behaviors
            .insert(no_op.identifier(), || Box::new(A::no_op_data()));
    }

    fn add_single_danmaku(
        &mut self,
        d: DanmakuSpawnData,
        preferred_idx: Option<(usize, i64)>,
    ) -> Vec<DanmakuSpawnData> {
        let behavior_identifiers: Vec<&'static str> =
            d.behavior.iter().map(|b| b.identifier()).collect();

        let handler = match self.handlers.get_mut(&behavior_identifiers) {
            Some(t) => t,
            None => {
                let no_ops = d
                    .behavior
                    .iter()
                    .map(|b| self.no_op_behaviors.get(b.identifier()).unwrap()())
                    .collect();

                self.next_identifier += 1;
                self.handlers.insert(
                    behavior_identifiers,
                    DanmakuBehaviorHandler::new(self.next_identifier, no_ops, false),
                );

                let id: Vec<&'static str> = d.behavior.iter().map(|b| b.identifier()).collect();

                self.handlers.get_mut(&id).unwrap()
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
                with_idx.push((d, idx, h.identifier))
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
        self.handlers.values_mut().for_each(|h| h.compact())
    }
}

struct DanmakuBehaviorHandler {
    always_keep: bool,
    identifier: i64,
    next_dan_identifier: i64,

    size_exp: u8,
    current_size: usize,

    behavior_no_ops: Vec<Box<dyn Behavior>>,
    columns: Columns,
}

impl DanmakuBehaviorHandler {
    fn new(
        identifier: i64,
        behavior_no_ops: Vec<Box<dyn Behavior>>,
        always_keep: bool,
    ) -> DanmakuBehaviorHandler {
        let required_main_columns: EnumSet<RequiredMainColumns> = behavior_no_ops
            .iter()
            .map(|b| b.required_main_columns())
            .collect();
        let extra_data_identifiers = behavior_no_ops
            .iter()
            .flat_map(|b| b.extra_columns())
            .unique()
            .collect();

        let size_exp = 7;
        let max_size = 1 << size_exp;

        DanmakuBehaviorHandler {
            always_keep,
            identifier,
            next_dan_identifier: 0,

            size_exp,
            current_size: 0,

            behavior_no_ops,
            columns: Columns::new(max_size, required_main_columns, extra_data_identifiers),
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
        let max = self.current_max_size();
        self.current_size as f64 + (max as f64 * 0.1) > max as f64
    }

    fn should_resize_down_soon(&self) -> bool {
        let step_down_max_size = 1 << (self.size_exp - 1);
        self.size_exp > 7
            && self.current_size as f64 + (step_down_max_size as f64 * 0.25)
                < step_down_max_size as f64
    }

    fn must_resize_before_add(&self, length: usize) -> bool {
        self.current_size + length >= self.current_max_size()
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

        self.columns.pos_x[i] = danmaku.pos.x;
        self.columns.pos_y[i] = danmaku.pos.y;
        self.columns.pos_z[i] = danmaku.pos.z;

        self.columns.old_pos_x[i] = danmaku.pos.x;
        self.columns.old_pos_y[i] = danmaku.pos.y;
        self.columns.old_pos_z[i] = danmaku.pos.z;

        self.columns.orientation[i] = danmaku.orientation;
        self.columns.old_orientation[i] = danmaku.orientation;

        self.columns.scale_x[i] = danmaku.shot_data.size_x;
        self.columns.scale_y[i] = danmaku.shot_data.size_y;
        self.columns.scale_z[i] = danmaku.shot_data.size_z;

        self.columns.main_color[i] = danmaku.shot_data.main_color;
        self.columns.old_main_color[i] = danmaku.shot_data.main_color;
        self.columns.secondary_color[i] = danmaku.shot_data.secondary_color;
        self.columns.old_secondary_color[i] = danmaku.shot_data.secondary_color;

        self.columns.damage[i] = danmaku.shot_data.damage;
        self.columns.form[i] = danmaku.shot_data.form;
        self.columns.render_properties[i] = danmaku.shot_data.render_properties;

        self.columns.ticks_existed[i] = 0;
        self.columns.end_time[i] = danmaku.shot_data.end_time;
        self.columns.dead[i] = false;
        self.columns.next_stage[i] = danmaku.next_stage;
        self.columns.parent[i] = danmaku.parent.unwrap_or(-1);
        self.columns.family_depth[i] = danmaku.family_depth;

        danmaku.parent.iter().for_each(|parent_id| {
            global_parent_map.insert(this_id, *parent_id);
        });
        global_family_depth_map.insert(this_id, danmaku.family_depth);

        let mut jd = 0;
        let mut js = 0;
        loop {
            match (danmaku.behavior.get(jd), self.behavior_no_ops.get(js)) {
                (Some(data), Some(no_op)) if data.identifier() == no_op.identifier() => {
                    data.transfer_extra_data(&mut self.columns, i);
                    jd += 1;
                    js += 1;
                }
                (_, Some(no_op)) => {
                    no_op.transfer_extra_data(&mut self.columns, i);
                    js += 1;
                }
                (Some(_), None) => {
                    panic!("Found data without handler")
                }
                (None, None) => break,
            }
        }

        self.columns.transform_mats[i].fill_with_identity();

        for c in &mut danmaku.children.iter_mut() {
            c.parent = Some(this_id);
        }

        danmaku.children
    }

    fn tick(&mut self) -> Vec<(DanmakuSpawnData, usize)> {
        for behavior in self.behavior_no_ops.iter() {
            behavior.act(&mut self.columns, self.current_size);
        }

        self.columns.grab_new_spawns()
    }

    fn lerp(start: f32, end: f32, t: f32) -> f32 {
        start * (1.0 - t) + end * t
    }

    #[inline]
    fn lerp_if_used(
        partial_ticks: f32,
        used: bool,
        i: usize,
        old: &[f32],
        new: &[f32],
    ) -> f32 {
        if used {
            Self::lerp(
                *old.get(i).unwrap_or(&0.0),
                *new.get(i).unwrap_or(&0.0),
                partial_ticks,
            )
        } else {
            *new.get(i).unwrap_or(&0.0)
        }
    }

    fn compute_transform_mats(&mut self, partial_ticks: f32) {
        let required_main_columns = self.columns.required_main_columns;

        if required_main_columns.contains(RequiredMainColumns::Appearance) {
            let requires_scale_x = required_main_columns.contains(RequiredMainColumns::ScaleX);
            let requires_scale_y = required_main_columns.contains(RequiredMainColumns::ScaleY);
            let requires_scale_z = required_main_columns.contains(RequiredMainColumns::ScaleZ);
            let requires_pos_x = required_main_columns.contains(RequiredMainColumns::PosX);
            let requires_pos_y = required_main_columns.contains(RequiredMainColumns::PosY);
            let requires_pos_z = required_main_columns.contains(RequiredMainColumns::PosZ);
            let requires_orientation =
                required_main_columns.contains(RequiredMainColumns::Orientation);

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

            let mut i = 0;
            while i < self.current_size {
                if !dead.get(i).unwrap_or(&false) {
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

                i += 1
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

        (0..self.current_size)
            .filter(|i| !dead.get(*i).unwrap_or(&false))
            .map(|i| (id.get(i).unwrap_or(&0), i))
            .map(|(id, i)| {
                let main_color = ColorHex(*main_color.get(i).unwrap_or(&0));
                let old_main_color = ColorHex(*old_main_color.get(i).unwrap_or(&0));
                let secondary_color = ColorHex(*secondary_color.get(i).unwrap_or(&0));
                let old_secondary_color = ColorHex(*old_secondary_color.get(i).unwrap_or(&0));

                (
                    *id,
                    RenderData {
                        form: form.get(i).unwrap(),
                        render_properties: render_properties.get(i).unwrap(),
                        model_mat: *transform_mats.get(i).unwrap_or(&Matrix4::identity()),
                        model_view_mat: Matrix4::identity(),
                        main_color: old_main_color.lerp_through_hsv(main_color, partial_ticks).0,
                        secondary_color: old_secondary_color
                            .lerp_through_hsv(secondary_color, partial_ticks)
                            .0,
                        ticks_existed: *ticks_existed.get(i).unwrap_or(&0),
                        end_time: *end_time.get(i).unwrap_or(&0),
                        distance_from_camera: -1.0,
                    },
                )
            })
            .collect()
    }

    fn resize(&mut self, force_up: bool) {
        if !force_up && self.dead() as f64 > self.current_size as f64 * 0.2 {
            self.compact()
        } else if force_up || self.should_resize_up_soon() {
            self.size_exp += 1
        } else if self.should_resize_down_soon() {
            self.size_exp -= 1
        } else {
            // Something weird is going on. Cancel the resizing
            return;
        }

        self.columns.resize(self.current_max_size());
    }

    fn compact_vec<A>(vec: &mut Vec<A>, remove: &[bool]) {
        let mut j = 0;
        vec.retain(|_| {
            j += 1;
            *remove.get(j - 1).unwrap_or(&false)
        })
    }

    fn compact(&mut self) {
        // No need to compact if the amount of dead is not too great
        if (self.dead() as f64) < self.current_size as f64 * 0.2 {
            return;
        }

        let dead = &self.columns.dead;

        [&mut self.columns.id, &mut self.columns.parent]
            .iter_mut()
            .for_each(|d| Self::compact_vec(d, dead));
        [
            &mut self.columns.pos_x,
            &mut self.columns.pos_y,
            &mut self.columns.pos_z,
            &mut self.columns.old_pos_x,
            &mut self.columns.old_pos_y,
            &mut self.columns.old_pos_z,
            &mut self.columns.scale_x,
            &mut self.columns.scale_y,
            &mut self.columns.scale_z,
            &mut self.columns.old_scale_x,
            &mut self.columns.old_scale_y,
            &mut self.columns.old_scale_z,
            &mut self.columns.damage,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        self.columns
            .extra_data
            .values_mut()
            .for_each(|d| Self::compact_vec(d, dead));

        [
            &mut self.columns.orientation,
            &mut self.columns.old_orientation,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        [
            &mut self.columns.main_color,
            &mut self.columns.old_main_color,
            &mut self.columns.secondary_color,
            &mut self.columns.old_secondary_color,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        Self::compact_vec(&mut self.columns.form, dead);
        Self::compact_vec(&mut self.columns.render_properties, dead);

        [
            &mut self.columns.ticks_existed,
            &mut self.columns.end_time,
            &mut self.columns.family_depth,
        ]
        .iter_mut()
        .for_each(|d| Self::compact_vec(d, dead));

        Self::compact_vec(&mut self.columns.next_stage, dead);
        Self::compact_vec(&mut self.columns.transform_mats, dead);

        let _ = &mut self.columns.dead.retain(|d| *d);

        self.current_size -= self.columns.current_dead.len();
        let _ = &mut self.columns.current_dead.clear();
    }
}
