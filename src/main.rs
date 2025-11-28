use bevy::prelude::*;


fn main(){
    App::new()
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(
        DefaultPlugins.set(AssetPlugin{
            file_path: "src/assets".into(),
            ..default()
        })
    )
    .add_systems(Startup, setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2d);

}