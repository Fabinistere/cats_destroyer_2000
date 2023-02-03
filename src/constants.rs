//! Constants
//!
//! 1 == one pixel
//! magical number = ratio

pub const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.1, 0.1, 0.1);

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub const TILE_SIZE: f32 = 1.;

pub mod character {

    use super::TILE_SIZE;
    
    pub const CHAR_SCALE: f32 = 1. * TILE_SIZE;

    pub mod npc {

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;

        pub const BLUE_CAT_STARTING_ANIM: usize = 0;

        pub mod movement {

            pub const BLUE_CAT_POSITION: (f32, f32, f32) = (0., 0., 0.);
        }
    }
}