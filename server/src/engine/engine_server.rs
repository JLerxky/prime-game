use rapier2d::dynamics::{
    BodyStatus, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet,
};
use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase, SharedShape};
use rapier2d::na::Vector2;
use rapier2d::pipeline::PhysicsPipeline;
use std::time::Instant;

pub async fn engine_start() {
    // 物理引擎初始化配置
    let mut pipeline = PhysicsPipeline::new();
    // 世界重力
    let gravity = Vector2::new(0.0, -100.0);
    //
    let integration_parameters = IntegrationParameters::default();
    //
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    // 刚体集合
    let mut bodies = RigidBodySet::new();
    // 碰撞体集合
    let mut colliders = ColliderSet::new();
    // 连接体集合
    let mut joints = JointSet::new();
    // 物理钩子
    let physics_hooks = ();
    // 事件处理器
    let event_handler = ();

    // 世界初始化物体
    create_object(&mut bodies, &mut colliders);

    // 物理引擎主循环
    let start_time = Instant::now();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_nanos((1f64 / 60f64 * 1000000000f64) as u64));
    for _i in 0..60 {
        interval.tick().await;

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
        for (colloder_handle, collider) in colliders.iter() {
            if let Some(body) = bodies.get(collider.parent()) {
                // 只更新在运动的物体
                if body.is_moving() && (body.linvel().amax().abs() >= 0.0001f32 || body.angvel().abs() >= 0.0001f32) {
                    println!(
                        "{:?} 位置: {:?}, 旋转: {:?}, 线速度: {:?}, 角速度: {:?}",
                        colloder_handle,
                        collider.position().translation,
                        collider.position().rotation,
                        body.linvel(),
                        body.angvel()
                    );
                }
            }
        }
    }
    let time = start_time.elapsed().as_secs_f64();
    println!("{}", time);
}

fn create_object(bodies: &mut RigidBodySet, colliders: &mut ColliderSet) {
    // 地面
    // 刚体类型
    let rigid_body = RigidBodyBuilder::new(BodyStatus::Static)
        .translation(0.0, -10.0)
        .build();
    // 碰撞体类型
    let collider = ColliderBuilder::new(SharedShape::cuboid(5000.0, 5.0))
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
        .gravity_scale(1.0)
        .can_sleep(true)
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
