mod save_data;
pub use save_data::*;

use std::time::Duration;

use crate::{assets::AssetManager, input_system::InputSystem, sound_manager::SoundManager};

#[cfg_attr(feature = "editor", derive(guiedit_derive::Inspectable))]
pub struct Context<'assets> {
    pub assets: &'assets AssetManager,
    pub sound: SoundManager<'assets>,
    pub completed_levels: SaveData,
    pub delta_time: Duration,
    pub input: InputSystem,
}
