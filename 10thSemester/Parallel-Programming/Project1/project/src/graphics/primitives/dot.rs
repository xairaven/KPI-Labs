use crate::graphics::Viewport;
use crate::graphics::primitives::point::Point;
use crate::graphics::units::Centimeter;
use eframe::emath::Pos2;
use eframe::epaint::{CircleShape, Color32, Stroke};
use egui::{Rect, Response, Sense, Shape, Vec2};

#[derive(Debug, Default)]
pub struct Dot {
    pub center: Point,
    pub radius: Centimeter,
    pub fill: Color32,
    pub stroke_color: Color32,
    pub stroke_width: Centimeter,
    pub tooltip: Option<TooltipMetadata>,
}

#[derive(Debug, Clone)]
pub struct TooltipMetadata {
    pub text: String,
    pub radius: Centimeter,
    pub center: Point,
    pub id: usize,
}

impl Dot {
    pub fn into_shape(self, viewport: &Viewport) -> Shape {
        let circle = CircleShape {
            center: Pos2::from(self.center.to_pixels(viewport)),
            radius: self.radius.to_pixels_vector_x(viewport).0 as f32,
            fill: self.fill,
            stroke: Stroke::new(
                self.stroke_width.to_pixels_vector_x(viewport).0 as f32,
                self.stroke_color,
            ),
        };

        Shape::Circle(circle)
    }
}

impl TooltipMetadata {
    pub fn show(&self, ui: &mut egui::Ui, response: &Response, viewport: &Viewport) {
        let radius = self.radius.to_pixels_vector_x(viewport).value();
        let center: Pos2 = self.center.to_pixels(viewport).into();

        let size = Vec2::splat(2.0 * radius as f32);
        let area = Rect::from_center_size(center, size);
        let response = ui.interact(area, response.id.with(self.id), Sense::hover());
        response.on_hover_text(self.text.clone());
    }
}
