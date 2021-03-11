use bevy::prelude::*;

use super::{
    event::{
        map_event::MapEventPlugin, move_event::MoveEventPlugin, window_event::WindowEventPlugin,
    },
    plugin::{
        camera_ctrl::CameraCtrl, clipboard::Clipboard, fps::Fps, player::PlayerPlugin,
        tile_map::TileMapPlugin,
    },
};

pub fn engine_start() {
    App::build()
        .add_resource(WindowDescriptor {
            title: String::from("初始游戏"),
            // 垂直同步
            vsync: true,
            // 是否可调整窗口大小
            resizable: false,
            // 是否有窗口外壳
            decorations: true,
            width: 960f32,
            height: 540f32,
            // 窗口模式
            // mode: WindowMode::BorderlessFullscreen,
            // 鼠标隐藏并锁定
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 8 })
        // 默认插件
        .add_plugins(DefaultPlugins)
        // 物理插件
        // 物理
        // .add_startup_system(setup_physics.system())
        // .add_system(print_events.system())
        // 设置摄像机
        .add_startup_system(set_camera.system())
        // 辅助功能插件
        .add_plugin(Fps)
        .add_plugin(Clipboard)
        // 事件
        .add_plugin(MoveEventPlugin)
        .add_plugin(MapEventPlugin)
        .add_plugin(WindowEventPlugin)
        // 地图初始化
        .add_plugin(TileMapPlugin)
        // 玩家
        .add_plugin(PlayerPlugin)
        .run();
}

fn set_camera(commands: &mut Commands) {
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .with(CameraCtrl)
        .spawn(CameraUiBundle::default());
}

// fn setup_physics(commands: &mut Commands) {
//     // Static rigid-body with a cuboid shape.
//     let rigid_body1 = RigidBodyBuilder::new_static();
//     let collider1 = ColliderBuilder::cuboid(10.0, 1.0);
//     // Keep the entity identifier.
//     let entity1 = commands
//         .spawn((rigid_body1, collider1))
//         .current_entity()
//         .unwrap();

//     // Dynamic rigid-body with ball shape.
//     let rigid_body2 = RigidBodyBuilder::new_dynamic().translation(0.0, 3.0);
//     let collider2 = ColliderBuilder::ball(0.5);
//     // Keep the entity identifier.
//     let entity2 = commands
//         .spawn((rigid_body2, collider2))
//         .current_entity()
//         .unwrap();

//     // Create the joint.
//     let joint_params = BallJoint::new(Point2::origin(), Point2::new(0.0, -3.0));
//     let joint_builder_component = JointBuilderComponent::new(joint_params, entity1, entity2);
//     commands.spawn((joint_builder_component,));
// }

// fn print_events(events: Res<EventQueue>) {
//     while let Ok(intersection_event) = events.intersection_events.pop() {
//         println!("Received intersection event: {:?}", intersection_event);
//     }

//     while let Ok(contact_event) = events.contact_events.pop() {
//         println!("Received contact event: {:?}", contact_event);
//     }
// }

// pub fn run_snake() {
//     snake();
// }
