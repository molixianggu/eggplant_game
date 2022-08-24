use std::collections::HashMap;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

// #[derive(Debug, Deserialize, Clone)]
// pub struct Frame {
//     pub state: String,
//     pub start: usize,
//     pub end: usize,
// }

#[derive(Debug, Deserialize, Clone)]
pub enum Frame {
    Successive {
        start: usize,
        end: usize,
        root_motion: Vec<Vec2>,
    },
    Sequence {
        vec: Vec<usize>,
        root_motion: Vec<Vec2>,
    },
}

impl Frame {
    pub fn next(&self, current: usize, index: &mut usize) -> (usize, Vec2) {
        match self {
            Frame::Successive {
                start,
                end,
                root_motion: _,
            } => {
                if current + 1 >= *end {
                    (*start, Vec2::ZERO)
                } else if current < *start {
                    (*start, Vec2::ZERO)
                } else {
                    (current + 1, Vec2::ZERO)
                }
            }
            Frame::Sequence { vec, root_motion } => {
                if *index + 1 >= vec.len() {
                    *index = 0;
                } else {
                    *index += 1;
                }
                (vec[*index], root_motion[*index])

                // if let Some(i) = vec.iter().position(|&v| v == current) {
                //     if i + 1 >= vec.len() {
                //         (vec[0], root_motion.last().unwrap_or(&Vec2::ZERO).clone())
                //     } else {
                //         (vec[i + 1], root_motion.get(i).unwrap_or(&Vec2::ZERO).clone())
                //     }
                // } else {
                //     (vec[0], Vec2::ZERO)
                // }
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone, TypeUuid)]
#[uuid = "14c30a78-6cde-43db-af66-08412e9151f7"]
pub struct AnimationData {
    pub name: String,
    pub texture: String,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub frames: HashMap<String, Frame>,
}

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
