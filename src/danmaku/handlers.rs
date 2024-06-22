use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use enumset::EnumSet;
use priority_queue::PriorityQueue;

use crate::danmaku::{
    data::{DanmakuSpawnData, RenderData},
    Behavior, DanmakuData,
};

pub struct TopDanmakuBehaviorsHandler<C: DanmakuData> {
    handlers: HashMap<Vec<&'static str>, DanmakuBehaviorHandler<C>>,
    behaviors: HashMap<&'static str, Rc<Behavior<C>>>,

    global_family_depth_map: HashMap<i128, i16>,
    global_parent_map: HashMap<i128, i128>,

    next_identifier: i64,
}
impl<C: DanmakuData> Default for TopDanmakuBehaviorsHandler<C> {
    fn default() -> Self {
        TopDanmakuBehaviorsHandler {
            handlers: HashMap::new(),
            behaviors: HashMap::new(),
            global_family_depth_map: HashMap::new(),
            global_parent_map: HashMap::new(),

            next_identifier: 0,
        }
    }
}

impl<C: DanmakuData> TopDanmakuBehaviorsHandler<C> {
    pub fn new() -> TopDanmakuBehaviorsHandler<C> {
        TopDanmakuBehaviorsHandler::default()
    }

    pub fn register_behavior(&mut self, behavior: Behavior<C>) {
        self.behaviors
            .insert(behavior.identifier, Rc::new(behavior));
    }

    fn add_single_danmaku(
        &mut self,
        d: DanmakuSpawnData<C::SpawnData, C::DataColumns>,
        preferred_idx: Option<(usize, i64)>,
    ) -> Vec<DanmakuSpawnData<C::SpawnData, C::DataColumns>> {
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

    pub fn add_danmaku(&mut self, danmaku: Vec<DanmakuSpawnData<C::SpawnData, C::DataColumns>>) {
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
        let mut with_idx: Vec<(_, usize, i64)> = vec![];
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

struct DanmakuBehaviorHandler<C: DanmakuData> {
    always_keep: bool,
    identifier: i64,
    next_dan_identifier: i64,

    size_exp: u8,
    current_size: usize,

    behaviors: Vec<Rc<Behavior<C>>>,
    columns: C,
}

impl<C: DanmakuData> DanmakuBehaviorHandler<C> {
    fn new(
        identifier: i64,
        behaviors: Vec<Rc<Behavior<C>>>,
        always_keep: bool,
    ) -> DanmakuBehaviorHandler<C> {
        let required_main_columns: EnumSet<C::DataColumns> =
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
            columns: C::new(max_size, required_main_columns),
        }
    }

    fn current_max_size(&self) -> usize {
        1 << self.size_exp
    }

    fn dead(&self) -> usize {
        self.columns.current_dead_len()
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

    fn add_danmaku_with_preffered_index(
        &mut self,
        mut danmaku: DanmakuSpawnData<C::SpawnData, C::DataColumns>,
        idx: Option<usize>,
        global_family_depth_map: &mut HashMap<i128, i16>,
        global_parent_map: &mut HashMap<i128, i128>,
    ) -> Vec<DanmakuSpawnData<C::SpawnData, C::DataColumns>> {
        let idx_with_filter = idx.filter(|i| *self.columns.dead().get(*i).unwrap_or(&false));
        let i = idx_with_filter.unwrap_or(self.current_size);

        if self.must_resize_before_add(if idx_with_filter.is_some() { 0 } else { 1 }) {
            self.resize(true)
        }

        self.current_size += 1;

        let this_id = ((self.identifier as i128) << 64) + (self.next_dan_identifier as i128);
        self.next_dan_identifier += 1;
        for c in &mut danmaku.children.iter_mut() {
            c.parent = Some(this_id);
        }

        danmaku.parent.iter().for_each(|parent_id| {
            global_parent_map.insert(this_id, *parent_id);
        });
        global_family_depth_map.insert(this_id, danmaku.family_depth);

        self.columns.add_danmaku_at_idx(i, danmaku, this_id)
    }

    fn tick(
        &mut self,
    ) -> Vec<(
        DanmakuSpawnData<C::SpawnData, C::DataColumns>,
        Option<usize>,
    )> {
        for behavior in self.behaviors.iter() {
            (behavior.act)(&mut self.columns, self.current_size);
        }

        self.columns.grab_new_spawns()
    }

    fn compute_and_get_render_data(&mut self, partial_ticks: f32) -> Vec<(i128, RenderData)> {
        self.columns
            .compute_and_get_render_data(self.current_size, partial_ticks)
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
