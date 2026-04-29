use bevy::{prelude::*, window::WindowFocused};

/// Run condition that is active if any [`WindowFocused::focused`] has been sent with false.
///
/// This indicates that any window has lost focus.
pub(crate) fn window_unfocused(mut reader: MessageReader<WindowFocused>) -> bool {
    reader.read().any(|w| !w.focused)
}
