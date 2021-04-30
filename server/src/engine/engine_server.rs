use std::{collections::HashMap, error::Error, sync::Arc};

use crate::net;
use data::server_db::{self, GameData};
use protocol::{
    data::update_data::{EntityState, UpdateData},
    packet::Packet,
    route::GameRoute,
};
use rapier2d::dynamics::{
    BodyStatus, CCDSolver, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodyHandle,
    RigidBodySet,
};
use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase, SharedShape};
use rapier2d::na::Vector2;
use rapier2d::pipeline::PhysicsPipeline;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

type ColliderSetState = Arc<Mutex<ColliderSet>>;
type RigidBodySetState = Arc<Mutex<RigidBodySet>>;
type JointSetState = Arc<Mutex<JointSet>>;
type PlayerHandleMapState = Arc<Mutex<HashMap<u32, RigidBodyHandle>>>;

pub async fn engine_start() -> Result<(), Box<dyn Error>> {
    let (net_tx, net_rx) = mpsc::channel::<Packet>(100);
    let (engine_tx, engine_rx) = mpsc::channel::<Packet>(100);

    // let _ = tokio::join!(net_server);

    let rigid_body_state = Arc::new(Mutex::new(RigidBodySet::new()));
    let collider_state = Arc::new(Mutex::new(ColliderSet::new()));
    let joint_state = Arc::new(Mutex::new(JointSet::new()));

    let player_handle_map: HashMap<u32, RigidBodyHandle> = HashMap::new();
    let player_handle_state = Arc::new(Mutex::new(player_handle_map));

    // 物理引擎主循环
    let engine_future = engine_main_loop(
        engine_tx.clone(),
        rigid_body_state.clone(),
        collider_state.clone(),
        joint_state.clone(),
    );
    tokio::spawn(async move { engine_future.await.unwrap() });

    let clean_body_future = clean_body(
        rigid_body_state.clone(),
        collider_state.clone(),
        joint_state.clone(),
    );
    tokio::spawn(async move { clean_body_future.await });

    // 监听网络模块传过来的消息
    let net_future = wait_for_net(
        engine_tx,
        net_rx,
        rigid_body_state.clone(),
        collider_state.clone(),
        joint_state.clone(),
        player_handle_state,
    );
    tokio::spawn(async move { net_future.await });

    // 网络监听
    let net_server = net::net_server::start_server(net_tx, engine_rx);

    let _ = net_server.await;
    Ok(())
}

