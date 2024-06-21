#![feature(portable_simd)]

mod behavior;
mod color;
mod form;

use crate::behavior::danmaku_data::{BehaviorData, DanmakuSpawnData};
use crate::behavior::standard_behaviors::*;
use crate::form::Form;
use behavior::handlers::TopDanmakuBehaviorsHandler;
use std::collections::HashMap;
use enumset::EnumSet;

pub fn test() {
    let mut top = TopDanmakuBehaviorsHandler::new();
    top.register_behavior(motion1_behavior());
    top.register_behavior(gravity1_behavior());
    top.register_behavior(acceleration1_behavior());

    top.register_behavior(rotate_orientation_behavior());
    top.register_behavior(rotate_forward_behavior());

    top.register_behavior(motion3_behavior());
    top.register_behavior(gravity3_behavior());
    top.register_behavior(acceleration3_behavior());
    
    top.register_behavior(mandatory_end());

    top.add_danmaku(vec![DanmakuSpawnData {
        end_time: 80,
        behavior_data: vec![
            BehaviorData::Damage(0.5),
            BehaviorData::MainColor(0xFF0000),
            BehaviorData::SecondaryColor(0xFFFFFF),
            BehaviorData::SizeX(0.5),
            BehaviorData::SizeY(0.5),
            BehaviorData::SizeZ(0.5),
            BehaviorData::Appearance {
                form: &Form::SPHERE,
            },
            BehaviorData::MotionZ(0.1),
        ],
        render_properties: HashMap::new(),
        behaviors: vec![MOTION1_BEHAVIOR_ID],
        next_stage: vec![],
        next_stage_add_data: EnumSet::EMPTY,
        parent: None,
        children: vec![],
        family_depth: -1,
    }]);

    top.tick();

    top.render_data(0.1);

    top.cleanup();
}
