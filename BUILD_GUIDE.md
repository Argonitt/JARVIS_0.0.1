# 📱 JARVIS для Android — Полное руководство по сборке

## 🎯 Что было сделано

Этот проект — адаптированная версия голосового ассистента JARVIS для Android с следующими изменениями:

### ✅ Основные изменения

1. **Аудио-рекордер для Android**
   - Создан Tauri плагин `tauri-plugin-jarvis-audio`
   - Использует Android `AudioRecord` API вместо `pv_recorder`
   - Поддержка Kotlin-обёртки для нативного Android API

2. **Модульная архитектура**
   - `jarvis-core` — условная компиляция для Android (`#[cfg(target_os = "android")]`)
   - `pv_recorder` — опциональная зависимость (не используется на Android)
   - Новый модуль `recorder/android.rs` — реализация для Android

3. **Android проект**
   - `AndroidManifest.xml` с необходимыми разрешениями
   - `MainActivity.kt` — с проверкой разрешений
   - Gradle конфигурация для Tauri 2.0

4. **Скрипты сборки**
   - `build-android.sh` — полная автоматизация сборки
   - `download-vosk-models.sh` — скачивание моделей

## 📁 Структура Android-адаптации

```
jarvis-android/
├── crates/
│   ├── jarvis-core/
│   │   └── src/recorder/
│   │       ├── android.rs          # ← Новый Android рекордер
│   │       └── pvrecorder.rs       # ← Desktop (не используется на Android)
│   └── jarvis-gui/
│       ├── gen/android/            # ← Android проект
│       │   ├── app/src/main/
│       │   │   ├── AndroidManifest.xml
│       │   │   ├── java/com/jarvis/app/MainActivity.kt
│       │   │   └── res/values/
│       │   ├── build.gradle.kts
│       │   └── settings.gradle.kts
│       └── tauri.conf.json         # ← Обновлённая конфигурация
├── plugins/
│   └── tauri-plugin-jarvis-audio/  # ← Новый Tauri плагин
│       ├── src/
│       │   ├── lib.rs
│       │   └── mobile.rs
│       └── android/
│           └── src/main/java/com/jarvis/audio/AudioPlugin.kt
└── scripts/
    ├── build-android.sh
    └── download-vosk-models.sh
```

## 🚀 Быстрый старт

### Шаг 1: Установка зависимостей

```bash
# 1. Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# 3. Tauri CLI
cargo install tauri-cli --version "^2.0"

# 4. Android Studio
# Скачай с https://developer.android.com/studio
# Установи SDK и NDK через SDK Manager
```

### Шаг 2: Настройка окружения

Добавь в `~/.bashrc` или `~/.zshrc`:

```bash
# Android SDK
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653  # Укажи свою версию
export PATH=$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin

# Java (если нужно)
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk  # Путь к твоей Java
```

Примени изменения:
```bash
source ~/.bashrc  # или ~/.zshrc
```

### Шаг 3: Сборка

```bash
cd jarvis-android

# Скачай Vosk модели
./scripts/download-vosk-models.sh

# Собери APK
./scripts/build-android.sh build
```

### Шаг 4: Установка на устройство

```bash
# Подключи Android устройство с включенной отладкой по USB
# Или запусти эмулятор в Android Studio

# Установи и запусти
./scripts/build-android.sh install
```

## 🔧 Ручная сборка (если скрипты не работают)

```bash
cd crates/jarvis-gui

# Инициализация Android проекта (один раз)
cargo tauri android init

# Сборка debug APK
cargo tauri android build --debug

# Или development сервер с hot reload
cargo tauri android dev
```

APK будет здесь:
```
crates/jarvis-gui/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
```

## 📋 Команды

| Команда | Описание |
|---------|----------|
| `./scripts/build-android.sh build` | Собрать APK |
| `./scripts/build-android.sh install` | Собрать и установить |
| `./scripts/build-android.sh dev` | Режим разработки |
| `./scripts/build-android.sh clean` | Очистить сборку |
| `./scripts/build-android.sh models` | Скачать модели |

## 🐛 Отладка

### Логи Android
```bash
# Все логи JARVIS
adb logcat | grep -i jarvis

# Только MainActivity
adb logcat -s "JarvisMainActivity:D"

# Только AudioPlugin
adb logcat -s "JarvisAudioPlugin:D"
```

### Проблемы с разрешениями
```bash
# Ручная выдача разрешения
adb shell pm grant com.jarvis.app android.permission.RECORD_AUDIO

# Проверка разрешений
adb shell dumpsys package com.jarvis.app | grep permission
```

### Переустановка
```bash
# Удалить старую версию
adb uninstall com.jarvis.app

# Установить новую
adb install -r app-universal-debug.apk
```

## 🌐 Поддерживаемые языки

| Язык | Модель | Размер | Качество |
|------|--------|--------|----------|
| Русский | vosk-model-small-ru-0.22 | ~40 MB | Хорошее |
| Английский | vosk-model-small-en-us-0.15 | ~40 MB | Хорошее |

Больше моделей: https://alphacephei.com/vosk/models

## ⚠️ Известные ограничения

1. **TTS (Text-to-Speech)** — используется системный Android TTS
2. **Фоновая работа** — требуется foreground service (в разработке)
3. **Bluetooth гарнитуры** — поддержка в разработке
4. **Размер APK** ~100 MB из-за моделей Vosk

## 🔌 Технические детали

### Как работает аудио на Android

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   Rust (Core)   │────→│  Tauri Plugin    │────→│  Kotlin (JNI)   │
│                 │     │                  │     │                 │
│ recorder::read  │     │ jarvis-audio     │     │ AudioRecord API │
│   _microphone() │     │                  │     │                 │
└─────────────────┘     └──────────────────┘     └─────────────────┘
        ↑                                                   │
        └───────────────────────────────────────────────────┘
                        Audio Buffer (Vec<i16>)
```

### Условная компиляция

```rust
// crates/jarvis-core/src/recorder.rs

#[cfg(target_os = "android")]
{
    android::read_microphone(frame_buffer);
}

#[cfg(not(target_os = "android"))]
{
    pvrecorder::read_microphone(frame_buffer);
}
```

### Cargo.toml features

```toml
[features]
default = ["jarvis_app"]
jarvis_app = ["vosk", "pv_recorder", ...]  # Desktop
android = ["vosk", ...]                     # Android (без pv_recorder)
```

## 🤝 Участие в проекте

Если хочешь улучшить Android-версию:

1. Форкни репозиторий
2. Создай ветку: `git checkout -b feature/android-improvement`
3. Сделай изменения
4. Тестируй: `cargo tauri android dev`
5. Отправь PR

## 📄 Лицензия

GPL-3.0-only — оригинальная лицензия проекта JARVIS

## 🙏 Благодарности

- [Priler](https://github.com/Priler) — создатель JARVIS
- [Tauri](https://tauri.app/) — фреймворк для desktop/mobile
- [Vosk](https://alphacephei.com/vosk/) — офлайн распознавание речи

---

**Готов к сборке!** 🚀 Запускай `./scripts/build-android.sh build` и получи свой JARVIS на Android!
