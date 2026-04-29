//! This stores warning messages

/// Warning on incomplete [`AnimationData`](crate::animations::prelude::AnimationData).
pub(crate) const WARN_INCOMPLETE_ANIMATION_DATA: &str = "Incomplete animation data.";
/// Warning on incomplete asset data.
pub(crate) const WARN_INCOMPLETE_ASSET_DATA: &str = "Incomplete asset data.";
/// Warning on incomplete [`CollisionData`](crate::physics::prelude::CollisionData).
pub(crate) const WARN_INCOMPLETE_COLLISION_DATA: &str = "Incomplete collision data.";

/// Warning on invalid [`AttackData`](crate::characters::prelude::AttackData`).
pub(crate) const WARN_INVALID_ATTACK_DATA: &str = "Invalid attack data.";
/// Warning on invalid [`UiNavAction`](crate::input::prelude::UiNavAction`).
pub(crate) const WARN_INVALID_UI_NAV: &str =
    "Invalid ui nav action. No next button found to navigate to.";
