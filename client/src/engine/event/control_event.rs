use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};
use protocol::{
    data::{control_data::ControlData, player_data::PlayerData},
    packet::Packet,
    route::GameRoute,
};

use crate::engine::plugin::network_plugin::NetWorkState;

pub struct ControlEventPlugin;

impl Plugin for ControlEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ControlLastOne {
            control: ControlData {
                uid: 0,
                direction: (0., 0.),
                action: 0,
            },
        })
        .add_event::<ControlEvent>()
        .add_system(event_listener_system.system());
    }
}

#[derive(Debug)]
pub struct ControlEvent {
    //方向 模拟输入
    pub direction: (f32, f32),
    // 动作 0停止, 1走, 2跑...
    pub action: u8,
}

pub struct ControlLastOne {
    control: ControlData,
}

fn event_listener_system(
    mut control_event_reader: EventReader<ControlEvent>,
    net_state: ResMut<NetWorkState>,
    mut control_last_one: ResMut<ControlLastOne>,
    mut player_query: Query<&RigidBodyHandleComponent, With<PlayerData>>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for control_event in control_event_reader.iter() {
        if let Ok(mut to_be_sent_queue) = net_state.to_be_sent_queue.lock() {
            if control_last_one.control.direction == control_event.direction
                && control_last_one.control.action == control_event.action
            {
                continue;
            }
            to_be_sent_queue.push(Packet::Game(GameRoute::Control(ControlData {
                uid: 0,
                direction: control_event.direction,
                action: control_event.action,
            })));
            control_last_one.control = ControlData {
                uid: 0,
                direction: control_event.direction,
                action: control_event.action,
            };
            if let Ok(rb_handle) = player_query.single_mut() {
                if let Some(rb) = rigid_bodies.get_mut(rb_handle.handle()) {
                    if control_event.action == 1 {
                        rb.set_linvel(
                            Vector2::new(control_event.direction.0, control_event.direction.1)
                                * 100.,
                            true,
                        );
                    }
                    if control_event.action == 2 {
                        rb.set_linvel(
                            Vector2::new(control_event.direction.0, control_event.direction.1)
                                * 150.,
                            true,
                        );
                    }
                }
            }
            // println!("收到控制事件: {:?}", control_event);
        }
    }
}
