/*
 * Heavily inspired by:
 * - https://github.com/bevyengine/bevy/tree/main/examples/mobile
 */

#[cfg(target_os = "android")]
mod android;

use bevy::{prelude::*, winit::WinitSettings};

pub(super) struct MobilePlugin;
impl Plugin for MobilePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_os = "android")]
        app.add_plugins(android::AndroidPlugin);

        // NOTE: Make the winit loop wait more aggressively when no user input is received.
        //       This can help reduce cpu usage on mobile devices.
        app.insert_resource(WinitSettings::mobile());
    }
}
