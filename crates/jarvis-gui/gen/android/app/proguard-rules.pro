# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.

# Keep Tauri classes
-keep class app.tauri.** { *; }
-keep class com.jarvis.app.** { *; }
-keep class com.jarvis.audio.** { *; }

# Keep Kotlin metadata
-keep class kotlin.Metadata { *; }

# Keep serialized classes
-keepclassmembers class * {
    @com.google.gson.annotations.SerializedName <fields>;
}

# Keep JavaScript interface
-keepattributes JavascriptInterface
-keepclassmembers class * {
    @android.webkit.JavascriptInterface <methods>;
}
