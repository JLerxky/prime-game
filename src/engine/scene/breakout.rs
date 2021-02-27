use bevy::{
    prelude::*,
    reflect::TypeRegistry,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use super::super::event::my_event::MyEvent;

use super::Route;

#[derive(Clone)]
pub struct BreakOut;

impl Plugin for BreakOut {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<Paddle>()
            .register_type::<Ball>()
            .register_type::<Scoreboard>()
            .add_resource(Scoreboard { score: 0f32 })
            .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            // .add_startup_system(save_scene_system.system())
            .add_startup_system(setup.system())
            .add_system(paddle_movement_system.system())
            .add_system(ball_collision_system.system())
            .add_system(ball_movement_system.system())
            .add_system(scoreboard_system.system());
    }
}

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Paddle {
    speed: f32,
}

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Ball {
    velocity: Vec3,
}

#[derive(Reflect, Default)]
#[reflect(Component)]
struct Scoreboard {
    score: f32,
}
enum Collider {
    Solid,
    Scorable,
    Paddle,
    Death,
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 标记Route页
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with(Route::BreakOut)
        .current_entity()
        .unwrap();
    // paddle
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(Paddle { speed: 500.0 })
        .with(Collider::Paddle)
        .with(Parent(root));

    // ball
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        })
        .with(Parent(root));
    // scoreboard
    commands
        .spawn(TextBundle {
            text: Text {
                font: asset_server.load("fonts/YouZai.ttf"),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    font_size: 40.0,
                    ..Default::default()
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Parent(root));

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    // left
    commands
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        .with(Parent(root));
    // right
    commands
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        .with(Parent(root));
    // bottom
    commands
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Death)
        .with(Parent(root));
    // top
    commands
        .spawn(SpriteBundle {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        .with(Parent(root));

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            commands
                // brick
                .spawn(SpriteBundle {
                    material: brick_material.clone(),
                    sprite: Sprite::new(brick_size),
                    transform: Transform::from_translation(brick_position),
                    ..Default::default()
                })
                .with(Collider::Scorable)
                .with(Parent(root));
        }
    }
}

fn save_scene_system(
    // _commands: &mut Commands,
    _world: &mut World,
    resources: &mut Resources,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    // 可以从任何ECS世界创建场景。您可以为场景创建一个新场景，也可以使用当前世界。
    let mut world = World::new();
    // paddle
    world.spawn((
        SpriteBundle {
            // material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        },
        Paddle { speed: 500.0 },
        Collider::Paddle,
    ));

    // ball
    world.spawn((
        SpriteBundle {
            // material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        },
        Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        },
    ));
    // scoreboard
    world.spawn(TextBundle {
        text: Text {
            // font: asset_server.load("fonts/YouZai.ttf"),
            value: "Score:".to_string(),
            style: TextStyle {
                color: Color::rgb(0.5, 0.5, 1.0),
                font_size: 40.0,
                ..Default::default()
            },
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    // Add walls
    // let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    // left
    world.spawn((
        SpriteBundle {
            // material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        Collider::Solid,
    ));
    // right
    world.spawn((
        SpriteBundle {
            // material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        Collider::Solid,
    ));
    // bottom
    world.spawn((
        SpriteBundle {
            // material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        Collider::Death,
    ));
    // top
    world.spawn((
        SpriteBundle {
            // material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        Collider::Solid,
    ));

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    // let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            world
                // brick
                .spawn((
                    SpriteBundle {
                        // material: brick_material.clone(),
                        sprite: Sprite::new(brick_size),
                        transform: Transform::from_translation(brick_position),
                        ..Default::default()
                    },
                    Collider::Scorable,
                ));
        }
    }
    // TypeRegistry包含了所有注册的类型信息，用来生成场景。
    let type_registry = resources.get::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(&world, &type_registry);

    // 场景序列化
    let scene_string = scene.serialize_ron(&type_registry).unwrap();
    println!("{}", &scene_string);

    // 文件保存场景信息
    write_into_file("assets/scenes/breakout.scn", scene_string);
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn write_into_file(path: &str, contents: String) {
    let path = Path::new(path);
    let display = path.display();

    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("无法创建 {}: {}", display, why),
        Ok(file) => file,
    };

    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(contents.as_bytes()) {
        Err(why) => {
            panic!("无法写入 {}: {}", display, why)
        }
        Ok(_) => println!("写入成功 {}", display),
    }
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction * paddle.speed;
        // bound the paddle within the walls
        translation.x = translation.x.min(380.0).max(-380.0);
    }
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (ball, mut transform) in ball_query.iter_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        if text.value.contains("Score:") {
            text.value = format!("Score: {}", scoreboard.score);
        }
    }
}

fn ball_collision_system(
    commands: &mut Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    mut my_events: ResMut<Events<MyEvent>>,
) {
    for (mut ball, ball_transform, sprite) in ball_query.iter_mut() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        // check collision with walls
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    scoreboard.score += 1f32;
                    commands.despawn(collider_entity);
                    my_events.send(MyEvent {
                        message: "+1分".to_string(),
                    });
                }
                if let Collider::Death = *collider {
                    scoreboard.score -= 1f32;
                    my_events.send(MyEvent {
                        message: "-1分".to_string(),
                    });
                }

                // 碰撞时反弹球
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                // 如果我们在x轴上碰到某物，则把x轴速度取反
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // 如果我们在y轴上碰到某物，则把y轴速度取反
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                // 如果此碰撞在实体上，则中断，否则继续检查实体是否也在碰撞中
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}
