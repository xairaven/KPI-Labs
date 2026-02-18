use crate::approximation;
use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use crate::graphics::screen::{Resolution, ScreenParams};
use crate::state::State;
use crate::ui::styles::{colors, strokes};
use egui::{Frame, Response, Sense, Shape};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,

    shapes: Vec<Shape>,
}

impl Canvas {
    pub fn process(&mut self, state: &mut State) {
        // Grid
        self.shapes = state
            .grid
            .process(self.screen_params)
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();

        if !state.approximation_state.is_initialized() {
            return;
        }

        let radius = self.screen_params.value_cm_to_px(strokes::DOT_RADIUS_CM);
        let dot_stroke = strokes::dot(self.screen_params);

        let curve_stroke = strokes::curve(self.screen_params, state.curve_color);
        let linear_stroke = strokes::curve(self.screen_params, state.linear_color);
        let parabola_stroke = strokes::curve(self.screen_params, state.quadratic_color);

        if state.is_curve_enabled {
            let mut dots: Vec<Shape> = state
                .approximation_state
                .points()
                .iter()
                .map(|point| {
                    point.as_screen(self.screen_params).as_dot(
                        radius,
                        state.curve_color,
                        dot_stroke,
                    )
                })
                .collect();
            self.shapes.append(&mut dots);

            let mut curve: Vec<Shape> = state
                .approximation_state
                .points()
                .windows(2)
                .map(|points| {
                    Line2D::new(points[0], points[1], curve_stroke)
                        .to_screen(self.screen_params)
                        .to_shape()
                })
                .collect();
            self.shapes.append(&mut curve);
        }

        if state.is_line_enabled {
            let shape = approximation::graphics::linear_line(
                state.approximation_state.linear_coefficients(),
                linear_stroke,
            )
            .to_screen(self.screen_params)
            .to_shape();
            self.shapes.push(shape);
        }

        if state.is_parabola_enabled {
            let mut quadratic_curve: Vec<Shape> =
                approximation::graphics::parabola_lines(
                    state.approximation_state.quadratic_coefficients(),
                    parabola_stroke,
                )
                .iter()
                .map(|line| line.to_screen(self.screen_params).to_shape())
                .collect();
            self.shapes.append(&mut quadratic_curve);
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) =
            ui.allocate_painter(painter_size, Sense::click_and_drag());
        self.screen_params.canvas_center = Point2D::from_pos2(response.rect.center());
        self.screen_params.resolution =
            Resolution::from(response.rect.max.x, response.rect.max.y);

        painter.extend(self.shapes.clone());

        // Check for dragging
        self.screen_params.update_offset_on_drag(ui, &response);

        response
    }

    pub fn show(&mut self, state: &mut State, ui: &mut egui::Ui) {
        if self.screen_params.px_per_cm < 5.0 {
            self.screen_params.px_per_cm = 5.0;
        }

        Frame::canvas(ui.style())
            .fill(colors::WHITE)
            .show(ui, |ui| {
                ui.input(|i| {
                    let delta = i.smooth_scroll_delta.y;
                    self.screen_params.px_per_cm += delta * 0.1;
                });
                self.process(state);
                self.draw(ui);
            });
    }
}
