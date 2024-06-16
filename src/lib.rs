mod behavior;
mod form;
mod color;

use crate::behavior::danmaku_data::{DanmakuSpawnData, ShotData};
use crate::form::Form;
use behavior::standard_behaviors::SimpleMotionBehavior;
use behavior::handlers::TopDanmakuBehaviorsHandler;
use nalgebra::{UnitQuaternion, Vector3};

pub fn test() {
    let mut top = TopDanmakuBehaviorsHandler::new();
    top.register_behavior::<SimpleMotionBehavior>();

    top.add_danmaku(vec![DanmakuSpawnData {
        pos: Vector3::new(0.0, 0.0, 0.0),
        orientation: UnitQuaternion::identity(),
        shot_data: ShotData::new(&Form::SPHERE),
        behavior: vec![Box::new(SimpleMotionBehavior { forward_speed: 0.1 })],
        next_stage: vec![],
        parent: None,
        children: vec![],
        family_depth: -1,
    }]);
    
    top.tick();
    
    top.render_data(0.1);
    
    top.cleanup();
}
