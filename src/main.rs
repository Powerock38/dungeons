use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*, time::common_conditions::on_timer};
use bevy_ecs_tilemap::{prelude::MaterialTilemapPlugin, TilemapPlugin};
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    actions::*, camera::*, dwellers::*, mobs::*, save_load::*, state::*, tasks::*, terrain::*,
    tilemap::*, tiles::*, ui::*,
};

mod actions;
mod camera;
mod data;
mod dwellers;
mod mobs;
mod random_text;
mod save_load;
mod state;
mod structures;
mod tasks;
mod terrain;
mod tilemap;
mod tilemap_data;
mod tiles;
mod ui;
mod utils;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "wgpu=error,naga=warn,dungeons=debug".into(),
                    ..default()
                }),
            // bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            TilemapPlugin,
            SaveLoadPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<CameraControl>()
        .add_event::<LoadChunk>()
        .add_event::<UnloadChunk>()
        .add_event::<TaskCompletionEvent>()
        .add_event::<SpawnDwellersOnChunk>()
        .add_event::<SpawnMobsOnChunk>()
        .configure_sets(Update, GameplaySet.run_if(in_state(GameState::Running)))
        .configure_sets(
            FixedUpdate,
            GameplaySet.run_if(in_state(GameState::Running)),
        )
        .add_systems(Startup, (spawn_camera, spawn_new_terrain, spawn_ui))
        .add_systems(
            Update,
            (
                init_font,
                update_ui_buttons,
                update_camera,
                toggle_state,
                load_chunks,
                (spawn_dwellers, spawn_mobs).after(load_chunks),
                (
                    // Game UI
                    keyboard_current_action,
                    focus_any_dweller,
                    click_terrain,
                    update_dwellers_selected,
                )
                    .in_set(GameplaySet),
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                // Game logic
                (update_dwellers, update_mobs).run_if(on_timer(Duration::from_millis(200))),
                (update_dwellers_load_chunks).run_if(on_timer(Duration::from_millis(1000))),
                (update_terrain).run_if(on_timer(Duration::from_millis(800))),
                update_dwellers_movement,
                update_mobs_movement,
                update_unreachable_tasks,
                update_unreachable_pathfinding_tasks.run_if(on_timer(Duration::from_millis(5000))),
                update_pickups,
                event_task_completion,
                manage_chunks,
                update_tilemap_from_data.after(manage_chunks),
            )
                .in_set(GameplaySet),
        )
        .init_state::<GameState>()
        .insert_resource(SaveName({
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect()
        }))
        .init_resource::<CurrentAction>()
        .init_resource::<DwellersSelected>()
        .add_plugins(MaterialTilemapPlugin::<TilemapMaterial>::default())
        .run();
}
