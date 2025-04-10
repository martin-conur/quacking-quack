extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{
    core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId},
    vtab::{BindInfo, InitInfo, TableFunctionInfo, VTab},
    Connection, Result,
};
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use libduckdb_sys as ffi;
use std::{
    error::Error,
    ffi::CString,
    sync::atomic::{AtomicUsize, Ordering},
};

#[repr(C)]
struct HelloBindData {
    name: String,
    repeat_count: usize
}

#[repr(C)]
struct HelloInitData {
    done: AtomicUsize,
}

struct HelloVTab;

impl VTab for HelloVTab {
    type InitData = HelloInitData;
    type BindData = HelloBindData;

    fn bind(bind: &BindInfo) -> Result<Self::BindData, Box<dyn std::error::Error>> {
        bind.add_result_column("column0", LogicalTypeHandle::from(LogicalTypeId::Varchar));
        let name = bind.get_parameter(0).to_string();
        let repeat_count = bind.get_parameter(1).to_int64() as usize;
        Ok(HelloBindData { name, repeat_count })
    }

    fn init(_: &InitInfo) -> Result<Self::InitData, Box<dyn std::error::Error>> {
        Ok(HelloInitData {
            done: AtomicUsize::new(0),
        })
    }

    fn func(func: &TableFunctionInfo<Self>, output: &mut DataChunkHandle) -> Result<(), Box<dyn std::error::Error>> {
        let init_data = func.get_init_data();
        let bind_data = func.get_bind_data();

        let current_row = init_data.done.load(Ordering::Relaxed);
        let repeat_count = bind_data.repeat_count;

        if current_row >= repeat_count {
            output.set_len(0);
        } else {
            let rows_to_produce = std::cmp::min(1024,repeat_count - current_row);
            let vector = output.flat_vector(0);

            for i in 0..rows_to_produce {
                let row_value = CString::new(format!("Rusty Quack {} 🐥", bind_data.name))?;
                vector.insert(i, row_value);
            }
            init_data
                .done
                .fetch_add(rows_to_produce, Ordering::Relaxed);
            output.set_len(rows_to_produce);
        }
        Ok(())
    }

    fn parameters() -> Option<Vec<LogicalTypeHandle>> {
        Some(vec![
            LogicalTypeHandle::from(LogicalTypeId::Varchar),
            LogicalTypeHandle::from(LogicalTypeId::Integer),
        ])
    }
}

const EXTENSION_NAME: &str = env!("CARGO_PKG_NAME");

#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    con.register_table_function::<HelloVTab>(EXTENSION_NAME)
        .expect("Failed to register hello table function");
    Ok(())
}