use anyhow::anyhow;
use pubnub_hyper::runtime::tokio_global::TokioGlobal;
use pubnub_hyper::runtime::trasnport::hyper::Hyper;
use pubnub_hyper::{core::json::object, Builder};
use pyo3::{exceptions::PyTypeError, prelude::*};

#[pyclass]
pub struct Pubnub {
    publish_key: String,
    subscribe_key: String,
    user_id: String,
    secret_key: String,
}

#[pymethods]
impl Pubnub {
    #[new]
    fn new(
        publish_key: String,
        subscribe_key: String,
        user_id: String,
        secret_key: String,
    ) -> PyResult<Self> {
        Python::with_gil(|_py| {
            Ok(Self {
                publish_key,
                subscribe_key,
                user_id,
                secret_key,
            })
        })
    }
}
