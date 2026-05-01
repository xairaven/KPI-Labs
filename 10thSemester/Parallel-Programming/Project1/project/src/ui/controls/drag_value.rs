use crossbeam::channel::Sender;
use egui::{DragValue, emath};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct DragValueNotifiable<'a, Num: emath::Numeric, T: Clone> {
    value: &'a mut Num,
    speed: f64,
    range: RangeInclusive<Num>,
    suffix: String,

    tx: Sender<T>,
    commands: Vec<T>,
}

impl<'a, Num: emath::Numeric, T: Clone> DragValueNotifiable<'a, Num, T> {
    pub fn new(value: &'a mut Num) -> Self {
        Self {
            value,
            speed: 1.0,
            range: Num::MIN..=Num::MAX,
            suffix: "".to_string(),

            tx: crossbeam::channel::unbounded::<T>().0,
            commands: vec![],
        }
    }

    pub fn speed(self, speed: impl Into<f64>) -> Self {
        let speed = speed.into();
        Self { speed, ..self }
    }

    pub fn range(self, range: RangeInclusive<Num>) -> Self {
        Self { range, ..self }
    }

    pub fn suffix(self, suffix: &'static str) -> Self {
        let suffix = suffix.to_string();
        Self { suffix, ..self }
    }

    pub fn tx(self, tx: Sender<T>) -> Self {
        Self { tx, ..self }
    }

    pub fn command(mut self, command: T) -> Self {
        self.commands.push(command);
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        if ui
            .add(
                DragValue::new(self.value)
                    .speed(self.speed)
                    .range(self.range.clone())
                    .suffix(self.suffix.clone()),
            )
            .changed()
        {
            for command in self.commands.iter() {
                let _ = self.tx.send(command.clone());
            }
        }
    }
}
