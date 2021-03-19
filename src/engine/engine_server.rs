use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase, SharedShape};
use rapier2d::na::Vector2;
use rapier2d::pipeline::PhysicsPipeline;
use rapier2d::{
    dynamics::{BodyStatus, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet},
    na::Isometry2,
};

pub fn engine_start() {
    // 物理引擎初始化配置
    let mut pipeline = PhysicsPipeline::new();
    // 世界重力
    let gravity = Vector2::new(0.0, -9.81);
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

    let mut colloder_handle = None;
    // 物理引擎主循环
    for i in 0..10 {
        if i == 0 {
            // 刚体类型
            let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
                .translation(1.0, 5.0)
                .rotation(0.0)
                .position(Isometry2::new(Vector2::new(1.0, 5.0), 0.0))
                // 线速度
                .linvel(0.0, 0.0)
                // 角速度
                .angvel(0.0)
                // 重力
                .gravity_scale(1.0)
                .can_sleep(true)
                .build();
            // 碰撞体类型
            let collider = ColliderBuilder::new(SharedShape::ball(0.5))
                // 密度
                .density(1.3)
                // 摩擦
                .friction(0.8)
                // 是否为传感器
                .sensor(true)
                .build();
            let rb_handle = bodies.insert(rigid_body);
            colloder_handle = Some(colliders.insert(collider, rb_handle, &mut bodies));
        }
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
        if let Some(ch) = colloder_handle {
            let collider = colliders.get(ch).unwrap();
            let rb = bodies.get(collider.parent()).unwrap();
            println!("刚体位置: {}", rb.position().translation);
            println!("碰撞体位置: {}", collider.position().translation);
        }
    }
}
