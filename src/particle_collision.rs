use bevy::prelude::*;
use bevy_xpbd::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(XPBPlugin::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_startup_system(startup)
        .run();
}
