use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

struct AnimationData {}

#[derive(Default)]
pub struct AnimationLoader;

impl AssetLoader for AnimationLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let animation_data = ron::de::from_bytes::<AnimationData>(bytes)?;
            info!("load data done: {:?}", animation_data);
            load_context.set_default_asset(LoadedAsset::new(animation_data));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["frame_anim.ron"]
    }
}

mod test_load_mod {
    use crate::frame_animation::dragon_models::{SkeRoot, TexRoot};

    #[test]
    fn test_load() {
        let data = std::fs::read_to_string("assets/animation/player01_ske.json").unwrap();
        let result: SkeRoot = serde_json::from_str(&data).unwrap();

        println!("result = {:?}", result);

        let data = std::fs::read_to_string("assets/animation/player01_tex.json").unwrap();
        let result: TexRoot = serde_json::from_str(&data).unwrap();

        println!("result = {:?}", result);
    }
}
