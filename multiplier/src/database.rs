// use crate::utils::{DBFloat, INSERT_QUERY};
// use once_cell::sync::OnceCell;
// use postgres::{Client, Error, NoTls};
// use std::sync::Mutex; // Changed from MutexGuard to Mutex for the static DB_CLIENT
// use crate::lib::FloatValue; // Import FloatValue from lib.rs

// // Global connection holder
// static DB_CLIENT: OnceCell<Mutex<Client>> = OnceCell::new();

// // Initialize DB client and create table
// pub fn connect_database() -> Result<(), Error> {
//     if DB_CLIENT.get().is_some() {
//         // Already initialized
//         return Ok(());
//     }

//     let mut client = Client::connect(
//         "host=localhost user=postgres password=tanmaydaga dbname=postgres", // Ensure this is correct
//         NoTls,
//     )?;

//     client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS TANMAY (
//             id SERIAL PRIMARY KEY,
//             number_a NUMERIC,     -- Removed precision and scale for flexibility with f32/f64
//             number_b NUMERIC,     -- Removed precision and scale
//             result   NUMERIC,     -- Removed precision and scale
//             count BIGINT,
//             layer_name TEXT,
//             model_name TEXT
//         )
//     ",
//     )?;

//     if DB_CLIENT.set(Mutex::new(client)).is_err() {
//         // This case should ideally not happen if we check DB_CLIENT.get().is_some() first,
//         // but as a fallback, we create a custom error.
//         return Err(Error::connect(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             "DB already initialized during set attempt",
//         )));
//     }
//     Ok(())
// }

// // pub fn get_db_client() -> MutexGuard<'static, Client> { // MutexGuard is not Send, cannot be held across .await
// //     DB_CLIENT
// //         .get()
// //         .expect("Call connect_database() first")
// //         .lock()
// //         .unwrap()
// // }

// // Insert one row using FloatValue
// pub fn insert_multiplication_data(
//     num_a: FloatValue,
//     num_b: FloatValue,
//     result: FloatValue,
//     layer_name: &str,
//     model_name: &str,
// ) -> Result<(), Error> {
//     let client_mutex = DB_CLIENT.get().expect("Database not initialized. Call connect_database() first.");
//     let mut client = client_mutex.lock().unwrap(); // Lock the mutex to get access to the Client

//     // Using a closure to handle the ToSql conversion for FloatValue
//     let to_sql_f32 = |f: &f32| f as &dyn ToSql;
//     let to_sql_f64 = |d: &f64| d as &dyn ToSql;

//     let params_a: &dyn ToSql = match num_a {
//         FloatValue::F32(ref val) => to_sql_f32(val),
//         FloatValue::F64(ref val) => to_sql_f64(val),
//     };
//     let params_b: &dyn ToSql = match num_b {
//         FloatValue::F32(ref val) => to_sql_f32(val),
//         FloatValue::F64(ref val) => to_sql_f64(val),
//     };
//     let params_c: &dyn ToSql = match result {
//         FloatValue::F32(ref val) => to_sql_f32(val),
//         FloatValue::F64(ref val) => to_sql_f64(val),
//     };

//     // println!(
//     //     "Executing insert: {:?}, {:?}, {:?}, {}, {}",
//     //     params_a, params_b, params_c, layer_name, model_name
//     // );

//     client.execute(
//         &*INSERT_QUERY, // Dereference Lazy<String> to &str
//         &[&params_a, &params_b, &params_c, &layer_name, &model_name],
//     )?;
//     Ok(())
// }

// // Commented out old insert_numbers as it's replaced by insert_multiplication_data
// // pub fn insert_numbers<'a, 'b, T>(
// //     num_a: T,
// //     num_b: T,
// //     result: T,
// //     layer_name: &'b str,
// //     model_name: &'b str,
// // ) where
// //     'b: 'a,
// //     T: DBFloat<'a>,
// // {
// //     println!(
// //         "{:?} {:?} {:?} {} {}",
// //         num_a, num_b, result, layer_name, model_name
// //     );

// //     let client = get_db_client();

