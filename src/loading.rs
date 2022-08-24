use crate::GameState;
use crate::frame_animation::{AnimationLoader, AnimationData};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<AnimationLoader>();
        app.add_asset::<AnimationData>();
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<TextureAtlasAssets>()
                .with_collection::<AnimationAssets>()
        );
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/MSYH.TTF")]
    pub ms_yh: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/bg/bg_ui.png")]
    pub bg_ui: Handle<Image>,
    #[asset(path = "textures/bg/bg_select.png")]
    pub bg_select: Handle<Image>,
    #[asset(path = "textures/n_bg/01_n.jpg")]
    pub scene_bg: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct TextureAtlasAssets {
    // if the sheet would have padding, we could set that with `padding_x` and `padding_y`
    #[asset(texture_atlas(tile_size_x = 640., tile_size_y = 480., columns = 1, rows = 8))]
    #[asset(path = "textures/bg/3 (74).bin.BMP")]
    pub bg_ui: Handle<TextureAtlas>,
    // if the sheet would have padding, we could set that with `padding_x` and `padding_y`
    #[asset(texture_atlas(tile_size_x = 80., tile_size_y = 80., columns = 8, rows = 36))]
    #[asset(path = "textures/01xx.png")]
    pub player: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct AnimationAssets {
    #[asset(path = "animation/01.frame_anim.ron")]
    pub player01: Handle<AnimationData>,
}
