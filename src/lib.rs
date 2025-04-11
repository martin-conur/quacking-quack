extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{
    core::{DataChunkHandle, LogicalTypeId},
    vscalar::{ScalarFunctionSignature, VScalar},
    vtab::arrow::WritableVector,
    Connection, Result,
};
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use std::error::Error;
use libduckdb_sys::{
    duckdb_string_t,
    duckdb_string_t_data,
    duckdb_string_t_length,
};
use duckdb::core::Inserter;
use duckdb::ffi;

use rust_stemmers::{Algorithm, Stemmer};
struct StemFunc;

impl VScalar for StemFunc {
    type State = ();

    unsafe fn invoke(
        _state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Extract the input word
        let input_vec = input.flat_vector(0);
        // slice of strings
        let input_slice = input_vec.as_slice_with_len::<duckdb_string_t>(input.len());
        // a flat writable vector
        let output_flat = output.flat_vector();

        // stemmer algorithm
        let stemmer = Stemmer::create(Algorithm::English);

        // stem all the words in the input slice
        // map the input slice to a vector of stemmed words
        let stemmed_words: Vec<String> = input_slice
            .iter()
            .map(|word| {
                let len = duckdb_string_t_length(*word);
                let c_ptr = duckdb_string_t_data(word as *const _ as *mut _);
                let string = String::from_utf8_lossy(std::slice::from_raw_parts(
                    c_ptr as *const u8,
                    len as usize,
                ));
                stem_word(string.as_ref(), &stemmer)
            })
            .collect::<Vec<String>>();

        for (i, stemmed_word) in stemmed_words.iter().enumerate() {
            output_flat.insert(i, stemmed_word.as_str())
        }

        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into()],
            LogicalTypeId::Varchar.into(),
        )]
    }
}

fn stem_word(word: &str, stemmer: &Stemmer) -> String {
    stemmer.stem(word).to_string()
}

const FUNCTION_NAME: &str = "quacking_quack";

#[duckdb_entrypoint_c_api]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    con.register_scalar_function::<StemFunc>(FUNCTION_NAME)
        .expect("Failed to register quacking_quack()");
    Ok(())
}