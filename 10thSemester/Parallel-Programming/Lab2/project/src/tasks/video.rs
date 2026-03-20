use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkKind, Benchmarkable};
use crate::task::executable::{Executable, RunMode};
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use image::{DynamicImage, RgbaImage, imageops};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

#[derive(Debug)]
pub struct VideoPipeline {
    input_video: PathBuf,
    input_directory: PathBuf,
    output_directory: PathBuf,
    watermark: RgbaImage,
    frame_paths: Vec<PathBuf>,
}

impl VideoPipeline {
    const BASE_DIRECTORY: &'static str = "kpi_video_lab";
    const FRAMES_INPUT_DIRECTORY: &'static str = "frames_in";
    const FRAMES_OUTPUT_DIRECTORY: &'static str = "frames_out";

    // You should pass the path to a short test video (e.g., 5-10 seconds)
    pub fn with_video(video_path: &str) -> Self {
        let base_dir = std::env::temp_dir().join(Self::BASE_DIRECTORY);

        Self {
            input_video: PathBuf::from(video_path),
            input_directory: base_dir.join(Self::FRAMES_INPUT_DIRECTORY),
            output_directory: base_dir.join(Self::FRAMES_OUTPUT_DIRECTORY),
            // Generating a simple semi-transparent red 100x100 watermark
            watermark: RgbaImage::from_pixel(100, 100, image::Rgba([255, 0, 0, 128])),
            frame_paths: vec![],
        }
    }
}

#[derive(Debug)]
pub struct DynamicImageResource {
    pub image: DynamicImage,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct RgbaImageResource {
    pub image: RgbaImage,
    pub path: PathBuf,
}

// Pure functions representing our processing stages
impl VideoPipeline {
    // Stage 1: Decode (Read image file from disk into memory)
    fn decode_stage(path: &Path) -> Result<DynamicImageResource, Error> {
        let image = image::open(path).map_err(SystemError::ImageOpen)?;

        Ok(DynamicImageResource {
            image,
            path: path.to_path_buf(),
        })
    }

    // Stage 2: Apply a grayscale filter
    fn filter_stage(resource: DynamicImageResource) -> DynamicImageResource {
        DynamicImageResource {
            image: resource.image.grayscale(),
            path: resource.path,
        }
    }

    // Stage 3: Apply the watermark
    fn watermark_stage(
        resource: DynamicImageResource, watermark: &RgbaImage,
    ) -> RgbaImageResource {
        let mut rgba_image = resource.image.to_rgba8();

        // Overlay watermark at top-left corner
        imageops::overlay(&mut rgba_image, watermark, 10, 10);

        RgbaImageResource {
            image: rgba_image,
            path: resource.path,
        }
    }

