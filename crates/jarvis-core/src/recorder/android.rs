//! Android Audio Recorder using Tauri Plugin
//! 
//! This module provides audio recording functionality for Android
//! using the Android AudioRecord API through a Tauri plugin.

use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

// Audio buffer for storing recorded frames
static AUDIO_BUFFER: OnceCell<Arc<Mutex<VecDeque<Vec<i16>>>>> = OnceCell::new();
static IS_RECORDING: AtomicBool = AtomicBool::new(false);
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static SAMPLE_RATE: OnceCell<i32> = OnceCell::new();
static FRAME_LENGTH: OnceCell<usize> = OnceCell::new();

/// Initialize the Android audio recorder
pub fn init_microphone(_device_index: i32, frame_length: u32) -> bool {
    if IS_INITIALIZED.load(Ordering::SeqCst) {
        return true;
    }

    // Initialize audio buffer
    let _ = AUDIO_BUFFER.set(Arc::new(Mutex::new(VecDeque::new())));
    
    // Set default values
    let _ = SAMPLE_RATE.set(16000);
    let _ = FRAME_LENGTH.set(frame_length as usize);
    
    IS_INITIALIZED.store(true, Ordering::SeqCst);
    
    info!("Android audio recorder initialized (frame_length: {})", frame_length);
    true
}

/// Read audio frame from buffer
/// This is called from the Rust side to get audio data
pub fn read_microphone(frame_buffer: &mut [i16]) {
    if !IS_INITIALIZED.load(Ordering::SeqCst) {
        error!("Android recorder not initialized");
        return;
    }

    if let Some(buffer) = AUDIO_BUFFER.get() {
        let mut queue = buffer.lock().unwrap();
        
        if let Some(frame) = queue.pop_front() {
            // Copy data to output buffer
            let len = frame_buffer.len().min(frame.len());
            frame_buffer[..len].copy_from_slice(&frame[..len]);
            
            // Fill remaining with zeros if needed
            if len < frame_buffer.len() {
                frame_buffer[len..].fill(0);
            }
        } else {
            // No data available, fill with zeros
            frame_buffer.fill(0);
        }
    }
}

/// Start recording
pub fn start_recording(device_index: i32, frame_length: u32) -> Result<(), ()> {
    init_microphone(device_index, frame_length);
    
    if IS_RECORDING.load(Ordering::SeqCst) {
        return Ok(());
    }

    // Note: Actual recording is started from the Android side via Tauri plugin
    // The plugin will emit audio data events that need to be captured
    
    IS_RECORDING.store(true, Ordering::SeqCst);
    info!("Android recording started");
    
    Ok(())
}

/// Stop recording
pub fn stop_recording() -> Result<(), ()> {
    if !IS_RECORDING.load(Ordering::SeqCst) {
        return Ok(());
    }

    IS_RECORDING.store(false, Ordering::SeqCst);
    
    // Clear buffer
    if let Some(buffer) = AUDIO_BUFFER.get() {
        buffer.lock().unwrap().clear();
    }
    
    info!("Android recording stopped");
    Ok(())
}

/// Check if recording
pub fn is_recording() -> bool {
    IS_RECORDING.load(Ordering::SeqCst)
}

/// Push audio data from Android plugin
/// This is called from the Tauri plugin when audio data is received
pub fn push_audio_data(data: Vec<i16>) {
    if !IS_RECORDING.load(Ordering::SeqCst) {
        return;
    }

    if let Some(buffer) = AUDIO_BUFFER.get() {
        let mut queue = buffer.lock().unwrap();
        
        // Keep buffer size limited to prevent memory issues
        if queue.len() > 100 {
            queue.pop_front();
        }
        
        queue.push_back(data);
    }
}

/// List available audio devices
/// On Android, we typically just have the built-in microphone
pub fn list_audio_devices() -> Vec<String> {
    vec![
        "Default Microphone".to_string(),
        "Built-in Microphone".to_string(),
    ]
}

/// Get audio device name
pub fn get_audio_device_name(idx: i32) -> String {
    match idx {
        -1 => "System Default".to_string(),
        0 => "Default Microphone".to_string(),
        1 => "Built-in Microphone".to_string(),
        _ => "Unknown Device".to_string(),
    }
}

/// Get sample rate
pub fn get_sample_rate() -> i32 {
    *SAMPLE_RATE.get().unwrap_or(&16000)
}

/// Get frame length
pub fn get_frame_length() -> usize {
    *FRAME_LENGTH.get().unwrap_or(&512)
}

/// Clear audio buffer
pub fn clear_buffer() {
    if let Some(buffer) = AUDIO_BUFFER.get() {
        buffer.lock().unwrap().clear();
    }
}
