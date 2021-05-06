use bevy::prelude::*;
use data::client_db::save_player;
use protocol::data::player_data::{PlayerData, PlayerListData};

pub struct PlayerUpdateEventPlugin;

impl Plugin for PlayerUpdateEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PlayerUpdateEvent>()
            .add_system(event_listener_system.system());
    }
}

pub struct PlayerUpdateEvent {
    pub player_list_data: PlayerListData,
}

fn event_listener_system(
    mut event_reader: EventReader<PlayerUpdateEvent>,
    mut player_bar_query: Query<(&mut PlayerData, &mut Transform), With<PlayerData>>,
) {
    for event in event_reader.iter() {
        'player: for player_data in &event.player_list_data.players {
            let _ = save_player(player_data.clone());
            for (mut old_player_data, mut transform) in player_bar_query.iter_mut() {
                if player_data.uid == old_player_data.uid {
                    *old_player_data = *player_data;
                    // println!("{:?}", &player_data);
                    let blood_len = 12. * (player_data.hp as f32 / player_data.max_hp as f32);
                    *transform = Transform {
                        translation: Vec3::new((blood_len / 2.) - 6., 12., 99.0),
                        scale: Vec3::new(blood_len / 4., 0.1, 0.),
                        ..Default::default()
                    };
                    continue 'player;
                }
            }
        }
    }
}
