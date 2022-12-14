mod serial_reader;

use anyhow::{anyhow, bail, Result};
use pyo3::{exceptions::PyTypeError, prelude::*, types::PyList};
use rayon::prelude::*;
use retry::{delay::Fixed, retry};
use serial_reader::SerialReader;
use std::{
    borrow::{Borrow, BorrowMut},
    io::Error,
    sync::{Arc, Mutex},
};

fn smooth_signal(lag: usize, signal: Vec<usize>) -> Vec<f32> {
    let mut smoothed = vec![0.0; signal.len()];
    let mut avg = 0.0;
    for i in 0..signal.len() {
        avg += signal[i] as f32;
        if i >= lag {
            avg -= signal[i - lag] as f32;
        }
        smoothed[i] = avg / lag as f32;
    }
    smoothed
}

fn mean(data: &Vec<f32>) -> Option<f32> {
    let sum = data.iter().par_bridge().sum::<f32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &Vec<f32>) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .into_par_iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);
                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

#[pyclass]
struct SignalProcessor {
    lag: usize,
    threshold: f32,
    influence: f32,
    ppg_signals: Arc<Mutex<Vec<usize>>>,
    gsr_signals: Arc<Mutex<Vec<usize>>>,
    entries: usize,
}

#[pymethods]
impl SignalProcessor {
    #[new]
    fn new(lag: usize, threshold: f32, influence: f32) -> PyResult<Self> {
        Ok(Self {
            lag,
            threshold,
            influence,
            ppg_signals: Arc::new(Mutex::new(Vec::new())),
            gsr_signals: Arc::new(Mutex::new(Vec::new())),
            entries: 0,
        })
    }

    fn smoothed_z_score(&mut self, y: Vec<f32>) -> Result<Vec<usize>> {
        if y.len() < self.lag + 2 {
            bail!("Data length is too short");
        }

        let mut peak_idx: Vec<usize> = vec![0; y.len()];
        let mut filtered_y = y[0..].to_vec();
        let mut lead_in = y[0..self.lag].to_vec();
        let mut avg_filter: Vec<f32> = vec![0.; y.len()];
        let mut std_filter: Vec<f32> = vec![0.; y.len()];

        avg_filter[self.lag - 1] = mean(&lead_in).unwrap();
        std_filter[self.lag - 1] = std_deviation(&lead_in).unwrap();

        for i in self.lag..y.len() {
            if (y[i] - avg_filter[i - 1]).abs() > self.threshold * std_filter[i - 1] {
                if y[i] > avg_filter[i - 1] {
                    peak_idx[i] = 1;
                } else {
                    peak_idx[i] = 2;
                }

                filtered_y[i] = avg_filter[i - 1] + self.influence * (y[i] - avg_filter[i - 1]);
            } else {
                peak_idx[i] = 0;
                filtered_y[i] = y[i];
            }

            let y_lag = filtered_y[i - self.lag..i].to_vec();
            avg_filter[i] = mean(&y_lag).unwrap();
            std_filter[i] = std_deviation(&y_lag).unwrap();
        }

        // count the number of peaks in the array
        let peak_0 = peak_idx.clone().into_par_iter().filter(|&x| x == 0).count();
        let peak_1 = peak_idx.clone().into_par_iter().filter(|&x| x == 1).count();
        let peak_2 = peak_idx.clone().into_par_iter().filter(|&x| x == 2).count();

        Ok(vec![peak_0, peak_1, peak_2])
    }

    fn run(&mut self, data: Vec<String>) -> Result<Vec<f32>> {
        // split the data into a vector of strings using the comma as delimiter
        // append the first element of the vector to the ppg_signals vector
        // append the second element of the vector to the gsr_signals vector
        data.into_par_iter().for_each(|line| {
            let signals = line.split(',').collect::<Vec<&str>>();
            let ppg_signal = signals[0].parse::<usize>().unwrap();
            let gsr_signal = signals[1].parse::<usize>().unwrap();
            self.ppg_signals.lock().unwrap().push(ppg_signal);
            self.gsr_signals.lock().unwrap().push(gsr_signal);
        });

        // smooth the data in ppg_signals vector
        let smoothed_ppg = smooth_signal(self.lag, self.ppg_signals.lock().unwrap().to_vec());
        // let peaks = self.smoothed_z_score(smoothed_ppg)?;

        // let output: Vec<_> = smoothed_ppg
        //     .into_iter()
        //     .enumerate()
        //     .peaks(PeaksDetector::new(30, 5.0, 0.0), |e| e.1.into())
        //     .map(|((i, _), p)| i)
        //     .collect();

        Ok(smoothed_ppg)
    }

    // fn start()

    // fn stop()

    // fn
}

/// A Python module implemented in Rust.
#[pymodule]
fn sweaty_heart(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SerialReader>()?;
    m.add_class::<SignalProcessor>()?;
    Ok(())
}
