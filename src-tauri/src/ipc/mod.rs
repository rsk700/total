mod messages;

use self::messages::{
    encoders::{
        json::{FromJson, ToJson},
        ldm::FromLdm,
    },
    value_base::ValueBase,
};
use crate::{core::Scanning, external_path};
use messages::leap_data_model as ldm;
pub use messages::types as ms;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use tauri::async_runtime::{spawn_blocking, Mutex as MutexAsync};

pub fn ipc_in<M>(message: String) -> M
where
    M: ValueBase,
    ldm::Value: FromLdm<M>,
{
    FromJson::new(message.as_bytes()).to_value().unwrap()
}

pub fn ipc_out<M>(value: M) -> Result<String, String>
where
    M: ValueBase,
    ldm::Value: FromLdm<M>,
{
    Ok(String::from_utf8(ToJson::from_value(&value)).unwrap())
}

pub fn ipc_err<M>(msg: M) -> Result<String, String>
where
    M: Into<String>,
{
    let msg: String = msg.into();
    Err(String::from_utf8(ToJson::from_value(&msg)).unwrap())
}

pub struct AppScanning(pub MutexAsync<Option<Arc<Mutex<Scanning>>>>);

#[tauri::command]
pub async fn start_scan(
    scanning: tauri::State<'_, AppScanning>,
    message: String,
) -> Result<String, String> {
    let path: String = ipc_in(message);
    let mut s = scanning.0.lock().await;
    *s = Some(Arc::new(Mutex::new(Scanning::new(path))));
    ipc_out(ms::structs::None {})
}

#[tauri::command]
pub async fn scan_step(
    scanning: tauri::State<'_, AppScanning>,
    message: String,
) -> Result<String, String> {
    let time_budget_ms: u128 = ipc_in::<i64>(message) as u128;
    let mut s = scanning.0.lock().await;
    let Some(s) = s.as_mut() else {
        return ipc_err("scan not started yet");
    };
    let start_time = Instant::now();
    while !s.lock().unwrap().scan_is_finished()
        && Instant::now().duration_since(start_time).as_millis() < time_budget_ms
    {
        let s = Arc::clone(s);
        spawn_blocking(move || {
            let mut s = s.lock().expect("internal error");
            s.scan_step();
        });
    }
    let s = s.lock().unwrap();
    let scan_state: ms::enums::ScanState = if s.scan_is_finished() {
        ms::enums::ScanState::Ready(ms::structs::None {})
    } else {
        ms::enums::ScanState::ScanProgress(ms::structs::ScanProgress::new(s.done_count() as i64))
    };
    ipc_out(scan_state)
}

#[tauri::command]
pub async fn get_aggregate_data(
    scanning: tauri::State<'_, AppScanning>,
    message: String,
) -> Result<String, String> {
    let up_to_fraction: f32 = ipc_in::<f64>(message) as f32;
    let mut s = scanning.0.lock().await;
    let Some(s) = s.as_mut() else {
        return ipc_err("scan not started yet");
    };
    let Ok(mut s) = s.lock() else {
        return ipc_err("internal error");
    };
    let agg = s.get_aggregate_data(up_to_fraction);
    ipc_out(agg)
}

#[tauri::command]
pub async fn open_path(message: String) -> Result<String, String> {
    let path: String = ipc_in(message);
    external_path::open_path(path);
    ipc_out(ms::structs::None {})
}