    // Stage 4: Encode (Save the processed image back to disk)
    fn encode_stage(
        data: RgbaImageResource, output_directory: &Path,
    ) -> Result<(), Error> {
        let file_name = data.path.file_name().ok_or(SystemError::FileName(
            data.path.to_string_lossy().to_string(),
        ))?;

        let image_path = output_directory.join(file_name);

        data.image
            .save(image_path)
            .map_err(SystemError::ImageSave)
            .map_err(Error::System)
    }
}

impl Benchmarkable for VideoPipeline {
    fn benchmarks(&self) -> Vec<BenchmarkKind> {
        vec![
            BenchmarkKind::Sequential,
            BenchmarkKind::Pipeline {
                buffer_capacity: 5,
                max_threads: 4,
            },
            BenchmarkKind::ProducerConsumer {
                buffer_capacity: 20,
                producers: 1,
                consumers: 2,
            },
            BenchmarkKind::ProducerConsumer {
                buffer_capacity: 20,
                producers: 1,
                consumers: 4,
            },
            BenchmarkKind::ProducerConsumer {
                buffer_capacity: 20,
                producers: 1,
                consumers: 8,
            },
        ]
    }
}

impl Manageable for VideoPipeline {
    fn setup(&mut self) -> Result<(), Error> {
        fs::create_dir_all(&self.input_directory)
            .map_err(SystemError::DirectoryCreate)?;
        fs::create_dir_all(&self.output_directory)
            .map_err(SystemError::DirectoryCreate)?;

        // Extract frames ONLY if the directory is empty to save time across benchmarks
        let is_empty = fs::read_dir(&self.input_directory)
            .map_err(SystemError::DirectoryRead)?
            .next()
            .is_none();

        if is_empty {
            log::info!("Extracting frames from video using FFmpeg...");
            let status = Command::new("ffmpeg")
                .arg("-i")
                .arg(&self.input_video)
                // Hardware scale filter to resize frames to 480p width.
                .arg("-vf")
                .arg("scale=854:-1")
                // Outputting as PNG. 480p PNGs will easily fit in the tmpfs RAM disk.
                .arg(self.input_directory.join("frame_%04d.png"))
                .status()
                .map_err(SystemError::FfmpegExecution)?;
            log::info!("FFmpeg exited with status: {}", status);
        }

        self.frame_paths.clear();

        for entry in fs::read_dir(&self.input_directory)
            .map_err(SystemError::DirectoryRead)?
            .flatten()
        {
            self.frame_paths.push(entry.path());
        }

        // Sorting guarantees the frames are processed sequentially
        self.frame_paths.sort();

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        log::info!("Compiling processed frames back into video...");

        // Output video path (saving in the current working directory).
        // Feel free to change this path if you want it saved elsewhere.
        const FINAL_VIDEO_NAME: &str = "processed_video.mp4";
        let final_video_path = PathBuf::from(FINAL_VIDEO_NAME);

        // Execute FFmpeg to compile the frames.
        // -y: Overwrite output file if it exists.
        // -r 24: Set framerate to 24 fps (adjust if your source video is different).
        // -i ...: Input pattern for the frames.
        // -c:v libx264: Use standard H.264 codec for max compatibility.
        // -pix_fmt yuv420p: Ensure pixel format is compatible with all players.
        let status = Command::new("ffmpeg")
            .arg("-y")
            .arg("-r")
            .arg("24")
            .arg("-i")
            .arg(self.output_directory.join("frame_%04d.png"))
            .arg("-c:v")
            .arg("libx264")
            .arg("-vf")
            .arg("pad=ceil(iw/2)*2:ceil(ih/2)*2")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg(&final_video_path)
            .status()
            .map_err(SystemError::FfmpegExecution)?;

        if status.success() {
            log::info!("Video successfully saved to: {:?}", final_video_path);
        } else {
            log::error!("FFmpeg compilation failed with status: {}", status);
        }

        // Clean up temporary directories.
        fs::remove_dir_all(&self.input_directory)
            .map_err(SystemError::DirectoryRemove)
            .map_err(Error::System)?;

        fs::remove_dir_all(&self.output_directory)
            .map_err(SystemError::DirectoryRemove)
            .map_err(Error::System)
    }
}

impl Executable for VideoPipeline {
    fn supported_modes(&self) -> Vec<RunMode> {
        vec![
            RunMode::Sequential,
            RunMode::Pipeline,
            RunMode::ProducerConsumer,
        ]
    }

    fn run(&self, kind: BenchmarkKind) -> Result<(), Error> {
        let count = match kind {
            BenchmarkKind::Sequential => self.sequential(),
            BenchmarkKind::Pipeline { .. } => self.pipeline(),
            BenchmarkKind::ProducerConsumer { consumers, .. } => {
                self.producer_consumer(consumers)
            },
            _ => return Ok(()),
        }?;

        log::info!(
            "TASK: {}, MODE: {}, PROCESSED FRAMES: {}",
            self.name(),
            kind,
            count
        );
        Ok(())
    }
}

// Execution Modes
impl VideoPipeline {
    pub fn sequential(&self) -> Result<usize, Error> {
        let mut count = 0;
        for path in &self.frame_paths {
            let data = Self::decode_stage(path)?;
            let data = Self::filter_stage(data);
            let data = Self::watermark_stage(data, &self.watermark);
            Self::encode_stage(data, &self.output_directory)?;
            count += 1;
        }
        Ok(count)
    }

