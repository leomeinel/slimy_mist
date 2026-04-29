use bevy::prelude::*;

/// Fading in and out of splash screen
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct FadeInOut {
    /// Total duration in seconds.
    pub(crate) total_duration: f32,
    /// Fade duration in seconds.
    pub(crate) fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    pub(crate) t: f32,
}
impl FadeInOut {
    pub(crate) fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

/// Tick [`FadeInOut`]
pub(crate) fn tick_fade_in_out<T>(mut query: Query<&mut FadeInOut, With<T>>, time: Res<Time>)
where
    T: Component,
{
    for mut anim in &mut query {
        anim.t += time.delta_secs();
    }
}

/// Apply [`FadeInOut`]
pub(crate) fn apply_fade_in_out(mut query: Query<(&FadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut query {
        image.color.set_alpha(anim.alpha())
    }
}