// //     client
// //         .(
// //             &*INSERT_QUERY, // You must declare this as a `static` string in `utils`
// //             &[&num_a, &num_b, &result, &layer_name, &model_name],
// //         )
// //         .unwrap();
// // }

// use std::sync::{Mutex, MutexGuard};

// use num_traits::FromPrimitive;
// use once_cell::sync::Lazy;
// use postgres::{Client, NoTls};
// use rust_decimal::Decimal;

// use crate::utils::INSERT_QUERY;

// static DB_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
//     let client = Client::connect(
//         "host=localhost user=postgres password=tanmaydaga dbname=postgres",
//         NoTls,
//     )
//     .expect("DB connection failed");
//     Mutex::new(client)
// });

// fn get_db_client() -> MutexGuard<'static, Client> {
//     // MutexGuard is not Send, cannot be held across .await
//     DB_CLIENT.lock().unwrap()
// }

// pub fn init() -> Result<(), postgres::Error> {
//     get_db_client().batch_execute(
//         " CREATE TABLE IF NOT EXISTS TANMAY (
//              id SERIAL PRIMARY KEY,
//              number_a NUMERIC,     -- Removed precision and scale for flexibility with f32/f64
//              number_b NUMERIC,     -- Removed precision and scale
//              result   NUMERIC,     -- Removed precision and scale
//              count BIGINT,
//              layer_name TEXT,
//              model_name TEXT
//         )",
//     )?;
//     Ok(())
// }

// pub fn insert_numbers(
//     num_a: f32,
//     num_b: f32,
//     result: f32,
//     layer_name: &str,
//     model_name: &str,
// ) -> Result<(), postgres::Error> {
//     let mut client = get_db_client();
//     // Convert all f32 to Decimal
//     let dec_num_a = Decimal::from_f32(num_a).unwrap();
//     let dec_num_b = Decimal::from_f32(num_b).unwrap();
//     let dec_result = Decimal::from_f32(result).unwrap();

//     client.execute(
//         &*INSERT_QUERY, // You must declare this as a `static` string in `utils`
//         &[
//             &dec_num_a,
//             &dec_num_b,
//             &dec_result,
//             &layer_name,
//             &model_name,
//         ],
//     )?;
//     Ok(())
// }

use std::sync::Mutex;

use mysql::prelude::*;
use mysql::*;
use num_traits::FromPrimitive;
use once_cell::sync::Lazy;
use rust_decimal::Decimal;
use crate::utils;


static DB_POOL: Lazy<Mutex<Pool>> = Lazy::new(|| {
    let url = "mysql://tanmay:12345678@localhost:3306/tanmaydaga";
    let pool = Pool::new(url).expect("Failed to connect to DB");
    Mutex::new(pool)
});

fn get_db_conn() -> PooledConn {
    DB_POOL
        .lock()
        .unwrap()
        .get_conn()
        .expect("Failed to get conn")
}

pub fn create_table(table_name: &str) {
    let mut query = format!(
        r"CREATE TABLE IF NOT EXISTS {} (
            id INT AUTO_INCREMENT PRIMARY KEY,
            number_a DECIMAL(14,9),
            number_b DECIMAL(14,9),
            result DECIMAL(14,9),
            count BIGINT)
        ",
        table_name
    );

    let mut conn = get_db_conn();
    conn.query_drop(&query).unwrap();

    // CREATING PROCEDURE
    
    query = utils::INSERT_QUERY.replace("TANMAY", table_name);
    conn.query_drop(&query).unwrap();



}

pub fn insert_numbers(num_a: f32, num_b: f32, result: f32 ) -> Result<(), &'static str> {
    let mut conn = get_db_conn();
    let dec_num_a = Decimal::from_f32(num_a).unwrap();
    let dec_num_b = Decimal::from_f32(num_b).unwrap();
    let dec_result = Decimal::from_f32(result).unwrap();

    conn.exec_drop(
        "CALL ProcessTanmay(:a, :b, :r)",
        params! {
            
            "a" => dec_num_a,
            "b" => dec_num_b,
            "r" => dec_result,
        },
    )
    .unwrap();

    Ok(())
}
