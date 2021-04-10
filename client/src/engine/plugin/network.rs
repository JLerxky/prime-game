use std::{
    io,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use bevy::{core::FixedTimestep, prelude::*};
use protocol::{
    data::account_data::AccountData,
    packet::Packet,
    route::{AccountRoute, GameRoute, HeartbeatRoute},
};
use tokio::net::UdpSocket;

use crate::engine::engine_client::WindowState;

use super::{camera_ctrl::CameraCtrl, ping::PingState};

// 当前玩家uid
pub static mut UID: u32 = 0;

pub struct NetWorkState {
    pub packet_queue: Arc<Mutex<Vec<Packet>>>,
    pub to_be_sent_queue: Arc<Mutex<Vec<Packet>>>,
}

pub struct SynEntity {
    pub id: u128,
    pub health: u64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct CheckEntityHealthFixedUpdateStage;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let packet_queue: Vec<Packet> = Vec::new();
        let packet_queue = Arc::new(Mutex::new(packet_queue));
        let packet_queue_c = packet_queue.clone();

        let to_be_sent_queue: Vec<Packet> = Vec::new();
        let to_be_sent_queue = Arc::new(Mutex::new(to_be_sent_queue));
        let to_be_sent_queue_c = to_be_sent_queue.clone();

        tokio::spawn(net_client_start(packet_queue_c, to_be_sent_queue_c));
        app.insert_resource(NetWorkState {
            packet_queue,
            to_be_sent_queue,
        })
        .add_system(net_handler_system.system())
        .add_stage_after(
            CoreStage::Update,
            CheckEntityHealthFixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(1.).with_label("build_map_fixed_timestep"))
                .with_system(check_entity_health.system()),
        );
    }
}

