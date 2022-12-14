use anyhow::{anyhow, bail};
use pyo3::{exceptions::PyTypeError, prelude::*};
use serialport::SerialPort;
use std::{io::BufRead, io::BufReader, time::Duration};

#[pyclass]
pub struct SerialReader {
    serial_port: String,
    baud_rate: u32,
    timeout: u64,
    pub connection: Option<Box<dyn SerialPort>>,
}

#[pymethods]
impl SerialReader {
    #[new]
    fn new(serial_port: String, baud_rate: Option<u32>, timeout: Option<u64>) -> PyResult<Self> {
        let br = baud_rate.unwrap_or(115_200);
        let to = timeout.unwrap_or(10);

        Python::with_gil(|_py| {
            Ok(Self {
                serial_port,
                baud_rate: br,
                connection: None,
                timeout: to,
            })
        })
    }

    fn open(&mut self) -> PyResult<()> {
        match serialport::new(&self.serial_port, self.baud_rate)
            .timeout(Duration::from_millis(self.timeout))
            .open()
        {
            Ok(conn) => Python::with_gil(|_py| self.connection = Some(conn)),
            Err(_) => {
                return Err(PyErr::new::<PyTypeError, _>(
                    "Connection to serial port failed!",
                ))
            }
        }
        Ok(())
    }

    pub fn read(&mut self) -> anyhow::Result<String> {
        // if self.connection.is_none() {
        //     return Err(anyhow!("No connection was opened!"));
        // }

        // let mut serial_buf: Vec<u8> = vec![0; 5];
        //
        // match self
        //     .connection
        //     .as_mut()
        //     .unwrap()
        //     .read(serial_buf.as_mut())
        // {
        //     Ok(v) => Ok(String::from_utf8_lossy(&serial_buf[..v]).to_string()),
        //     Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(String::from("")),
        //     Err(e) => Err(anyhow::Error::from(e)),
        // }
        let connection = match serialport::new(&self.serial_port, self.baud_rate)
            .timeout(Duration::from_millis(self.timeout))
            .open()
        {
            Ok(conn) => conn,
            Err(_) => bail!("Connection to serial port failed!"),
        };

        let mut reader = BufReader::new(connection);
        let mut my_str = String::new();
        match reader.read_line(&mut my_str) {
            Ok(_) => Ok(my_str),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(String::from("")),
            Err(e) => Err(anyhow::Error::from(e)),
        }
        // Ok(my_str)
    }

    fn close(&mut self) -> PyResult<()> {
        match self.connection {
            Some(_) => Python::with_gil(|_py| self.connection = None),
            None => return Err(PyErr::new::<PyTypeError, _>("Open connection not found!")),
        }
        Ok(())
    }
}
