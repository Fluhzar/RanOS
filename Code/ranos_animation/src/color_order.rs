//! Specifies various orders to choose from.

use serde::{Deserialize, Serialize};

use ranos_ds::rgb::RGB;

/// Color order used by various animations, can be a predetermined order or a random order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorOrder {
    /// Order determined by random colors generated when needed.
    Random,
    /// Order determined by random bright colors generated when needed.
    RandomBright,
    /// Order determined by the associated data which is looped through sequentially.
    Ordered(Vec<RGB>),
}