fn check_entity_health(
    mut commands: Commands,
    syn_entity_query: Query<(&SynEntity, Entity), Without<CameraCtrl>>,
) {
    let health_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    for (syn_entity, entity) in syn_entity_query.iter() {
        if health_now > (syn_entity.health + 1) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn net_handler_system(
    net_state: ResMut<NetWorkState>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    window: Res<WindowState>,
    mut syn_entity_query: Query<(&mut SynEntity, &mut Transform), Without<CameraCtrl>>,
    mut camera_query: Query<(&mut Transform, &CameraCtrl)>,
    mut ping_state: ResMut<PingState>,
) {
    if let Ok(mut packet_queue) = net_state.packet_queue.lock() {
        for _ in 0..10 {
            if packet_queue.is_empty() {
                return;
            }
            let packet = packet_queue[0].clone();
            packet_queue.remove(0);

            match packet {
                Packet::Heartbeat(heartbeat_route) => match heartbeat_route {
                    HeartbeatRoute::In => {}
                    HeartbeatRoute::Out => {}
                    protocol::route::HeartbeatRoute::Keep(time) => {
                        let time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                            - time;
                        ping_state.ping = time as f32;
                        // println!("ping: {}", time);
                    }
                },
                Packet::Account(account_route) => match account_route {
                    AccountRoute::Login(login_data) => unsafe {
                        UID = login_data.uid;
                    },
                    AccountRoute::Logout(_) => {}
                },
                Packet::Game(game_route) => {
                    match game_route {
                        GameRoute::Update(update_data) => {
                            let health_now = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            'update_data: for rigid_body_state in update_data.states {
                                for (mut syn_entity, mut transform) in syn_entity_query.iter_mut() {
                                    // println!("3");
                                    if syn_entity.id == rigid_body_state.id.into() {
                                        *transform = Transform {
                                            translation: Vec3::new(
                                                rigid_body_state.translation.0
                                                    * window.tile_width_proportion,
                                                rigid_body_state.translation.1
                                                    * window.tile_height_proportion,
                                                99.0,
                                            ),
                                            rotation: Quat::from_rotation_z(
                                                rigid_body_state.rotation,
                                            ),
                                            scale: Vec3::new(1., 1., 1.),
                                        };
                                        syn_entity.health = health_now;
                                        unsafe {
                                            if rigid_body_state.entity_type == 1
                                                && UID == rigid_body_state.id as u32
                                            {
                                                if let Some((mut camera_transform, _)) =
                                                    camera_query.iter_mut().next()
                                                {
                                                    camera_transform.translation = Vec3::new(
                                                        rigid_body_state.translation.0
                                                            * window.tile_width_proportion,
                                                        rigid_body_state.translation.1
                                                            * window.tile_height_proportion,
                                                        99.0,
                                                    );
                                                }
                                            }
                                        }
                                        continue 'update_data;
                                    }
                                }
                                // println!("4");

                                // 未生成的实体根据实体类型生成新实体
                                let mut texture_handle = asset_server.load("textures/chars/0.png");
                                let mut tile_size =
                                    Vec2::new(window.tile_width * 1f32, window.tile_height * 1f32);

                                match rigid_body_state.entity_type {
                                    // tile
                                    0 => {
                                        texture_handle = asset_server.load(
                                            format!(
                                                "textures/tile/{}.png",
                                                rigid_body_state.texture.0
                                            )
                                            .as_str(),
                                        );
                                    }
                                    // 玩家实体
                                    1 => {
                                        texture_handle = asset_server.load(
                                            format!(
                                                "textures/chars/{}.png",
                                                rigid_body_state.texture.0
                                            )
                                            .as_str(),
                                        );
                                        tile_size *= 2f32;
                                    }
                                    // 可动实体
                                    2 => {
                                        texture_handle = asset_server.load(
                                            format!(
                                                "textures/movable/{}.png",
                                                rigid_body_state.texture.0
                                            )
                                            .as_str(),
                                        );
                                        tile_size = Vec2::new(
                                            window.tile_width * 0.5f32,
                                            window.tile_width * 1f32,
                                        );
                                    }
                                    // 不可动实体
                                    3 => {
                                        texture_handle = asset_server.load(
                                            format!(
                                                "textures/unmovable/{}.png",
                                                rigid_body_state.texture.0
                                            )
                                            .as_str(),
                                        );
                                    }
                                    // 其它
                                    _ => {}
                                }

                                let scale = Vec3::new(1., 1., 0.);

                                let texture_atlas = TextureAtlas::from_grid(
                                    texture_handle,
                                    tile_size,
                                    rigid_body_state.texture.1.into(),
                                    1,
                                );
                                let texture_atlas_handle = texture_atlases.add(texture_atlas);
                                commands
                                    .spawn_bundle(SpriteSheetBundle {
                                        texture_atlas: texture_atlas_handle,
                                        transform: Transform {
                                            translation: Vec3::new(
                                                rigid_body_state.translation.0
                                                    * window.tile_width_proportion,
                                                rigid_body_state.translation.1
                                                    * window.tile_height_proportion,
                                                99.0,
                                            ),
                                            rotation: Quat::from_rotation_z(
                                                rigid_body_state.rotation,
                                            ),
                                            scale,
                                        },
                                        ..Default::default()
                                    })
                                    .insert(Timer::from_seconds(0.1, true))
                                    .insert(SynEntity {
                                        id: rigid_body_state.id.into(),
                                        health: SystemTime::now()
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    });
                            }
                        }
                        GameRoute::Control(_control_data) => {}
                    }
                    // 游戏逻辑数据包每帧执行一次
                    return;
                }
            }
        }
    }
}

async fn net_client_start(
    packet_queue: Arc<Mutex<Vec<Packet>>>,
    to_be_sent_queue: Arc<Mutex<Vec<Packet>>>,
) -> io::Result<()> {
    // 连接服务器
    println!("客户端网络连接ing...");
    let sock = UdpSocket::bind(common::config::CLIENT_ADDR).await?;
    let remote_addr = common::config::SERVER_ADDR;
    sock.connect(remote_addr).await?;
    println!("客户端网络连接成功: {:?}", sock.local_addr());
    let r = Arc::new(sock);
    let s = r.clone();
    let s1 = r.clone();

    // 登录服务器
    s.send(
        &bincode::serialize(&Packet::Account(AccountRoute::Login(AccountData {
            uid: 0,
            group: 0,
        })))
        .unwrap()[0..],
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f64(1f64));
        loop {
            interval.tick().await;
            s1.send(
                &bincode::serialize(&Packet::Heartbeat(HeartbeatRoute::Keep(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                )))
                .unwrap()[0..],
            )
            .await
            .unwrap();
            // println!("发送Heartbeat");
        }
    });

    let mut buf = [0; 2048];
    loop {
        // println!("接收ing");
        if let Ok(len) = r.recv(&mut buf).await {
            // println!("接收来自服务器的 {:?} bytes", len);
            let packet = bincode::deserialize(&buf[..len]);
            // 转发事件
            if let Ok(packet) = packet {
                if let Ok(mut packet_queue) = packet_queue.lock() {
                    packet_queue.push(packet);
                }
            }
        }

        if let Ok(mut to_be_sent_queue) = to_be_sent_queue.lock() {
            // println!("0");
            let to_be_sent_queue_c = to_be_sent_queue.clone();
            to_be_sent_queue.clear();
            for to_be_sent_packet in to_be_sent_queue_c.iter() {
                // println!("1");
                let s = s.clone();
                let to_be_sent_packet = to_be_sent_packet.clone();
                tokio::spawn(async move {
                    // println!("2");
                    s.send(&bincode::serialize(&to_be_sent_packet).unwrap()[0..])
                        .await
                        .unwrap();
                    // println!("3");
                });
            }
        }
    }
}
