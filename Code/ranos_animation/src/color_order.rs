//! # Color Order

use serde::{Serialize, Deserialize};

use ranos_ds::rgb::RGB;

/// Color order used by `Breath`, can be a predetermined order or a random order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorOrder {
    /// Order determined by random colors generated when needed.
    Random,
    /// Order determined by random bright colors generated when needed.
    RandomBright,
    /// Order determined by the associated data which is looped through sequentially.
    Ordered(Vec<RGB>),
}
