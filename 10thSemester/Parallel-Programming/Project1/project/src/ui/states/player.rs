use crate::backend::commands::UiCommand;
use crate::backend::simulation::SimulationSettings;
use crate::backend::snapshot::CrystalSnapshot;
use crate::graphics::Viewport;
use crate::graphics::figures::border::Border;
use crate::graphics::primitives::dot::TooltipMetadata;
use crossbeam::channel::Sender;
use egui::Shape;
use std::time::Instant;

#[derive(Debug)]
pub struct Player {
    pub mode: ViewMode,
    pub real_time: RealTimeVisualizer,
    pub history: SnapshotStorage,
}

#[derive(Debug, Default)]
pub struct DrawResponse {
    pub shapes: Vec<Shape>,
    pub tooltips: Vec<TooltipMetadata>,
}

impl Player {
    pub fn new(command_tx: Sender<UiCommand>) -> Self {
        Self {
            mode: Default::default(),
            real_time: RealTimeVisualizer::new(command_tx.clone()),
            history: Default::default(),
        }
    }

    pub fn visualize(&mut self, viewport: &Viewport) -> DrawResponse {
        match self.mode {
            ViewMode::RealTime => self.real_time.visualize(viewport),
            ViewMode::Snapshot => self.history.visualize(viewport),
        }
    }

    pub fn pass_real_snapshot(&mut self, snapshot: CrystalSnapshot) {
        if self.mode == ViewMode::RealTime {
            self.real_time.update_snapshot(snapshot);
        }
    }

    pub fn finish_real_time(&mut self) {
        self.real_time.finish();
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ViewMode {
    #[default]
    RealTime,
    Snapshot,
}

impl std::fmt::Display for ViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RealTime => write!(f, "Real Time"),
            Self::Snapshot => write!(f, "Snapshot"),
        }
    }
}

#[derive(Debug)]
pub struct RealTimeVisualizer {
    pub is_enabled: bool,
    last_snapshot: Option<CrystalSnapshot>,
    start_time: Option<Instant>,

    border: Border,

    ui_tx: Sender<UiCommand>,
}

impl RealTimeVisualizer {
    pub fn new(ui_tx: Sender<UiCommand>) -> Self {
        Self {
            is_enabled: false,
            last_snapshot: None,
            start_time: None,
            border: Default::default(),
            ui_tx,
        }
    }

    pub fn start(&mut self, settings: &SimulationSettings) {
        self.is_enabled = true;
        self.start_time = Some(Instant::now());
        self.last_snapshot = None;
        self.border.resize(&settings.crystal_size);

        let _ = self
            .ui_tx
            .try_send(UiCommand::StartSimulation(settings.clone()));
    }

    pub fn stop(&mut self) {
        self.finish();
        let _ = self.ui_tx.try_send(UiCommand::StopSimulation);
    }

    pub fn update_snapshot(&mut self, snapshot: CrystalSnapshot) {
        self.last_snapshot = Some(snapshot);
    }

    pub fn time(&self) -> String {
        match &self.start_time {
            None => String::from("00 : 00"),
            Some(start_time) => {
                let seconds = start_time.elapsed().as_secs_f64();

                let view_minutes = (seconds / 60.0).floor() as u64;
                let view_seconds = (seconds % 60.0).floor() as u64;

                format!("{:02} : {:02}", view_minutes, view_seconds)
            },
        }
    }

    pub fn total_atoms(&self) -> Option<usize> {
        self.last_snapshot
            .as_ref()
            .map(|snapshot| snapshot.total_atoms)
    }

    pub fn visualize(&mut self, viewport: &Viewport) -> DrawResponse {
        if !self.is_enabled && self.last_snapshot.is_none() {
            return DrawResponse::default();
        }

        let mut response = DrawResponse::default();

        let border = self
            .border
            .lines()
            .iter()
            .map(|line| line.to_pixels(viewport).to_shape())
            .collect::<Vec<Shape>>();
        response.shapes.extend(border);

        if let Some(snapshot) = &self.last_snapshot {
            let dots = snapshot.dots();

            for dot in &dots {
                if let Some(tooltip) = &dot.tooltip {
                    response.tooltips.push(tooltip.clone());
                }
            }

            let shapes = dots
                .into_iter()
                .map(|dot| dot.into_shape(viewport))
                .collect::<Vec<Shape>>();

            response.shapes.extend(shapes);
        }

        response
    }

    pub fn finish(&mut self) {
        self.is_enabled = false;
        self.start_time = None;
    }
}

#[derive(Debug, Default, Clone)]
pub struct SnapshotStorage {
    storage: Vec<CrystalSnapshot>,
    current: Option<usize>,
}

impl SnapshotStorage {
    pub fn add(&mut self, snapshot: CrystalSnapshot) {
        self.storage.push(snapshot);
        if self.len() == 1 {
            self.current = Some(0);
        }
    }

    pub fn get(&self, index: usize) -> Option<&CrystalSnapshot> {
        self.storage.get(index)
    }

    pub fn current_index(&self) -> &Option<usize> {
        &self.current
    }

    pub fn left(&mut self) {
        if let Some(index) = &mut self.current
            && *index > 0usize
        {
            *index -= 1usize;
        }
    }

    pub fn right(&mut self) {
        let len = self.storage.len();
        if let Some(index) = &mut self.current
            && *index < (len - 1)
        {
            *index += 1usize;
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn clear(&mut self) {
        self.current = None;
        self.storage.clear();
    }

    pub fn visualize(&mut self, viewport: &Viewport) -> DrawResponse {
        let snapshot = if let Some(index) = self.current_index()
            && let Some(snapshot) = self.get(*index)
        {
            snapshot
        } else {
            return DrawResponse::default();
        };

        let mut response = DrawResponse::default();

        let mut border = Border::default();
        border.resize(&snapshot.size);
        let border = border
            .lines()
            .iter()
            .map(|line| line.to_pixels(viewport).to_shape())
            .collect::<Vec<Shape>>();
        response.shapes.extend(border);

        let dots = snapshot.dots();

        for dot in &dots {
            if let Some(tooltip) = &dot.tooltip {
                response.tooltips.push(tooltip.clone());
            }
        }

        let dot_shapes = dots
            .into_iter()
            .map(|dot| dot.into_shape(viewport))
            .collect::<Vec<Shape>>();
        response.shapes.extend(dot_shapes);

        response
    }
}