pub async fn engine_main_loop(
    engine_tx: Sender<Packet>,
    rigid_body_state: RigidBodySetState,
    collider_state: ColliderSetState,
    joint_state: JointSetState,
) -> Result<(), Box<dyn Error>> {
    println!("物理引擎已启动!");
    // 物理引擎初始化配置
    let mut pipeline = PhysicsPipeline::new();
    // 世界重力
    let gravity = Vector2::new(0.0, 0.0);
    //
    let integration_parameters = IntegrationParameters::default();
    //
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    // 碰撞体集合
    // let mut colliders = ColliderSet::new();
    // 连接体集合
    // let mut joints = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    // 物理钩子
    let physics_hooks = ();
    // 事件处理器
    let event_handler = ();

    // 世界初始化物体
    create_object(rigid_body_state.clone(), collider_state.clone()).await;

    // 物理引擎主循环
    // let start_time = Instant::now();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f64(1f64 / 60f64));
    let mut frame_no: u128 = 0;
    // println!("main_0");
    loop {
        // println!("main_1");
        // println!("{}", &frame_no);
        interval.tick().await;
        // println!("main_2");
        let mut bodies = &mut rigid_body_state.lock().await;
        let mut colliders = &mut collider_state.lock().await;
        let mut joints = &mut joint_state.lock().await;
        // 运行物理引擎计算世界
        pipeline.step(
            &gravity,
            &integration_parameters,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        // 处理运行后结果世界状态
        let mut states = Vec::new();
        for (_colloder_handle, collider) in colliders.iter() {
            if let Some(body) = bodies.get(collider.parent()) {
                // 更新所有动态物体
                if body.is_dynamic()
                // 只更新在运动的物体
                // if body.is_moving()
                //     && (body.linvel().amax().abs() >= 0.0001f32 || body.angvel().abs() >= 0.0001f32)
                {
                    let mut state = EntityState {
                        id: body.user_data as u64,
                        translation: (
                            collider.position().translation.x,
                            collider.position().translation.y,
                        ),
                        rotation: collider.position().rotation.angle(),
                        linvel: (body.linvel().x, body.linvel().y),
                        angvel: (body.angvel(), body.angvel()),
                        texture: (0, 0, 0),
                        entity_type: 0,
                    };
                    state.make_up_data(body.user_data);
                    states.push(state);
                }
            }
        }
        let packet = Packet::Game(GameRoute::Update(UpdateData {
            frame: frame_no,
            states,
        }));
        let _ = engine_tx.send(packet.clone()).await;
        // println!("{:?}", &packet);
        frame_no += 1;
    }
    // let time = start_time.elapsed().as_secs_f64();
    // println!("{}", time);
}

async fn clean_body(
    rigid_body_state: RigidBodySetState,
    collider_state: ColliderSetState,
    joint_state: JointSetState,
) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs_f64(1f64));
    loop {
        interval.tick().await;
        let bodies = &mut rigid_body_state.lock().await;
        let colliders = &mut collider_state.lock().await;
        let joints = &mut joint_state.lock().await;

        let mut handles_for_remove = Vec::new();

        'bodies_iter: for (body_handle, body) in bodies.iter_mut() {
            if body.position().translation.x.abs() > 9999f32
                || body.position().translation.y.abs() > 9999f32
            {
                handles_for_remove.push(body_handle);
                continue;
            }
            // 判断玩家实体是否还在线
            let mut entity_state = EntityState {
                id: 0,
                translation: (0., 0.),
                rotation: 0.,
                linvel: (0., 0.),
                angvel: (0., 0.),
                texture: (0, 0, 0),
                entity_type: 0,
            };
            entity_state.make_up_data(body.user_data);

            if entity_state.entity_type == 1 {
                match server_db::find(GameData::player_online(None)) {
                    Ok(data) => {
                        if data.len() > 0 {
                            let uid_list: Vec<&str> = data.split(",").collect();
                            for index in 0..uid_list.len() {
                                if uid_list[index].eq(&entity_state.id.to_string()) {
                                    continue 'bodies_iter;
                                }
                            }
                        }
                        handles_for_remove.push(body_handle);
                    }
                    Err(_) => {}
                }
            }
        }

        for handle in handles_for_remove {
            println!("清除(过界/离线)实体: {:?}", &handle);
            bodies.remove(handle, colliders, joints);
            println!("剩余实体: {:?}", &bodies.len());
        }
    }
}

async fn create_object(rigid_body_state: RigidBodySetState, collider_state: ColliderSetState) {
    let bodies = &mut rigid_body_state.lock().await;
    let colliders = &mut collider_state.lock().await;

    // 旋转体
    // 刚体类型
    let rb_state = EntityState {
        id: 1000,
        translation: (0., 0.),
        rotation: 0.,
        linvel: (0., 0.),
        angvel: (0., 0.),
        texture: (0, 5, 1),
        entity_type: 2,
    };
    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .translation(0.0, 200.0)
        // .rotation(0.0)
        // .position(Isometry2::new(Vector2::new(1.0, 5.0), 0.0))
        // 线速度
        .linvel(0.0, 0.0)
        // 角速度
        .angvel(1.0)
        // 重力
        .gravity_scale(0.0)
        // .can_sleep(true)
        .user_data(rb_state.get_data())
        .build();
    // 碰撞体类型
    let collider = ColliderBuilder::new(SharedShape::ball(5.0))
        // 密度
        .density(1.0)
        // 摩擦
        .friction(1.0)
        // 是否为传感器
        // .sensor(true)
        .build();
    let rb_handle = bodies.insert(rigid_body);

    colliders.insert(collider, rb_handle, bodies);
}

