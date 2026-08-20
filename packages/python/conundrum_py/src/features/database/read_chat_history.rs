use pyo3::prelude::*;

#[pyfunction]
fn get_chat_history() -> String {
    "Hello from conundrum-py!".to_string()
}
