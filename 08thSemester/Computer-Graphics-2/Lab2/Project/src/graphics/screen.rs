use crate::geometry::point2d::Point2D;
use egui::{Response, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct ScreenParams {
    pub canvas_center: Point2D,
    pub resolution: Resolution,
    pub px_per_cm: f32,
    pub unit_length: f32,

    pub is_dragging_offset_enabled: bool,
    pub offset: (f32, f32),
}

impl Default for ScreenParams {
    fn default() -> Self {
        Self {
            canvas_center: Point2D::new(500.0, 500.0),
            resolution: Default::default(),
            px_per_cm: 20.0,
            unit_length: 0.1,

            is_dragging_offset_enabled: true,
            offset: (0.0, 0.0),
        }
    }
}

impl ScreenParams {
    pub fn value_cm_to_px(&self, value: f32) -> f32 {
        value / self.unit_length * self.px_per_cm
    }

    pub fn point_cm_to_px(&self, point: Point2D) -> Point2D {
        debug_assert!(!point.converted_to_screen);

        let x = self.canvas_center.x
            + (point.x / self.unit_length * self.px_per_cm)
            + self.offset.0;
        let y = self.canvas_center.y - (point.y / self.unit_length * self.px_per_cm)
            + self.offset.1;

        Point2D::new(x, y).with_converted_checked()
    }

    pub fn value_px_to_cm(&self, value: f32) -> f32 {
        value / self.px_per_cm * self.unit_length
    }

    pub fn point_px_to_cm(&self, point: Point2D) -> Point2D {
        debug_assert!(point.converted_to_screen);

        let x = (point.x * self.unit_length / self.px_per_cm) - self.canvas_center.x
            + self.offset.0;
        let y = (point.y * self.unit_length / self.px_per_cm)
            + self.canvas_center.y
            + self.offset.1;

        Point2D::new(x, y).with_converted_unchecked()
    }

    pub fn vec2_px_to_cm(&self, vec: Vec2) -> Vec2 {
        Vec2::new(self.value_px_to_cm(vec.x), -self.value_px_to_cm(vec.y))
    }

    pub fn vec2_cm_to_px(&self, vec: Vec2) -> Vec2 {
        Vec2::new(self.value_cm_to_px(vec.x), -self.value_cm_to_px(vec.y))
    }

    pub fn update_offset_on_drag(&mut self, ui: &egui::Ui, response: &Response) {
        if self.is_dragging_offset_enabled && response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);

            let delta = response.drag_delta();
            let dragging_coefficient = 1.0;

            self.offset.0 += delta.x * dragging_coefficient;
            self.offset.1 += delta.y * dragging_coefficient;
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Resolution {
    pub width: f32,
    pub height: f32,
}

impl Resolution {
    pub fn from(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
