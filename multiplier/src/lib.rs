use std::{env, sync::Arc};

use pyo3::prelude::*;

mod database;
mod logger;
mod utils;

use logger::CHANNEL;

// // use database::insert_numbers;

// use pyo3::prelude::*;
// use utils::DBFloat;

// use crossbeam::channel::unbounded;
// use once_cell::sync::OnceCell;

// use crossbeam::channel::{Receiver, Sender};
// use database::insert_multiplication_data; // Ensure this import is active
// use std::thread;

// pub enum FloatValue {
//     // Made public
//     F32(f32),
//     F64(f64),
// }

// static QUEUE: OnceCell<(
//     Sender<(FloatValue, FloatValue, FloatValue, String, String)>,
//     Receiver<(FloatValue, FloatValue, FloatValue, String, String)>,
// )> = OnceCell::new();

// // fn logger<'a, 'b, T>(num1: T, num2: T, result: T, layer_name: &'b str, model_name: &'b str)
// // where
// //     'b: 'a,
// //     T: DBFloat<'a>,
// // {
// //     // insert_numbers(num1, num2, result, layer_name, model_name);
// // }

// fn log_to_queue<'a, 'b, T>(vector_to_log: Vec<(T, T, T)>, layer_name: &'b str, model_name: &'b str)
// where
//     'b: 'a,
//     T: DBFloat<'a> + Into<FloatValue> + Copy, // Added Into<FloatValue> and Copy
// {
//     if let Some((s, _)) = QUEUE.get() {
//         for (a, b, c) in vector_to_log {
//             // Convert T to FloatValue before sending
//             let val_a = a.into();
//             let val_b = b.into();
//             let val_c = c.into();
//             s.send((
//                 val_a,
//                 val_b,
//                 val_c,
//                 layer_name.to_string(),
//                 model_name.to_string(),
//             ))
//             .unwrap();
//         }
//     }
// }

// // Implement Into<FloatValue> for f32 and f64
// impl From<f32> for FloatValue {
//     fn from(val: f32) -> Self {
//         FloatValue::F32(val)
//     }
// }

// impl From<f64> for FloatValue {
//     fn from(val: f64) -> Self {
//         FloatValue::F64(val)
//     }
// }

// fn mat_mul<'a, 'b, T>(
//     matrix_a: &Vec<Vec<T>>,
//     matrix_b: &Vec<Vec<T>>,
//     layer_name: &'b str,
//     model_name: &'b str,
// ) -> Result<Vec<Vec<T>>, &'static str>
// where
//     'b: 'a,
//     T: DBFloat<'a> + Into<FloatValue> + Copy, // Added Into<FloatValue> and Copy
// {
//     let a_rows = matrix_a.len();
//     let a_cols = matrix_a[0].len();
//     let b_rows = matrix_b.len();
//     let b_cols = matrix_b[0].len();

//     if a_rows == 0 || b_rows == 0 {
//         return Err("Dimensions not correct");
//     }

//     if a_cols != b_rows {
//         return Err("Matrix multiplication not possible");
//     }

//     // Transpose for better cache locality
//     let matrix_b_t: Vec<Vec<T>> = (0..b_cols)
//         .map(|j| (0..b_rows).map(|i| matrix_b[i][j]).collect())
//         .collect();

//     let result: Vec<Vec<T>> = matrix_a
//         .iter()
//         .map(|row_a| {
//             matrix_b_t
//                 .iter()
//                 .map(|col_b| {
//                     let mut sum = T::zero();
//                     let mut logs = vec![];
//                     for (a, b) in row_a.iter().zip(col_b.iter()) {
//                         let prod = *a * *b;
//                         logs.push((prod, *a, *b)); // Storing prod, a, b
//                         sum = sum + prod;
//                     }

//                     // Send to queue instead of direct logging
//                     log_to_queue(logs, layer_name, model_name);

//                     sum
//                 })
//                 .collect()
//         })
//         .collect();

//     Ok(result)
// }

// #[pyfunction]
// pub fn mat_mul_f32(
//     matrix_a: Vec<Vec<f32>>,
//     matrix_b: Vec<Vec<f32>>,
//     layer_name: String,
//     model_name: String,
// ) -> PyResult<Vec<Vec<f32>>> {
//     mat_mul(&matrix_a, &matrix_b, &layer_name, &model_name)
//         .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(err))
// }
// #[pyfunction]
// pub fn mat_mul_f64(
//     matrix_a: Vec<Vec<f64>>,
//     matrix_b: Vec<Vec<f64>>,
//     layer_name: String,
//     model_name: String,
// ) -> PyResult<Vec<Vec<f64>>> {
//     mat_mul(&matrix_a, &matrix_b, &layer_name, &model_name)
//         .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(err))
// }

