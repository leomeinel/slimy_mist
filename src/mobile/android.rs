/*
 * Heavily inspired by:
 * - https://github.com/bevyengine/bevy/tree/main/examples/mobile
 */

use bevy::{prelude::*, window::AppLifecycle};

pub(super) struct AndroidPlugin;
impl Plugin for AndroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_lifetime.run_if(any_with_component::<AudioSink>),
        );
    }
}

/// Pause audio when app goes into background and resume when it returns.
fn handle_lifetime(mut reader: MessageReader<AppLifecycle>, audio_sink: Single<&AudioSink>) {
    for app_lifecycle in reader.read() {
        match app_lifecycle {
            AppLifecycle::Suspended => audio_sink.pause(),
            AppLifecycle::Running => audio_sink.play(),
            AppLifecycle::Idle | AppLifecycle::WillSuspend | AppLifecycle::WillResume => (),
        }
    }
}