async fn wait_for_net(
    engine_tx: Sender<Packet>,
    mut net_rx: Receiver<Packet>,
    rigid_body_state: RigidBodySetState,
    collider_state: ColliderSetState,
    _joint_state: JointSetState,
    player_handle_state: PlayerHandleMapState,
) {
    // let mut entity_id: u64 = 1000;
    // let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(100));
    loop {
        // interval.tick().await;
        if let Some(game_event) = net_rx.recv().await {
            let bodies = &mut rigid_body_state.lock().await;
            let colliders = &mut collider_state.lock().await;
            let player_handle_map = &mut player_handle_state.lock().await;
            match game_event {
                // 玩家登录生成角色
                Packet::Account(account_route) => match account_route {
                    protocol::route::AccountRoute::Login(login_data) => {
                        println!("玩家加入: {}", &login_data.uid);
                        // 球
                        // 刚体类型
                        let rb_state = EntityState {
                            id: login_data.uid as u64,
                            translation: (0., 0.),
                            rotation: 0.,
                            linvel: (0., 0.),
                            angvel: (0., 0.),
                            texture: (1, 4, 3),
                            entity_type: 1,
                        };
                        let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
                            .translation(0.0, 100.0)
                            // 线速度
                            .linvel(0.0, 0.0)
                            // 角速度
                            .angvel(0.0)
                            // 重力
                            .gravity_scale(1.0)
                            .lock_rotations()
                            .user_data(rb_state.get_data())
                            .build();
                        // 碰撞体类型
                        let collider = ColliderBuilder::cuboid(32.0, 64.0)
                            // 密度
                            .density(1.0)
                            // 摩擦
                            .friction(0.0)
                            .build();
                        let rb_handle = bodies.insert(rigid_body);
                        colliders.insert(collider, rb_handle, bodies);
                        player_handle_map.insert(login_data.uid, rb_handle);
                        // println!("{:?}", player_handle_map);
                        // entity_id += 1;

                        // 发送当前所有可移动实体状态给新登录玩家
                        let mut states = Vec::new();
                        for (_colloder_handle, collider) in colliders.iter() {
                            if let Some(body) = bodies.get(collider.parent()) {
                                // 只更新可运动的物体
                                if body.is_dynamic() {
                                    let mut state = EntityState {
                                        id: body.user_data as u64,
                                        translation: (
                                            collider.position().translation.x,
                                            collider.position().translation.y,
                                        ),
                                        rotation: collider.position().rotation.angle(),
                                        linvel: (body.linvel().x, body.linvel().y),
                                        angvel: (body.angvel(), body.angvel()),
                                        texture: (0, 0, 0),
                                        entity_type: 0,
                                    };
                                    state.make_up_data(body.user_data);
                                    states.push(state);
                                }
                            }
                        }
                        let packet =
                            Packet::Game(GameRoute::Update(UpdateData { frame: 0, states }));
                        let _ = engine_tx.send(packet.clone()).await;
                        let _ = engine_tx.send(packet.clone()).await;
                    }
                    protocol::route::AccountRoute::Logout(_) => {}
                    protocol::route::AccountRoute::GetInfo(_) => {}
                },
                // 玩家控制
                Packet::Game(game_route) => match game_route {
                    protocol::route::GameRoute::Control(control_data) => {
                        // println!("1");
                        if let Some(handle) = player_handle_map.get(&control_data.uid) {
                            // println!("2");
                            if let Some(body) = bodies.get_mut(*handle) {
                                body.set_linvel(
                                    Vector2::new(
                                        control_data.direction.0 * 100f32,
                                        control_data.direction.1 * 100f32,
                                    ),
                                    true,
                                );
                                // println!("控制移动");
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
