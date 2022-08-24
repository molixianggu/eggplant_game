use crate::actions::Actions;
use crate::frame_animation::AnimationData;
use crate::loading::{AnimationAssets, TextureAssets, TextureAtlasAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_scene)
                    .with_system(spawn_player)
                    .with_system(spawn_camera),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player)
                    .with_system(animate_sprite),
            );
    }
}

fn spawn_scene(mut commands: Commands, textures: Res<TextureAssets>) {
    // commands.spawn_bundle(SpriteBundle {
    //     texture: textures.scene_bg.clone(),
    //     ..default()
    // });
    commands
        .spawn()
        .insert(Collider::cuboid(640.0, 64.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -230.0, 0.0)));
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, textures_atlas: Res<TextureAtlasAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures_atlas.player.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(PlayerStateComponent::default())
        .insert(Player)
        .insert(TagetMove {
            taget: Vec2 { x: 0.0, y: 0.0 },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        // .insert(Restitution::coefficient(0.7))
        .insert(LockedAxes::ROTATION_LOCKED);
}

#[derive(Debug, Clone, Copy)]
enum PlayerState {
    IdleLeft,
    IdleRigth,
    WalkLeft,
    WalkRigth,
}

#[derive(Component)]
struct PlayerStateComponent {
    state: PlayerState,
    index: usize,
}

impl PlayerStateComponent {}

impl Default for PlayerStateComponent {
    fn default() -> Self {
        Self {
            state: PlayerState::IdleRigth,
            index: 0,
        }
    }
}

#[derive(Component)]
struct TagetMove {
    taget: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    animation: Res<AnimationAssets>,
    animation_assets: Res<Assets<AnimationData>>,
    mut query: Query<(
        &Transform,
        &mut TagetMove,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut PlayerStateComponent,
    )>,
) {
    for (transform, mut taget_move, mut timer, mut sprite, mut config) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            config.state;
            if let Some(data) = animation_assets.get(&animation.player01) {
                if let Some(frame) = data.frames.get(&config.state.to_string()) {
                    let (index, root_motion) = frame.next(sprite.index, &mut config.index);
                    sprite.index = index;
                    taget_move.taget.x = transform.translation.x + root_motion.x;
                    taget_move.taget.y = transform.translation.y + root_motion.y;
                }
            }
        }
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut PlayerStateComponent, &TagetMove)>,
) {
    // let speed = 150.;
    // let movement = Vec3::new(
    //     actions.player_movement.unwrap().x * speed * time.delta_seconds(),
    //     actions.player_movement.unwrap().y * speed * time.delta_seconds(),
    //     0.,
    // );
    // for (mut player_transform, mut state, _) in player_query.iter_mut() {
    //     // player_transform.translation += movement;

    // }

    for (mut transform, mut state, taget_move) in player_query.iter_mut() {
        if actions.player_movement.is_none() {
            state.state = match state.state {
                PlayerState::WalkLeft => PlayerState::IdleLeft,
                PlayerState::WalkRigth => PlayerState::IdleRigth,
                _ => state.state,
            };
            continue;
        } else if actions.player_movement.unwrap().x > 0.0 {
            state.state = PlayerState::WalkRigth;
        } else if actions.player_movement.unwrap().x < 0.0 {
            state.state = PlayerState::WalkLeft;
        }
        transform.translation.x +=
            (taget_move.taget.x - transform.translation.x) * time.delta_seconds() * 10.0;
    }
}

impl PlayerState {
    fn to_string(&self) -> String {
        match self {
            PlayerState::IdleLeft => "idle-left".to_string(),
            PlayerState::IdleRigth => "idle-rigth".to_string(),
            PlayerState::WalkLeft => "walk-left".to_string(),
            PlayerState::WalkRigth => "walk-rigth".to_string(),
        }
    }
}
