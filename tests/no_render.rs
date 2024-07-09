use std::backtrace::Backtrace;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}

#[test]
fn plugin_does_not_crash_without_render() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);

    app.add_plugins(AssetPlugin::default());
    app.add_plugins(ImagePlugin::default());
    app.add_plugins(LdtkPlugin);

    app.add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0));

    let mut num_entities = 0;
    loop {
        app.update();

        num_entities = app
            .world
            .query::<&LevelIid>()
            .iter(&app.world)
            .count();

        println!("Backtrace: {}", Backtrace::force_capture());
        if num_entities != 0 {
            break;
        }
    }

    assert_ne!(num_entities, 0);
}
