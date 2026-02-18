use crate::geometry::point2d::Point2D;
use crate::interpolation::state::InterpolationContext;
use crate::state::State;
use crate::ui::modals::Modal;
use egui::scroll_area::ScrollBarVisibility;
use egui::{DragValue, Grid, Id};

pub struct InputModal {
    id: i64,
    name: String,

    width: f32,
    height: f32,

    is_open: bool,
}

impl Default for InputModal {
    fn default() -> Self {
        Self {
            id: rand::random(),
            name: "Input Points".to_string(),

            is_open: true,

            width: 100.0,
            height: 300.0,
        }
    }
}

impl Modal for InputModal {
    fn show(&mut self, main_state: &mut State, ui: &egui::Ui) {
        let interpolation_ctx = &mut main_state.interpolation_context;

        if self.is_open {
            let modal = egui::Modal::new(Id::new(self.id)).show(ui.ctx(), |ui| {
                ui.set_width(self.width);
                ui.set_max_height(self.height);

                ui.vertical_centered_justified(|ui| {
                    ui.heading(&self.name);
                });

                ui.add_space(16.0);

                if interpolation_ctx.points_view.is_empty() {
                    interpolation_ctx.points_view.push(Point2D::default());
                }

                egui::ScrollArea::vertical()
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                    .show(ui, |ui| {
                        let mut to_remove_element: Option<usize> = None;
                        Grid::new("Approximation.Points")
                            .num_columns(6)
                            .striped(false)
                            .show(ui, |ui| {
                                for (index, point) in
                                    interpolation_ctx.points_view.iter_mut().enumerate()
                                {
                                    ui.label(format!("#{}", index + 1));
                                    ui.label("X:");
                                    ui.add(
                                        DragValue::new(&mut point.x)
                                            .speed(0.01)
                                            .range(f32::MIN..=f32::MAX),
                                    );

                                    ui.label("Y:");
                                    ui.add(
                                        DragValue::new(&mut point.y)
                                            .speed(0.01)
                                            .range(f32::MIN..=f32::MAX),
                                    );

                                    if ui.button("-").clicked() {
                                        to_remove_element = Some(index);
                                    }
                                    ui.end_row();
                                }
                            });
                        if let Some(index) = to_remove_element {
                            interpolation_ctx.points_view.remove(index);
                        }
                    });

                ui.vertical_centered_justified(|ui| {
                    if ui.button("+").clicked() {
                        interpolation_ctx.points_view.push(Point2D::default());
                    }
                });

                ui.add_space(16.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered_justified(|ui| {
                        if ui.button("Save").clicked() {
                            interpolation_ctx.initialize();
                            self.close(interpolation_ctx);
                        }
                    });
                    columns[1].vertical_centered_justified(|ui| {
                        if ui.button("Close").clicked() {
                            self.close(interpolation_ctx);
                        }
                    });
                });
            });

            if modal.should_close() {
                self.close(interpolation_ctx);
            }
        }
    }

    fn is_closed(&self) -> bool {
        !self.is_open
    }
}

impl InputModal {
    fn close(&mut self, ctx: &mut InterpolationContext) {
        if !ctx.is_initialized() {
            *ctx = Default::default();
        }
        self.is_open = false;
    }
}
