package com.jarvis.audio

import android.Manifest
import android.content.pm.PackageManager
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder
import android.os.Handler
import android.os.Looper
import android.util.Log
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import com.google.gson.Gson
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicBoolean

@InvokeArg
class AudioConfigArgs {
    var sampleRate: Int = 16000
    var bufferSize: Int = 512
}

@InvokeArg
class StartRecordingArgs {
    var deviceIndex: Int = -1
    var frameLength: Int = 512
}

@TauriPlugin
class AudioPlugin(private val activity: app.tauri.plugin.Activity) : Plugin(activity) {
    companion object {
        private const val TAG = "JarvisAudioPlugin"
        private const val PERMISSION_REQUEST_CODE = 1001
        private const val SAMPLE_RATE = 16000
        private const val CHANNEL_CONFIG = AudioFormat.CHANNEL_IN_MONO
        private const val AUDIO_FORMAT = AudioFormat.ENCODING_PCM_16BIT
    }

    private var audioRecord: AudioRecord? = null
    private var bufferSize: Int = 0
    private val isRecording = AtomicBoolean(false)
    private val executor = Executors.newSingleThreadExecutor()
    private val mainHandler = Handler(Looper.getMainLooper())
    private val gson = Gson()
    
    // Audio buffer queue for streaming to Rust
    private val audioBuffer = java.util.concurrent.LinkedBlockingQueue<ShortArray>(100)

    override fun load() {
        Log.d(TAG, "AudioPlugin loaded")
        bufferSize = AudioRecord.getMinBufferSize(SAMPLE_RATE, CHANNEL_CONFIG, AUDIO_FORMAT)
        Log.d(TAG, "Min buffer size: $bufferSize")
    }

    @Command
    fun checkPermission(invoke: Invoke) {
        val hasPermission = ContextCompat.checkSelfPermission(
            activity,
            Manifest.permission.RECORD_AUDIO
        ) == PackageManager.PERMISSION_GRANTED
        
        invoke.resolve(java.util.HashMap<String, Any>().apply {
            put("granted", hasPermission)
        })
    }

    @Command
    fun requestPermission(invoke: Invoke) {
        val hasPermission = ContextCompat.checkSelfPermission(
            activity,
            Manifest.permission.RECORD_AUDIO
        ) == PackageManager.PERMISSION_GRANTED
        
        if (hasPermission) {
            invoke.resolve(java.util.HashMap<String, Any>().apply {
                put("granted", true)
            })
            return
        }
        
        // Request permission
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.RECORD_AUDIO),
            PERMISSION_REQUEST_CODE
        )
        
        // For simplicity, resolve with current status
        // In production, you should handle the permission callback
        invoke.resolve(java.util.HashMap<String, Any>().apply {
            put("granted", false)
        })
    }

    @Command
    fun startRecording(invoke: Invoke) {
        val args = invoke.parseArgs(StartRecordingArgs::class.java)
        
        // Check permission
        if (ContextCompat.checkSelfPermission(
                activity,
                Manifest.permission.RECORD_AUDIO
            ) != PackageManager.PERMISSION_GRANTED) {
            invoke.reject("RECORD_AUDIO permission not granted")
            return
        }

        if (isRecording.get()) {
            invoke.reject("Already recording")
            return
        }

        try {
            // Initialize AudioRecord
            audioRecord = AudioRecord(
                MediaRecorder.AudioSource.MIC,
                SAMPLE_RATE,
                CHANNEL_CONFIG,
                AUDIO_FORMAT,
                bufferSize * 2
            )

            if (audioRecord?.state != AudioRecord.STATE_INITIALIZED) {
                invoke.reject("Failed to initialize AudioRecord")
                return
            }

            audioRecord?.startRecording()
            isRecording.set(true)

            // Start recording thread
            executor.execute {
                val buffer = ShortArray(args.frameLength)
                while (isRecording.get()) {
                    val read = audioRecord?.read(buffer, 0, buffer.size) ?: 0
                    if (read > 0) {
                        // Copy buffer and add to queue
                        val data = buffer.copyOf()
                        audioBuffer.offer(data)
                        
                        // Emit event to Rust
                        mainHandler.post {
                            trigger("audio-data", gson.toJson(mapOf(
                                "data" to data.toList(),
                                "read" to read
                            )))
                        }
                    }
                }
            }

            Log.d(TAG, "Recording started")
            invoke.resolve()
        } catch (e: Exception) {
            Log.e(TAG, "Error starting recording: ${e.message}")
            invoke.reject("Error starting recording: ${e.message}")
        }
    }

    @Command
    fun stopRecording(invoke: Invoke) {
        if (!isRecording.get()) {
            invoke.resolve()
            return
        }

        isRecording.set(false)
        
        try {
            audioRecord?.stop()
            audioRecord?.release()
            audioRecord = null
            audioBuffer.clear()
            
            Log.d(TAG, "Recording stopped")
            invoke.resolve()
        } catch (e: Exception) {
            Log.e(TAG, "Error stopping recording: ${e.message}")
            invoke.reject("Error stopping recording: ${e.message}")
        }
    }

    @Command
    fun readAudioFrame(invoke: Invoke) {
        val args = invoke.parseArgs(StartRecordingArgs::class.java)
        
        if (!isRecording.get()) {
            invoke.reject("Not recording")
            return
        }

        // Try to get audio data from buffer with timeout
        val data = audioBuffer.poll(100, java.util.concurrent.TimeUnit.MILLISECONDS)
        
        if (data != null) {
            invoke.resolve(java.util.HashMap<String, Any>().apply {
                put("data", data.toList())
                put("read", data.size)
            })
        } else {
            // Return empty frame
            invoke.resolve(java.util.HashMap<String, Any>().apply {
                put("data", emptyList<Short>())
                put("read", 0)
            })
        }
    }

    @Command
    fun isRecording(invoke: Invoke) {
        invoke.resolve(java.util.HashMap<String, Any>().apply {
            put("recording", isRecording.get())
        })
    }

    @Command
    fun getAudioDevices(invoke: Invoke) {
        // On Android, we typically just have the built-in microphone
        // and optionally Bluetooth headsets
        val devices = listOf("Default Microphone", "Built-in Microphone")
        
        invoke.resolve(java.util.HashMap<String, Any>().apply {
            put("devices", devices)
        })
    }

    @Command
    fun getConfig(invoke: Invoke) {
        invoke.resolve(java.util.HashMap<String, Any>().apply {
            put("sampleRate", SAMPLE_RATE)
            put("bufferSize", bufferSize)
            put("frameLength", 512)
        })
    }

    override fun destroy() {
        isRecording.set(false)
        executor.shutdown()
        audioRecord?.release()
        audioRecord = null
        super.destroy()
    }
}
