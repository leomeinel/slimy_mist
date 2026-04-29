mod color;
mod images;
mod rng;
mod run_conditions;
mod timers;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::color::{color_from_rgb, color_from_rgba};
    pub(crate) use super::images::{has_opaque_neighbor, is_transparent_pixel, pixel_index};
    pub(crate) use super::rng::{ForkedRng, setup_rng};
    pub(crate) use super::run_conditions::window_unfocused;
    pub(crate) use super::timers::{
        remove_oneshot_component_timers, tick_component_timers, tick_resource_timer,
    };
}
