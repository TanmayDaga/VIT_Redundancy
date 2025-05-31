// use std::fmt::Debug;

// use crate::FloatValue;
// use num_traits::Float;
// use once_cell::sync::Lazy;
// use postgres::types::ToSql; // Removed self // Corrected import for FloatValue

// const SCALE: u8 = 16; // Increased scale for more precision with f64

// pub static INSERT_QUERY: Lazy<String> = Lazy::new(|| {
//     format!(
//         "WITH vals AS (
//         SELECT
//             $1::NUMERIC AS val_a, -- Let the database handle casting from text representation if needed
//             $2::NUMERIC AS val_b,
//             $3::NUMERIC AS val_r,
//             $4::TEXT AS l,
//             $5::TEXT AS m
//     ),
//     rounded_vals AS ( -- Apply rounding after initial cast
//         SELECT
//             ROUND(val_a, {scale}) AS a,
//             ROUND(val_b, {scale}) AS b,
//             ROUND(val_r, {scale}) AS r,
//             l,
//             m
//         FROM vals
//     ),
//     existing AS (
//         SELECT id FROM TANMAY, rounded_vals
//         WHERE ROUND(result, {scale}) = rounded_vals.r
//           AND ((ROUND(number_a, {scale}) = rounded_vals.a AND ROUND(number_b, {scale}) = rounded_vals.b) OR
//                (ROUND(number_a, {scale}) = rounded_vals.b AND ROUND(number_b, {scale}) = rounded_vals.a)) -- Check both orders
//           AND layer_name = rounded_vals.l
//           AND model_name = rounded_vals.m
//         LIMIT 1
//     ),
//     upd AS (
//         UPDATE TANMAY
//         SET count = count + 1
//         WHERE id IN (SELECT id FROM existing)
//         RETURNING *
//     )
//     INSERT INTO TANMAY (number_a, number_b, result, count, layer_name, model_name)
//     SELECT a, b, r, 1, l, m FROM rounded_vals
//     WHERE NOT EXISTS (SELECT * FROM upd) AND NOT EXISTS (SELECT * FROM existing WHERE upd.* IS NULL);", // Ensure not to insert if existing and not updated (e.g. due to some lock)
//         scale = SCALE
//     )
// });

// // The DBFloat trait might still be useful if you have other generic database functions,
// // but for the current insert_multiplication_data, it's not directly used with FloatValue.
// pub trait DBFloat<'a>: Debug + Float + Send + Sync + ToSql + 'a + Into<FloatValue> {}

// impl<'a> DBFloat<'a> for f32 {}
// impl<'a> DBFloat<'a> for f64 {}

// // Need to implement ToSql for FloatValue if we were to pass it directly
// // to `client.execute` in a context where its type isn't erased to `&dyn ToSql` immediately.
// // However, in `insert_multiplication_data`, we match and cast to `&dyn ToSql` manually.

// // Example of how you might implement ToSql for FloatValue if needed elsewhere:
// // impl ToSql for FloatValue {
// //     fn to_sql(&self, ty: &postgres::types::Type, out: &mut bytes::BytesMut) -> Result<postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
// //         match self {
// //             FloatValue::F32(v) => v.to_sql(ty, out),
// //             FloatValue::F64(v) => v.to_sql(ty, out),
// //         }
// //     }
// //     fn accepts(ty: &postgres::types::Type) -> bool {
// //         f32::accepts(ty) || f64::accepts(ty) // Or more specific NUMERIC checks
// //     }
// //     postgres::types::to_sql_checked!();
// // }

// // Ensure FloatValue is accessible if it's defined in lib.rs
// // pub use crate::lib::FloatValue;

use once_cell::sync::Lazy;

pub const SCALE: u32 = 7; // Increased scale for more precision with f64

pub static INSERT_QUERY: &'static str =
    "INSERT INTO TANMAY (number_a, number_b, result, `count`) VALUES (:a, :b, :r, 1);";
