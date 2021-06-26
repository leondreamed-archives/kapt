#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod audio;
mod kapture;
mod recording;
mod state;
mod utils;

use audio::AudioSource;
use lazy_static::lazy_static;
use state::KaptState;
use std::sync::RwLock;
lazy_static! {
  static ref KAPT_STATE: RwLock<KaptState> = RwLock::new(KaptState::new());
}

#[tauri::command]
async fn deactivate_kapt() {
  recording::deactivate_kapt(&*KAPT_STATE).await;
}

#[tauri::command]
async fn activate_kapt(audio_source: usize) {
  recording::start_recording(&*KAPT_STATE, audio_source).await;
}

#[tauri::command]
// timestamp - Unix timestamp of when the user pressed the Kapture button (in seconds)
async fn create_kapture(timestamp: i64) -> String {
  kapture::create_kapture(&*KAPT_STATE, timestamp as u128).await
}

#[tauri::command]
fn get_audio_sources() -> Vec<AudioSource> {
  audio::get_audio_sources()
}

fn main() {
  tauri::Builder::default()
    .manage(&*KAPT_STATE)
    .invoke_handler(tauri::generate_handler![
      activate_kapt,
      deactivate_kapt,
      create_kapture,
      get_audio_sources
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
