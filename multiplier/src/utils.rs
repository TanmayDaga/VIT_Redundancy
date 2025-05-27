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

const SCALE: u8 = 5; // Increased scale for more precision with f64

pub static INSERT_QUERY: Lazy<String> = Lazy::new(|| {
    format!(
        r#"WITH input_params AS (
            SELECT
                $1::NUMERIC(1000,12) AS val_a_in,
                $2::NUMERIC(1000,12) AS val_b_in,
                $3::NUMERIC(1000,12) AS val_r_in,
                $4::TEXT AS l_in,
                $5::TEXT AS m_in
        ),
        rounded_params AS (
            SELECT
                ROUND(val_a_in, {scale}) AS final_a,
                ROUND(val_b_in, {scale}) AS final_b,
                ROUND(val_r_in, {scale}) AS final_r,
                l_in AS final_l,
                m_in AS final_m
            FROM input_params
        ),
        -- Perform the existence check and retrieve the ID of the row to update
        row_to_update AS (
            SELECT t.id
            FROM TANMAY t, rounded_params rp
            WHERE ROUND(t.result, {scale}) = rp.final_r
              AND t.layer_name = rp.final_l
              AND t.model_name = rp.final_m
              AND (
                    (ROUND(t.number_a, {scale}) = rp.final_a AND ROUND(t.number_b, {scale}) = rp.final_b) OR
                    (ROUND(t.number_a, {scale}) = rp.final_b AND ROUND(t.number_b, {scale}) = rp.final_a)
                  )
            LIMIT 1 -- Ensure we target at most one row for update
        ),
        -- Attempt to update the row if one was found
        updated_row AS (
            UPDATE TANMAY
            SET count = count + 1
            WHERE id = (SELECT id FROM row_to_update) -- This will not update if row_to_update is empty (id would be NULL)
            RETURNING id -- We need to know if an update actually happened
        )
        -- Insert a new row if the update did not occur (i.e., updated_row CTE is empty)
        INSERT INTO TANMAY (number_a, number_b, result, count, layer_name, model_name)
        SELECT rp.final_a, rp.final_b, rp.final_r, 1, rp.final_l, rp.final_m
        FROM rounded_params rp
        WHERE NOT EXISTS (SELECT 1 FROM updated_row);
        "#,
        scale = SCALE
    )
});