    pub fn pipeline(&self) -> Result<usize, Error> {
        // Crossbeam bounded channels for strict memory control between stages
        let cap = 5;
        let (raw_tx, raw_rx) = crossbeam::channel::bounded(cap);
        let (decoded_tx, decoded_rx) = crossbeam::channel::bounded(cap);
        let (filtered_tx, filtered_rx) = crossbeam::channel::bounded(cap);

        let (errors_tx, errors_rx) = crossbeam::channel::bounded(1);

        thread::scope(|s| {
            // Stage 1: Decode
            s.spawn(move || {
                for path in self.frame_paths.iter() {
                    let decoded = Self::decode_stage(path);

                    if raw_tx.send(decoded).is_err() {
                        break;
                    }
                }
            });

            // Stage 2: Filter
            s.spawn({
                let errors_tx = errors_tx.clone();
                move || {
                    for data in raw_rx {
                        let data = match data {
                            Ok(data) => data,
                            Err(error) => {
                                let _ = errors_tx.try_send(error);
                                break;
                            },
                        };

                        let filtered = Self::filter_stage(data);
                        if decoded_tx.send(filtered).is_err() {
                            break;
                        }
                    }
                }
            });

            // Stage 3: Watermark
            s.spawn(move || {
                for data in decoded_rx {
                    let watermarked = Self::watermark_stage(data, &self.watermark);
                    if filtered_tx.send(watermarked).is_err() {
                        break;
                    }
                }
            });

            // Stage 4: Encode
            s.spawn({
                let errors_tx = errors_tx.clone();
                move || {
                    let mut count = 0;
                    for data in filtered_rx {
                        match Self::encode_stage(data, &self.output_directory) {
                            Ok(_) => count += 1,
                            Err(error) => {
                                let _ = errors_tx.try_send(error);
                                break;
                            },
                        }
                    }
                    count
                }
            });
        });

        if let Ok(error) = errors_rx.try_recv() {
            return Err(error);
        }

        Ok(self.frame_paths.len())
    }

    pub fn producer_consumer(&self, consumers: usize) -> Result<usize, Error> {
        let cap = 20;
        let (tasks_tx, tasks_rx) = crossbeam::channel::bounded(cap);
        let (results_tx, results_rx) =
            crossbeam::channel::bounded(self.frame_paths.len());
        let (errors_tx, errors_rx) = crossbeam::channel::bounded(1);

        thread::scope(|s| {
            // Producer
            s.spawn(move || {
                for path in self.frame_paths.iter() {
                    if tasks_tx.send(path).is_err() {
                        break;
                    }
                }
            });

            // Consumers
            for _ in 0..consumers {
                let tasks_rx = tasks_rx.clone();
                let results_tx = results_tx.clone();
                let errors_tx = errors_tx.clone();

                let watermark = self.watermark.clone();
                let output_directory = self.output_directory.clone();

                s.spawn(move || {
                    for path in tasks_rx {
                        let data = match Self::decode_stage(path) {
                            Ok(data) => data,
                            Err(error) => {
                                let _ = errors_tx.try_send(error);
                                break;
                            },
                        };
                        let data = Self::filter_stage(data);
                        let data = Self::watermark_stage(data, &watermark);
                        match Self::encode_stage(data, &output_directory) {
                            Ok(_) => {
                                if results_tx.send(1).is_err() {
                                    break;
                                }
                            },
                            Err(error) => {
                                let _ = errors_tx.try_send(error);
                                break;
                            },
                        }
                    }
                });
            }
        });

        if let Ok(error) = errors_rx.try_recv() {
            return Err(error);
        }

        // CRITICAL FIX: Explicitly drop the original channel ends held by the main thread.
        // This ensures the receivers know no more messages will ever be sent, preventing infinite blocking.
        drop(tasks_rx);
        drop(results_tx);
        drop(errors_tx);

        let mut total = 0;
        for _ in results_rx {
            total += 1;
        }

        Ok(total)
    }
}

impl Reportable for VideoPipeline {
    fn name(&self) -> &'static str {
        "Video Processing Pipeline"
    }
}

impl Measurable for VideoPipeline {}
