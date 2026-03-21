/*
 * File: camera.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2026 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::{prelude::*, window::WindowResized};
use bevy_fast_light::prelude::*;

use crate::characters::prelude::*;

/// Main camera that renders the world to the canvas.
#[derive(Component)]
pub(crate) struct CanvasCamera;

/// Center the camera on [`Player`]
pub(super) fn center_camera_on_player(
    mut camera: Single<&mut Transform, (With<CanvasCamera>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<CanvasCamera>)>,
) {
    let target_pos = player.translation.xy().extend(camera.translation.z);
    camera.translation = target_pos;
}

/// Spawn [`Camera2d`]
pub(super) fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Canvas Camera"),
        Camera2d,
        Msaa::Off,
        CanvasCamera,
        AmbientLight2d::default(),
    ));
}

/// Threshold used to determine whether we should use a larger scale for [`Projection`].
///
/// This is compared to the minimum length retrieved from [`WindowResized`].
const SCALE_THRESHOLD: f32 = 500.;

/// Scales camera projection to fit the window (integer multiples only).
///
/// Heavily inspired by: <https://bevy.org/examples/2d-rendering/pixel-grid-snap/>
pub(super) fn fit_canvas(
    mut reader: MessageReader<WindowResized>,
    mut projection: Single<&mut Projection, With<CanvasCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };

    for resized in reader.read() {
        // Adjust scale based on short side of window
        let min_length = resized.width.min(resized.height);
        let canvas_height = if min_length > SCALE_THRESHOLD {
            360.
        } else {
            180.
        };
        let scale = 1. / (resized.height / canvas_height).round();

        projection.scale = scale;
    }
}

/// How quickly should the camera snap to the target location.
const CAMERA_DECAY_RATE: f32 = 3.;

/// Update the camera position by tracking the player.
///
/// Heavily inspired by: <https://bevy.org/examples/camera/2d-top-down-camera/>
pub(super) fn update_camera(
    mut camera: Single<&mut Transform, (With<CanvasCamera>, Without<Player>)>,
    player: Single<&Transform, (Changed<Transform>, With<Player>, Without<CanvasCamera>)>,
    time: Res<Time>,
) {
    let target_pos = player.translation.xy().extend(camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&target_pos, CAMERA_DECAY_RATE, time.delta_secs());
}
