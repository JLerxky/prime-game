use bevy::prelude::*;
use protocol::{
    data::skill_data::{SkillData, SkillType},
    packet::Packet,
    route::GameRoute,
};

use crate::engine::plugin::network_plugin::NetWorkState;

pub struct SkillEventPlugin;

impl Plugin for SkillEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SkillEvent>()
            .add_system(event_listener_system.system());
    }
}

#[derive(Debug)]
pub struct SkillEvent {
    //方向 模拟输入
    pub direction: (f32, f32),
    // 动作 0停止, 1走, 2跑...
    pub skill_type: SkillType,
}

fn event_listener_system(
    mut skill_event_reader: EventReader<SkillEvent>,
    net_state: ResMut<NetWorkState>,
) {
    for skill_event in skill_event_reader.iter() {
        if let Ok(mut to_be_sent_queue) = net_state.to_be_sent_queue.lock() {
            to_be_sent_queue.push(Packet::Game(GameRoute::Skill(SkillData {
                uid: 0,
                direction: skill_event.direction,
                skill_type: skill_event.skill_type,
                texture: (0, 6, 1),
            })));
            // println!("收到技能事件: {:?}", skill_event);
        }
    }
}
