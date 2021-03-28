use std::{error::Error, sync::Arc};

use common::{GameEvent, UpdateData};
use rapier2d::dynamics::{
    BodyStatus, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet,
};
use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase, SharedShape};
use rapier2d::na::Vector2;
use rapier2d::pipeline::PhysicsPipeline;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

type ColliderState = Arc<Mutex<ColliderSet>>;
type RigidBodyState = Arc<Mutex<RigidBodySet>>;

pub async fn engine_start(
    engine_tx: Sender<GameEvent>,
    net_rx: Receiver<GameEvent>,
) -> Result<(), Box<dyn Error>> {
    let rigid_body_state = Arc::new(Mutex::new(RigidBodySet::new()));
    let collider_state = Arc::new(Mutex::new(ColliderSet::new()));

    // 监听网络模块传过来的消息
    let net_future = wait_for_net(net_rx, rigid_body_state.clone(), collider_state.clone());

    // 物理引擎主循环
    let engine_future =
        engine_main_loop(engine_tx, rigid_body_state.clone(), collider_state.clone());
    println!("物理引擎已启动!");
    let _ = tokio::join!(net_future, engine_future);
    Ok(())
}

pub async fn engine_main_loop(
    engine_tx: Sender<GameEvent>,
    rigid_body_state: RigidBodyState,
    collider_state: ColliderState,
) -> Result<(), Box<dyn Error>> {
    // 物理引擎初始化配置
    let mut pipeline = PhysicsPipeline::new();
    // 世界重力
    let gravity = Vector2::new(0.0, -100.0);
    //
    let integration_parameters = IntegrationParameters::default();
    //
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    // 碰撞体集合
    // let mut colliders = ColliderSet::new();
    // 连接体集合
    let mut joints = JointSet::new();
    // 物理钩子
    let physics_hooks = ();
    // 事件处理器
    let event_handler = ();

    // 世界初始化物体
    create_object(rigid_body_state.clone(), collider_state.clone()).await;

    // 物理引擎主循环
    // let start_time = Instant::now();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_nanos(
        (1f64 / 60f64 * 1000000000f64) as u64,
    ));
    loop {
        interval.tick().await;
        let mut bodies = &mut rigid_body_state.lock().await;
        let mut colliders = &mut collider_state.lock().await;

        // 运行物理引擎计算世界
        pipeline.step(
            &gravity,
            &integration_parameters,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            &physics_hooks,
            &event_handler,
        );

        // 处理运行后结果世界状态
        for (_colloder_handle, collider) in colliders.iter() {
            if let Some(body) = bodies.get(collider.parent()) {
                // 只更新在运动的物体
                if body.is_moving()
                    && (body.linvel().amax().abs() >= 0.0001f32 || body.angvel().abs() >= 0.0001f32)
                {
                    // println!(
                    //     "{:?} (位置: {:?}, 旋转: {:?}, 线速度: {:?}, 角速度: {:?})",
                    //     body.user_data,
                    //     collider.position().translation,
                    //     collider.position().rotation,
                    //     body.linvel(),
                    //     body.angvel()
                    // );
                    let packet = GameEvent::Update(UpdateData {
                        id: body.user_data,
                        translation: [
                            collider.position().translation.x,
                            collider.position().translation.y,
                        ],
                        rotation: [
                            collider.position().rotation.re,
                            collider.position().rotation.im,
                        ],
                    });
                    let _ = engine_tx.send(packet).await;
                }
            }
        }
    }
    // let time = start_time.elapsed().as_secs_f64();
    // println!("{}", time);
}

async fn create_object(rigid_body_state: RigidBodyState, collider_state: ColliderState) {
    let bodies = &mut rigid_body_state.lock().await;
    let colliders = &mut collider_state.lock().await;
    // 地面
    // 刚体类型
    let rigid_body = RigidBodyBuilder::new(BodyStatus::Static)
        .translation(0.0, -10.0)
        .build();
    // 碰撞体类型
    let collider = ColliderBuilder::new(SharedShape::cuboid(5000.0, 10.0))
        // 摩擦
        .friction(0.0)
        // 是否为传感器
        // .sensor(true)
        .build();
    let rb_handle = bodies.insert(rigid_body);
    colliders.insert(collider, rb_handle, bodies);

    // 球
    // 刚体类型
    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
        .translation(0.0, 50.0)
        // .rotation(0.0)
        // .position(Isometry2::new(Vector2::new(1.0, 5.0), 0.0))
        // 线速度
        .linvel(0.0, 0.0)
        // 角速度
        .angvel(0.0)
        // 重力
        .gravity_scale(10.0)
        // .can_sleep(true)
        .user_data(0)
        .build();
    // 碰撞体类型
    let collider = ColliderBuilder::new(SharedShape::ball(5.0))
        // 密度
        .density(1.0)
        // 摩擦
        .friction(0.0)
        // 是否为传感器
        // .sensor(true)
        .build();
    let rb_handle = bodies.insert(rigid_body);

    colliders.insert(collider, rb_handle, bodies);
}

async fn wait_for_net(
    mut net_rx: Receiver<GameEvent>,
    rigid_body_state: RigidBodyState,
    collider_state: ColliderState,
) {
    // let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(100));
    loop {
        // interval.tick().await;
        if let Some(game_event) = net_rx.recv().await {
            let bodies = &mut rigid_body_state.lock().await;
            let colliders = &mut collider_state.lock().await;
            match game_event {
                GameEvent::Login(login_data) => {
                    let mut body_id = 0u128;
                    if let Some((_last_handle, last_body)) = bodies.iter().last() {
                        body_id = last_body.user_data + 1u128;
                    }
                    println!("uid: {}", body_id);
                    // 球
                    // 刚体类型
                    let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
                        .translation(2.0, 60.0)
                        // 线速度
                        .linvel(0.0, 0.0)
                        // 角速度
                        .angvel(0.0)
                        // 重力
                        .gravity_scale(10.0)
                        .user_data(body_id)
                        .build();
                    // 碰撞体类型
                    let collider = ColliderBuilder::new(SharedShape::ball(2.0))
                        // 密度
                        .density(1.0)
                        // 摩擦
                        .friction(0.0)
                        .build();
                    let rb_handle = bodies.insert(rigid_body);
                    colliders.insert(collider, rb_handle, bodies);
                }
                _ => {}
            }
        }
    }
}
