use bevy::prelude::*;
pub use save_load_assets::*;
pub use save_load_systems::*;
pub use save_load_ui::*;

use crate::{
    data::{ObjectId, TileId},
    BuildResult, Dweller, Mob, Task, TaskKind, TaskNeeds, TilePlaced,
};

mod save_load_assets;
mod save_load_systems;
mod save_load_ui;

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                save_world_before.before(save_world),
                save_world,
                load_world,
                spawn_load_save_ui,
                scan_sprite_loaders,
            ),
        )
        .register_type::<SaveName>()
        .register_type::<Dweller>()
        .register_type::<ObjectId>()
        .register_type::<Option<ObjectId>>()
        .register_type::<Vec<ObjectId>>()
        .register_type::<TileId>()
        .register_type::<TilePlaced>()
        .register_type::<Mob>()
        .register_type::<Task>()
        .register_type::<Option<Entity>>()
        .register_type::<TaskKind>()
        .register_type::<BuildResult>()
        .register_type::<TaskNeeds>()
        .register_type::<Vec<IVec2>>()
        .register_type::<SpriteLoader>();
    }
}
