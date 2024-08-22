//! Constants
//!
//! 1 == one pixel
//! magical number = ratio

pub const CLEAR: bevy::color::Color = bevy::color::Color::srgb(0.1, 0.1, 0.1);

pub const RESOLUTION: f32 = 16. / 9.;

pub const TILE_SIZE: f32 = 1.;

pub const FRAME_TIME: f32 = 0.1;
pub const CLOUD_FRAME_TIME: f32 = 0.001 * FRAME_TIME;

pub mod character {

    use super::TILE_SIZE;

    pub const CHAR_SCALE: f32 = 1. * TILE_SIZE;
    pub const CHAR_Z: f32 = 10.;

    pub const CHAR_HITBOX_HEIGHT: f32 = 1.5 * CHAR_SCALE;
    pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;
    pub const CHAR_HITBOX_Y_OFFSET: f32 = -3.5 * CHAR_SCALE;
    pub const CHAR_HITBOX_Z_OFFSET: f32 = 0. * CHAR_SCALE;

    pub mod npcs {

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;

        pub const BLUE_CAT_STARTING_ANIM: usize = 0;
        pub const BLACK_CAT_STARTING_ANIM: usize = 2;

        pub mod movement {
            use crate::constants::character::CHAR_Z;

            pub const BLUE_CAT_STARTING_POSITION: (f32, f32, f32) = (-5., -48., CHAR_Z);
            pub const BLACK_CAT_STARTING_POSITION: (f32, f32, f32) = (-5., 20., CHAR_Z);
        }
    }

    pub mod effects {

        pub const DAZE_TIMER: u64 = 2;
        pub const DAZE_STARTING_ANIM: usize = 0;
        pub const DAZE_Y_OFFSET: (f32, f32, f32) = (0., 10., 0.);
    }
}

pub mod locations {

    pub const LEVEL_Z: f32 = 3.;
    pub const LEVEL_POSITION: (f32, f32, f32) = (0., 0., LEVEL_Z);
    pub const LEVEL_SCALE: (f32, f32, f32) = (1., 1., 1.);

    pub const FLOOR_Z: f32 = 1.;
    pub const FLOOR_POSITION: (f32, f32, f32) = (0., 0., FLOOR_Z);

    pub mod level_one {
        use super::LEVEL_Z;

        pub const IN_DOOR_POSITION: (f32, f32, f32) = (-5., -36., LEVEL_Z);
        pub const ALT_DOOR_POSITION: (f32, f32, f32) = (2.5, 6., LEVEL_Z);
        pub const OUT_DOOR_POSITION: (f32, f32, f32) = (-5., 36., LEVEL_Z);

        pub const BUTTON_POSITION: (f32, f32, f32) = (24., 6., LEVEL_Z - 1.);
        pub const BUTTON_HITBOX_X_OFFSET: (f32, f32, f32) = (-3., 0., 0.);
        pub const BUTTON_SENSOR_POSITION: (f32, f32, f32) = (25.5, 6., LEVEL_Z);

        pub const WAYPOINT_TOP: (f32, f32, f32) = (-5., 30., 0.);
        pub const WAYPOINT_BOT: (f32, f32, f32) = (-5., -30., 0.);
    }
}

pub mod cinematics {
    pub const CLOUDS_LIMIT: f32 = -333.;
    pub const CLOUDS_RESET: f32 = 334.;
    pub const SECOND_CLOUDS_INIT: f32 = 1001.;
}

pub mod ui {
    pub mod tablet {
        use bevy::prelude::Color;

        pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
        pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
        pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

        pub const HOVERED_INACTIVE_BUTTON: Color = Color::srgb(0.75, 0.75, 0.75);
        pub const INACTIVE_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
    }

    pub const DRAGGED_ENTITY_Z: f32 = 100.;
}
