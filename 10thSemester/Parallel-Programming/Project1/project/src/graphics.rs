use crate::graphics::primitives::point::PointPixel;
use crate::graphics::units::{Centimeter, Pixel};
use egui::{InputState, Response};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Viewport {
    pub config: ViewportConfig,
    pub geometry: ViewportGeometry,
    pub state: ViewportState,
}

impl Viewport {
    pub fn handle_pan(&mut self, ui: &mut egui::Ui, response: Response) -> bool {
        if self.config.is_pannable && response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);

            let delta = response.drag_delta();
            const DRAGGING_COEFFICIENT: f64 = 1.0;

            self.geometry.offset.x += Pixel(delta.x as f64 * DRAGGING_COEFFICIENT);
            self.geometry.offset.y += Pixel(delta.y as f64 * DRAGGING_COEFFICIENT);

            ui.ctx().request_repaint();

            return true;
        }

        false
    }

    pub fn handle_scroll(&mut self, input_state: &InputState) -> bool {
        if !self.config.is_zoomable {
            return false;
        }

        let delta = input_state.smooth_scroll_delta.y;

        self.geometry.pixels_per_centimeter += (delta as f64) * 0.1;

        delta != 0.0
    }

    pub fn update_state(&mut self, response: &Response) {
        let bounds = ViewportBounds::from(response);
        // Update zero point
        let zero_point = self.geometry.zero_point_location.get_point(&bounds);
        self.state.zero_point = zero_point;
        // Update viewport location
        self.state.bounds = bounds;
    }
}

#[derive(Debug)]
pub struct ViewportConfig {
    pub is_pannable: bool,
    pub is_zoomable: bool,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            is_pannable: true,
            is_zoomable: true,
        }
    }
}

pub const PX_PER_CM_RANGE: RangeInclusive<f64> = 5.0..=100.0;
const DEFAULT_PX_PER_CM: f64 = 20.0;

#[derive(Debug)]
pub struct ViewportGeometry {
    pub zero_point_location: ZeroPointLocation,
    pub pixels_per_centimeter: f64,
    pub offset: PointPixel,
}

impl Default for ViewportGeometry {
    fn default() -> Self {
        Self {
            zero_point_location: ZeroPointLocation::Center,
            pixels_per_centimeter: DEFAULT_PX_PER_CM,
            offset: PointPixel {
                x: Pixel(0.0),
                y: Pixel(0.0),
            },
        }
    }
}

impl ViewportGeometry {
    pub fn reset_pixels_per_centimeter(&mut self) {
        self.pixels_per_centimeter = DEFAULT_PX_PER_CM;
    }

    pub fn reset_offset(&mut self) {
        self.offset = PointPixel::zero();
    }
}

#[derive(Debug, Default)]
pub struct ViewportState {
    pub zero_point: PointPixel,
    pub bounds: ViewportBounds<Pixel>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZeroPointLocation {
    Center,
    BottomLeftWithOffset { offset: Pixel },
    TopLeftWithOffset { offset: Pixel },
    BottomRightWithOffset { offset: Pixel },
    TopRightWithOffset { offset: Pixel },
}

impl ZeroPointLocation {
    pub fn get_point(&self, bounds: &ViewportBounds<Pixel>) -> PointPixel {
        match self {
            ZeroPointLocation::Center => PointPixel {
                x: bounds.center_x,
                y: bounds.center_y,
            },
            ZeroPointLocation::BottomLeftWithOffset { offset } => PointPixel::new(
                bounds.minimum_x.value() + offset.value(),
                bounds.maximum_y.value() - offset.value(),
            ),
            ZeroPointLocation::TopLeftWithOffset { offset } => PointPixel::new(
                bounds.minimum_x.value() + offset.value(),
                bounds.minimum_y.value() + offset.value(),
            ),
            ZeroPointLocation::BottomRightWithOffset { offset } => PointPixel::new(
                bounds.maximum_x.value() - offset.value(),
                bounds.maximum_y.value() - offset.value(),
            ),
            ZeroPointLocation::TopRightWithOffset { offset } => PointPixel::new(
                bounds.maximum_x.value() - offset.value(),
                bounds.minimum_y.value() + offset.value(),
            ),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ViewportBounds<Unit: Default + Clone> {
    pub minimum_x: Unit,
    pub maximum_x: Unit,
    pub minimum_y: Unit,
    pub maximum_y: Unit,
    pub center_x: Unit,
    pub center_y: Unit,
}

impl From<&Response> for ViewportBounds<Pixel> {
    fn from(response: &Response) -> Self {
        let (center_x, center_y) = response.rect.center().into();

        Self {
            minimum_x: Pixel(response.rect.min.x as f64),
            maximum_x: Pixel(response.rect.max.x as f64),
            minimum_y: Pixel(response.rect.min.y as f64),
            maximum_y: Pixel(response.rect.max.y as f64),
            center_x: Pixel(center_x as f64),
            center_y: Pixel(center_y as f64),
        }
    }
}

impl ViewportBounds<Pixel> {
    pub fn to_centimeters(self, viewport: &Viewport) -> ViewportBounds<Centimeter> {
        ViewportBounds::<Centimeter> {
            minimum_x: self.minimum_x.to_centimeters_x(viewport),
            maximum_x: self.maximum_x.to_centimeters_x(viewport),

            // For example: in pixels, minimum Y is 35 an maximum Y is 500.
            // In centimeters, minimum Y is 17.5 and maximum Y is 1.75 (for example).
            // So we need to invert them.
            minimum_y: self.maximum_y.to_centimeters_y(viewport),
            maximum_y: self.minimum_y.to_centimeters_y(viewport),

            center_x: self.center_x.to_centimeters_x(viewport),
            center_y: self.center_y.to_centimeters_y(viewport),
        }
    }
}

pub mod figures {
    pub mod border;
    pub mod grid;
}
pub mod primitives {
    pub mod dot;
    pub mod line;
    pub mod point;
}
pub mod units;
