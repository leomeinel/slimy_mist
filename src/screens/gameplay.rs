/*
 * File: gameplay.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 * -----
 * Heavily inspired by: https://github.com/TheBevyFlock/bevy_new_2d
 */

//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    characters::prelude::*, core::prelude::*, images::prelude::*, input::prelude::*,
    levels::prelude::*, procgen::prelude::*, render::prelude::*, screens::prelude::*,
    ui::prelude::*, utils::prelude::*,
};

pub(super) struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            OnEnter(Screen::Gameplay),
            (
                EnterGameplaySystems::Resources,
                EnterGameplaySystems::Sprites,
                EnterGameplaySystems::Animations,
                EnterGameplaySystems::Levels,
                EnterGameplaySystems::Camera,
                EnterGameplaySystems::NavMesh,
            )
                .chain(),
        );

        app.add_systems(
            OnEnter(Screen::Gameplay),
            insert_resources.in_set(EnterGameplaySystems::Resources),
        );
        app.add_systems(OnExit(Screen::Gameplay), remove_resources);
        app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
        app.add_systems(
            OnEnter(Menu::None),
            unpause.run_if(in_state(Screen::Gameplay)),
        );

        app.add_systems(
            Update,
            (
                (pause, spawn_pause_overlay, open_pause_menu).run_if(
                    in_state(Menu::None).and(
                        input_just_pressed(KeyCode::KeyP)
                            .or(input_just_pressed(KeyCode::Escape))
                            .or(window_unfocused),
                    ),
                ),
                close_menu.run_if(not(in_state(Menu::None)).and(input_just_pressed(KeyCode::KeyP))),
            )
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

/// A [`SystemSet`] for systems that initialize [`Screen::Gameplay`]
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) enum EnterGameplaySystems {
    Resources,
    Sprites,
    Animations,
    Levels,
    Camera,
    NavMesh,
}

/// Spawn pause overlay
fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(PAUSE_BACKGROUND),
        DespawnOnExit(Pause(true)),
    ));
}

/// Open pause menu
fn open_pause_menu(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::Pause);
}

/// Close pause menu
fn close_menu(mut next_state: ResMut<NextState<Menu>>) {
    (*next_state).set_if_neq(Menu::None);
}

/// Unpause the game
fn unpause(mut next_state: ResMut<NextState<Pause>>) {
    (*next_state).set_if_neq(Pause(false));
}

/// Pause the game
fn pause(mut next_state: ResMut<NextState<Pause>>) {
    (*next_state).set_if_neq(Pause(true));
}

/// Insert [`Resource`]s
fn insert_resources(mut commands: Commands) {
    commands.init_resource::<DayTimer>();
    commands.init_resource::<DayUpdateTimer>();
    commands.init_resource::<JoystickMap>();
    commands.init_resource::<JoystickRect<{ JoystickID::Movement as u8 }>>();
    commands.init_resource::<MouseDrag>();
    commands.init_resource::<PointerStartTimeSecs>();
    commands.init_resource::<ProcGenCache<OverworldProcGen>>();
    commands.init_resource::<ProcGenCache<Slime>>();
    commands.init_resource::<ProcGenCache<StreetLight>>();
}

/// Remove [`Resource`]s
fn remove_resources(mut commands: Commands) {
    commands.remove_resource::<DayTimer>();
    commands.remove_resource::<DayUpdateTimer>();
    commands.remove_resource::<DisplayImage<Player>>();
    commands.remove_resource::<DisplayImage<Slime>>();
    commands.remove_resource::<JoystickMap>();
    commands.remove_resource::<JoystickRect<{ JoystickID::Movement as u8 }>>();
    commands.remove_resource::<MouseDrag>();
    commands.remove_resource::<PointerStartTimeSecs>();
    commands.remove_resource::<ProcGenCache<OverworldProcGen>>();
    commands.remove_resource::<ProcGenCache<Slime>>();
    commands.remove_resource::<ProcGenCache<StreetLight>>();
}
