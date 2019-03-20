use super::Margin;
use crate::{enums::Alignment, structs::Spacer};

property!(
    /// `VerticalAlignment` describes the vertical alignment of a widget.
    VerticalAlignment(Alignment)
);

// --- Trait implementations ---

/// Used to align the position of a widget vertical.
pub trait VerticalAlignmentExtension {
    /// Calculates the y position of the widget depending on the available height, the goal height
    /// margin and Vertical alignment.
    fn align_y(&self, available_height: f64, height: f64, margin: Margin) -> f64;

    /// Calculates the height of the widget depending on the available height, the goal height
    /// margin and Vertical alignment.
    fn align_height(&self, available_height: f64, height: f64, margin: Margin) -> f64;
}

impl VerticalAlignmentExtension for VerticalAlignment {
    fn align_y(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        self.0
            .align_position(available_height, height, margin.top(), margin.bottom())
    }

    fn align_height(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        self.0
            .align_measure(available_height, height, margin.top(), margin.bottom())
    }
}

// --- Conversions ---

impl From<&str> for VerticalAlignment {
    fn from(s: &str) -> VerticalAlignment {
        VerticalAlignment(s.into())
    }
}

impl Into<PropertySource<VerticalAlignment>> for &str {
    fn into(self) -> PropertySource<VerticalAlignment> {
        PropertySource::Value(VerticalAlignment::from(self))
    }
}
