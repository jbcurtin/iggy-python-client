mod client;
mod consumer;
mod identifier;
mod receive_message;
mod send_message;

use client::IggyClient;
use pyo3::prelude::*;
use receive_message::ReceiveMessage;
use send_message::SendMessage;

/// A Python module implemented in Rust.
#[pymodule]
fn iggy_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SendMessage>()?;
    m.add_class::<ReceiveMessage>()?;
    m.add_class::<IggyClient>()?;
    m.add_class::<consumer::Consumer>()?;
    m.add_class::<identifier::Identifier>()?;
    Ok(())
}
