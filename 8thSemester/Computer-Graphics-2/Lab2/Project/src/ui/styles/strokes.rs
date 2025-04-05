use crate::graphics::screen::ScreenParams;
use crate::ui::styles::colors;
use eframe::epaint::Stroke;
use egui::Color32;

const AXIS_WIDTH_PX: f32 = 1.5;
const GRID_WIDTH_PX: f32 = 0.8;

pub const DOT_RADIUS_CM: f32 = 0.03;
const DOT_WIDTH_CM: f32 = 0.003;
const CURVE_WIDTH_CM: f32 = 0.01;

pub fn axis_lime() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::LIME)
}

pub fn axis_red() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::DARK_RED)
}
pub fn grid_gray() -> Stroke {
    Stroke::new(GRID_WIDTH_PX, colors::GRAY)
}

pub fn curve(screen_params: ScreenParams, color: Color32) -> Stroke {
    let width = screen_params.value_cm_to_px(CURVE_WIDTH_CM);
    Stroke::new(width, color)
}

pub fn dot(screen_params: ScreenParams) -> Stroke {
    let width = screen_params.value_cm_to_px(DOT_WIDTH_CM);
    Stroke::new(width, colors::BLACK)
}
