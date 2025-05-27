use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam::channel::{Receiver, Sender, bounded};
use once_cell::sync::Lazy;
use pyo3::pyfunction;

use crate::database;

// Shared channel sender
pub static CHANNEL: Lazy<Mutex<Option<Sender<(Vec<(f32, f32, f32)>, Arc<String>, Arc<String>)>>>> =
    Lazy::new(|| Mutex::new(None));
// Stop flag
pub static STOP_FLAG: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
// Vector of thread handles
pub static HANDLES: Lazy<Mutex<Vec<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn init() -> Result<(), postgres::Error> {
    let (tx, rx): (
        Sender<(Vec<(f32, f32, f32)>, Arc<String>, Arc<String>)>,
        Receiver<(Vec<(f32, f32, f32)>, Arc<String>, Arc<String>)>,
    ) = bounded(10);

    *CHANNEL.lock().unwrap() = Some(tx);
    STOP_FLAG.store(false, Ordering::SeqCst);

    database::init()?; // Ensure DB initialized

    let arc_rx = Arc::new(rx);
    let mut handles = Vec::new();

    for _ in 0..3 {
        let thread_rx = arc_rx.clone();
        let handle = thread::spawn(move || {
            while !STOP_FLAG.load(Ordering::SeqCst) {
                if let Ok(data) = thread_rx.try_recv() {
                    for (x, y, z) in data.0 {
                        if let Err(e) =
                            database::insert_numbers(x, y, z, data.1.as_ref(), data.2.as_ref())
                        {
                            eprintln!("Failed to insert data into database: {}", e);
                        }
                    }
                } else {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });
        handles.push(handle);
    }

    *HANDLES.lock().unwrap() = handles;

    Ok(())
}

#[pyfunction]
pub fn stop() {
    STOP_FLAG.store(true, Ordering::SeqCst);

    let mut handles = HANDLES.lock().unwrap();
    for handle in handles.drain(..) {
        let _ = handle.join();
    }

    *CHANNEL.lock().unwrap() = None;
}
