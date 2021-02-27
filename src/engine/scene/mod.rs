pub mod breakout;

use bevy::prelude::*;

use self::breakout::BreakOut;

#[derive(Clone)]
pub enum Route {
    Index,
    BreakOut(BreakOut),
}

pub struct Scene;
impl Plugin for Scene {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Routes::new())
            .add_event::<RouteEvent>()
            .add_system(route_event_listener_system.system());
    }
}

pub struct Routes {
    routes: Vec<Route>,
    current: usize,
    before: usize,
}

impl Routes {
    fn new() -> Routes {
        Routes {
            routes: vec![Route::Index],
            current: 0,
            before: 0,
        }
    }

    fn goto(&mut self, route: Route) {
        self.routes.push(route);
        self.before = self.current;
        self.current += 1;
    }

    fn back(&mut self) {
        if self.current > 0 {
            self.before = self.current;
            self.current -= 1;
        }
    }

    fn go_ahead(&mut self) {
        if self.current + 1 < self.routes.len() {
            self.before = self.current;
            self.current += 1;
        }
    }
}

// 路由事件
pub struct RouteEvent {
    pub routes: Routes,
}

fn route_event_listener_system(
    commands: &mut Commands,
    mut route_event_reader: Local<EventReader<RouteEvent>>,
    route_events: Res<Events<RouteEvent>>,
    route_query: Query<(Entity, &Route)>,
) {
    for route_event in route_event_reader.iter(&route_events) {
        match &route_event.routes.routes[route_event.routes.before] {
            Route::BreakOut(..) => {
                for (entity, route) in route_query.iter() {
                    if let Route::BreakOut(..) = route {
                        commands.despawn_recursive(entity);
                    }
                }
            }
            Route::Index => {}
        }
        match &route_event.routes.routes[route_event.routes.current] {
            Route::BreakOut(plugin) => {
                // app.add_plugin(plugin.clone());
            }
            Route::Index => {}
        }
    }
}
