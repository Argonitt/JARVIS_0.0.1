use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub mod mobile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioData {
    pub data: Vec<i16>,
    pub read: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: i32,
    pub buffer_size: i32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            buffer_size: 512,
        }
    }
}

static AUDIO_PLUGIN: OnceCell<Arc<Mutex<JarvisAudioPlugin>>> = OnceCell::new();

pub struct JarvisAudioPlugin {
    is_recording: bool,
    config: AudioConfig,
    audio_buffer: Vec<Vec<i16>>,
}

impl JarvisAudioPlugin {
    pub fn new() -> Self {
        Self {
            is_recording: false,
            config: AudioConfig::default(),
            audio_buffer: Vec::new(),
        }
    }

    pub fn start_recording(&mut self) -> Result<(), String> {
        self.is_recording = true;
        self.audio_buffer.clear();
        log::info!("Audio recording started");
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<(), String> {
        self.is_recording = false;
        self.audio_buffer.clear();
        log::info!("Audio recording stopped");
        Ok(())
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn get_config(&self) -> AudioConfig {
        self.config.clone()
    }

    pub fn push_audio_data(&mut self, data: Vec<i16>) {
        if self.is_recording {
            self.audio_buffer.push(data);
        }
    }

    pub fn get_audio_data(&mut self) -> Option<Vec<i16>> {
        if !self.audio_buffer.is_empty() {
            Some(self.audio_buffer.remove(0))
        } else {
            None
        }
    }
}

// Tauri Commands
#[tauri::command]
async fn start_recording<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    if let Some(plugin) = AUDIO_PLUGIN.get() {
        plugin.lock().unwrap().start_recording()
    } else {
        Err("Audio plugin not initialized".to_string())
    }
}

#[tauri::command]
async fn stop_recording<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(), String> {
    if let Some(plugin) = AUDIO_PLUGIN.get() {
        plugin.lock().unwrap().stop_recording()
    } else {
        Err("Audio plugin not initialized".to_string())
    }
}

#[tauri::command]
async fn is_recording<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<bool, String> {
    if let Some(plugin) = AUDIO_PLUGIN.get() {
        Ok(plugin.lock().unwrap().is_recording())
    } else {
        Err("Audio plugin not initialized".to_string())
    }
}

#[tauri::command]
async fn get_audio_config<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<AudioConfig, String> {
    if let Some(plugin) = AUDIO_PLUGIN.get() {
        Ok(plugin.lock().unwrap().get_config())
    } else {
        Err("Audio plugin not initialized".to_string())
    }
}

#[tauri::command]
async fn check_permission<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<bool, String> {
    mobile::check_permission()
}

#[tauri::command]
async fn request_permission<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<bool, String> {
    mobile::request_permission()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    // Initialize the global plugin instance
    let _ = AUDIO_PLUGIN.set(Arc::new(Mutex::new(JarvisAudioPlugin::new())));

    Builder::new("jarvis-audio")
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            is_recording,
            get_audio_config,
            check_permission,
            request_permission,
        ])
        .setup(|_app| {
            log::info!("JARVIS Audio Plugin initialized");
            Ok(())
        })
        .build()
}

// Public API for use in other crates
pub fn get_plugin() -> Option<Arc<Mutex<JarvisAudioPlugin>>> {
    AUDIO_PLUGIN.get().cloned()
}

pub fn init_plugin() {
    let _ = AUDIO_PLUGIN.set(Arc::new(Mutex::new(JarvisAudioPlugin::new())));
}

/// Push audio data from platform-specific code
pub fn push_audio_data(data: Vec<i16>) {
    if let Some(plugin) = AUDIO_PLUGIN.get() {
        plugin.lock().unwrap().push_audio_data(data);
    }
}
