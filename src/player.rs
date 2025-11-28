// player.rs
use bevy::prelude::*;

const TILE_SIZE: u32 = 64;
const WALK_FRAMES: usize = 9;
const MOVE_SPEED: f32 = 140.0;
const ANIM_DT: f32 = 0.1;

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing{
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState{
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
){
    let texture = asset_server.load("character-spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas{
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Player,
        AnimationState{facing, moving: false, was_moving: false},
        AnimationTimer(Timer::from_second(ANIM_DT, TimerMode::Repeating)),
    ));

}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_transform: Single<&mut Transform, With<Player>>,
){
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft){
        direction.x -=1.0;
    }
    if input.pressed(KeyCode::ArrowRight){
        direction.x +=1.0;
    }
    if input.pressed(KeyCode::ArrowUp){
        direction.y +=1.0;
    }
    if input.pressed(KeyCode::ArrowDown){
        direction.y -=1.0;
    }

    if direction != Vec2::ZERO{
        let speed = 300.0;
        let delta = direction.normalize() *speed * time.delta_secs();
        player_transform.translation.x += delta.x;
        player_transform.translation.y += delta.y;

        if direction.x.abs() > direction.y.abs(){
            anim.facing = if direction.x > 0.0 {Facing::Right} else {Facing::Left};
        } else {
            anim.facing = if direction.y > 0.0 {Facing::Up} else {Facing::Down};
        }

    } else {
        anim.moving = false;
    }


}