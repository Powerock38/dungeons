use bevy::prelude::*;

use crate::{GameState, UiButton, UiWindow, UiWindowBundle, SAVE_DIR};

pub fn spawn_load_save_ui(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_windows: Query<Entity, With<UiWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if let Some(window) = q_windows.iter().next() {
            commands.entity(window).despawn_recursive();

            next_state.set(GameState::Running);
        } else {
            commands
                .spawn(UiWindowBundle::default())
                .with_children(|c| {
                    // Save button
                    c.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(5.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: UiButton::NORMAL.into(),
                            ..default()
                        },
                        UiButton::SaveGame,
                    ))
                    .with_children(|c| {
                        c.spawn(TextBundle::from_section(
                            "Save",
                            TextStyle {
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });

                    // Saves list
                    if let Ok(save_files) =
                        std::fs::read_dir(format!("assets/{SAVE_DIR}")).map(|dir| {
                            let mut saves = dir
                                .filter_map(|entry| {
                                    entry
                                        .ok()
                                        .filter(|entry| {
                                            entry.file_type().ok().map_or(false, |ft| ft.is_dir())
                                        })
                                        .and_then(|entry| entry.file_name().into_string().ok())
                                })
                                .collect::<Vec<_>>();

                            saves.sort_by(|a, b| b.cmp(a));
                            saves
                        })
                    {
                        for save_file in save_files {
                            c.spawn((
                                ButtonBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(5.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    background_color: UiButton::NORMAL.into(),
                                    ..default()
                                },
                                UiButton::LoadGame(save_file.clone()),
                            ))
                            .with_children(|c| {
                                c.spawn(TextBundle::from_section(
                                    save_file,
                                    TextStyle {
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..default()
                                    },
                                ));
                            });
                        }
                    } else {
                        error!("Failed to read save files");
                    }
                });

            next_state.set(GameState::Paused);
        }
    }
}
