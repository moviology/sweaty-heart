mod serial_reader;

use pyo3::prelude::*;
use serial_reader::SerialReader;


#[pyclass]
struct SignalProcessor {
    lag: usize,
    threshold: f64,
    influence: f64,
    ppg_signals: Vec<usize>,
    gsr_signals: Vec<usize>,
}

#[pymethods]
impl SignalProcessor {
    #[new]
    fn new(serial_port: String, lag: usize, threshold: f64, influence: f64) -> PyResult<Self> {
        Ok(Self {
            lag,
            threshold,
            influence,
            ppg_signals: Vec::new(),
            gsr_signals: Vec::new(),
        })
    }

    // fn start()

    // fn stop()

    // fn
}

/// A Python module implemented in Rust.
#[pymodule]
fn sweaty_heart(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<SerialReader>()?;
    m.add_class::<SignalProcessor>()?;
    Ok(())
}
