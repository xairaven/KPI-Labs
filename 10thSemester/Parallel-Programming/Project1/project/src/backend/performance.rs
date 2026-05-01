use crossbeam::channel::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

#[derive(Debug)]
pub struct PerformanceMonitor {
    pub current_metrics: SystemMetrics,
    pub metrics_receiver: Receiver<SystemMetrics>,
}

// Struct to hold the data passing through the channel
#[derive(Debug, Default)]
pub struct SystemMetrics {
    pub cpus_info: Vec<CPUInfo>,
    pub global_cpu_usage: f32,
    pub memory_used_mb: f32,
    pub memory_total_mb: f32,
}

#[derive(Debug, Default)]
pub struct CPUInfo {
    pub name: String,
    pub usage: f32,
}

const BYTES_IN_MEGABYTE: f32 = 1_048_576.0;

impl PerformanceMonitor {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();

        thread::spawn(move || {
            let mut sys = System::new_with_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything().without_swap()),
            );

            thread::sleep(Duration::from_millis(500));

            loop {
                sys.refresh_cpu_usage();
                sys.refresh_memory();

                let metrics = SystemMetrics {
                    cpus_info: sys
                        .cpus()
                        .iter()
                        .map(|unit| CPUInfo {
                            name: unit.name().to_string(),
                            usage: unit.cpu_usage(),
                        })
                        .collect(),
                    global_cpu_usage: sys.global_cpu_usage(),
                    memory_used_mb: sys.used_memory() as f32 / BYTES_IN_MEGABYTE,
                    memory_total_mb: sys.total_memory() as f32 / BYTES_IN_MEGABYTE,
                };

                // Exit the thread gracefully if the app closes
                if tx.send(metrics).is_err() {
                    break;
                }

                thread::sleep(Duration::from_millis(1000));
            }
        });

        Self {
            current_metrics: Default::default(),
            metrics_receiver: rx,
        }
    }

    pub fn update(&mut self) -> Result<(), TryRecvError> {
        match self.metrics_receiver.try_recv() {
            Ok(metrics) => {
                self.current_metrics = metrics;
                Ok(())
            },
            Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected),
            Err(_) => Ok(()),
        }
    }
}
