# JARVIS for Android 🚀

Это адаптированная версия голосового ассистента JARVIS для Android устройств.

## 📱 Особенности Android-версии

- **Полностью офлайн** — работает без интернета
- **Распознавание речи** — через Vosk (русский и английский)
- **Wake Word** — активация голосом ("Джарвис")
- **Приватность** — никаких данных не отправляются в облако
- **Минимальные требования** — Android 7.0+ (API 24)

## 🛠️ Системные требования

### Для сборки:
- **Rust** 1.70+ ([установка](https://rustup.rs/))
- **Java** JDK 17+
- **Android SDK** (через Android Studio)
- **Android NDK**
- **Node.js** 18+ (для фронтенда)

### Для запуска:
- **Android** 7.0+ (API 24)
- **Микрофон**
- **~100 MB** свободного места

## 🚀 Быстрый старт

### 1. Установка зависимостей

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Android targets для Rust
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Tauri CLI
cargo install tauri-cli --version "^2.0"

# Android SDK (через Android Studio)
# Скачай: https://developer.android.com/studio
```

### 2. Настройка окружения

Добавь в `~/.bashrc` или `~/.zshrc`:

```bash
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653  # Укажи свою версию NDK
export PATH=$PATH:$ANDROID_HOME/platform-tools
```

### 3. Сборка

```bash
# Клонируй репозиторий
git clone https://github.com/Priler/jarvis.git
cd jarvis

# Скачай Vosk модели
./scripts/download-vosk-models.sh

# Собери APK
./scripts/build-android.sh build
```

### 4. Установка на устройство

```bash
# Подключи Android устройство по USB (с включенной отладкой)
# Или запусти эмулятор

# Установи и запусти
./scripts/build-android.sh install
```

## 📋 Команды

```bash
# Сборка APK
./scripts/build-android.sh build

# Установка на устройство
./scripts/build-android.sh install

# Режим разработки (hot reload)
./scripts/build-android.sh dev

# Скачать модели
./scripts/build-android.sh models

# Очистка
./scripts/build-android.sh clean
```

## 🔧 Ручная сборка

Если скрипты не работают:

```bash
cd crates/jarvis-gui

# Инициализация Android проекта
cargo tauri android init

# Сборка debug APK
cargo tauri android build --debug

# Или development сервер
cargo tauri android dev
```

APK будет находиться в:
```
crates/jarvis-gui/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
```

## 🐛 Отладка

### Логи устройства:
```bash
adb logcat | grep -i jarvis
```

### Просмотр логов приложения:
```bash
adb shell am start -n com.jarvis.app/.MainActivity
adb logcat -s "JarvisMainActivity:D" "JarvisAudioPlugin:D"
```

### Проблемы с разрешениями:
```bash
adb shell pm grant com.jarvis.app android.permission.RECORD_AUDIO
```

## 📁 Структура проекта

```
jarvis-android/
├── crates/
│   ├── jarvis-core/          # Ядро (адаптировано для Android)
│   ├── jarvis-gui/           # Tauri GUI + Android проект
│   └── ...
├── plugins/
│   └── tauri-plugin-jarvis-audio/  # Плагин для аудио на Android
├── resources/
│   └── vosk/                 # Модели распознавания речи
└── scripts/
    ├── build-android.sh      # Скрипт сборки
    └── download-vosk-models.sh  # Скачивание моделей
```

## 🔌 Что изменено для Android

### 1. Аудио-рекордер
- ❌ `pv_recorder` — не работает на Android
- ✅ `Android AudioRecord` — через Tauri плагин

### 2. Зависимости
- `pv_recorder` — опциональный (не используется на Android)
- Добавлен `tauri-plugin-jarvis-audio` — для Android аудио

### 3. Разрешения
Добавлены в `AndroidManifest.xml`:
- `RECORD_AUDIO` — запись аудио
- `MODIFY_AUDIO_SETTINGS` — настройки аудио
- `WRITE_EXTERNAL_STORAGE` — сохранение файлов

## 🌐 Поддерживаемые языки

| Язык | Модель | Размер |
|------|--------|--------|
| Русский | vosk-model-small-ru-0.22 | ~40 MB |
| Английский | vosk-model-small-en-us-0.15 | ~40 MB |

## ⚠️ Известные ограничения

1. **TTS (Text-to-Speech)** — в текущей версии используется системный Android TTS
2. **Фоновая работа** — требуется foreground service (в разработке)
3. **Bluetooth гарнитуры** — поддержка в разработке

## 🤝 Вклад в проект

Если хочешь помочь:
1. Форкни репозиторий
2. Создай ветку: `git checkout -b feature/android-improvement`
3. Сделай коммит: `git commit -am 'Add some feature'`
4. Запушь: `git push origin feature/android-improvement`
5. Создай Pull Request

## 📄 Лицензия

GPL-3.0-only — оригинальная лицензия проекта JARVIS

## 🙏 Благодарности

- [Priler](https://github.com/Priler) — создатель оригинального JARVIS
- [Tauri](https://tauri.app/) — фреймворк для desktop/mobile приложений
- [Vosk](https://alphacephei.com/vosk/) — офлайн распознавание речи
- [Rustpotter](https://github.com/GiviMAD/rustpotter) — wake word detection

---

**Примечание:** Это неофициальная Android-адаптация. Оригинальный проект: https://github.com/Priler/jarvis