// #[pyfunction]
// pub fn init() -> PyResult<()> {
//     println!("Initializing multiplier module...");

//     if QUEUE.get().is_none() {
//         let (s, r) = unbounded::<(FloatValue, FloatValue, FloatValue, String, String)>();
//         if QUEUE.set((s, r)).is_err() {
//             return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
//                 "Failed to set up queue.",
//             ));
//         }
//     }

//     // Initialize database connection
//     if let Err(e) = database::connect_database() {
//         return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
//             "Failed to connect to database: {}",
//             e
//         )));
//     }
//     println!("Database connection established.");

//     // Spawn a thread to process the queue
//     let (_, r) = QUEUE.get().expect("Queue should be initialized").clone(); // Clone receiver for the new thread
//     thread::spawn(move || {
//         println!("Worker thread started. Waiting for data...");
//         loop {
//             match r.recv() {
//                 Ok((val_a, val_b, val_c, layer_name, model_name)) => {
//                     // println!("Received data: {:?}, {:?}, {:?}, {}, {}", val_a, val_b, val_c, layer_name, model_name);
//                     if let Err(e) = database::insert_multiplication_data(
//                         val_a,
//                         val_b,
//                         val_c,
//                         &layer_name,
//                         &model_name,
//                     ) {
//                         eprintln!("Failed to insert data into database: {}", e);
//                     } else {
//                         // println!("Data inserted successfully.");
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!(
//                         "Error receiving from queue: {}. Worker thread shutting down.",
//                         e
//                     );
//                     break; // Exit loop if channel is disconnected
//                 }
//             }
//         }
//     });
//     println!("Worker thread spawned.");

//     println!("Initialization complete.");
//     Ok(())
// }

// /// PyO3 module definition
// #[pymodule]
// fn multiplier(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(mat_mul_f32, m)?)?;
//     m.add_function(wrap_pyfunction!(mat_mul_f64, m)?)?;
//     m.add_function(wrap_pyfunction!(init, m)?)?;
//     Ok(())
// }

use rayon::{ThreadPoolBuilder, prelude::*};
use utils::INSERT_QUERY;

#[pyfunction]
fn mat_mul<'b>(
    matrix_a: Vec<Vec<f32>>,
    matrix_b: Vec<Vec<f32>>,
    layer_name: String,
    model_name: String,
) -> PyResult<Vec<Vec<f32>>> {
    let no_of_threads: u8 = env::var("NO_OF_PROCESSING_THREADS")
        .unwrap()
        .parse()
        .unwrap();

    let a_rows = matrix_a.len();
    let a_cols = matrix_a[0].len();
    let b_rows = matrix_b.len();
    let b_cols = matrix_b[0].len();
    let table_name = format!("TANMAY_{}_{}", layer_name, model_name);
    let insert_query = Arc::new(INSERT_QUERY.replace(
        "TANMAY",
        &table_name,
    ));
    println!("{:?}", insert_query);

    let pool = ThreadPoolBuilder::new()
        .num_threads(no_of_threads as usize)
        .build()
        .unwrap();

    if a_rows == 0 || b_rows == 0 {
        // return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
        //     //                 "Failed to set up queue.",
        //     //             ));

        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "Dimensions not correct",
        ));
    }

    if a_cols != b_rows {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "Matrix multiplication not possible",
        ));
    }

    // Transpose for better cache locality
    let matrix_b_t: Vec<Vec<f32>> = (0..b_cols)
        .map(|j| (0..b_rows).map(|i| matrix_b[i][j]).collect())
        .collect();

    database::create_table(&table_name);
   
    pool.install(|| {
        let result: Vec<Vec<f32>> = matrix_a
            .par_iter()
            .map(|row_a| {
                matrix_b_t
                    .par_iter()
                    .map(|col_b| {
                        let mut sum = 0f32;
                        let mut logs = vec![];

                        for (a, b) in row_a.iter().zip(col_b.iter()) {
                            let prod = *a * *b;
                            logs.push((prod, *a, *b)); // Storing prod, a, b
                            sum = sum + prod;
                        }

                        // Send to queue instead of direct logging
                        if let Some(tx) = &*CHANNEL.lock().unwrap() {
                            let _ = tx.send((logs, insert_query.clone()));
                        }
                        sum
                    })
                    .collect()
            })
            .collect();

        Ok(result)
    })
}

#[pyfunction]
pub fn init() -> PyResult<()> {
    dotenv::dotenv().ok();
    logger::init();
    Ok(())
}

/// PyO3 module definition
#[pymodule]
fn multiplier(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mat_mul, m)?)?;
    m.add_function(wrap_pyfunction!(logger::stop, m)?)?;
    m.add_function(wrap_pyfunction!(init, m)?)?;
    Ok(())
}
