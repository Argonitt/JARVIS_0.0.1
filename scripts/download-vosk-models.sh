#!/bin/bash

# Script to download Vosk models for Android
# Models are optimized for mobile devices (smaller size)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESOURCES_DIR="$SCRIPT_DIR/../resources/vosk"

echo "Downloading Vosk models for Android..."

# Create directory if it doesn't exist
mkdir -p "$RESOURCES_DIR"

# Small Russian model (optimized for mobile)
RU_MODEL_URL="https://alphacephei.com/vosk/models/vosk-model-small-ru-0.22.zip"
RU_MODEL_NAME="vosk-model-small-ru-0.22"

# Small English model (optimized for mobile)
EN_MODEL_URL="https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip"
EN_MODEL_NAME="vosk-model-small-en-us-0.15"

# Function to download and extract model
download_model() {
    local url=$1
    local name=$2
    
    if [ -d "$RESOURCES_DIR/$name" ]; then
        echo "Model $name already exists, skipping..."
        return
    fi
    
    echo "Downloading $name..."
    cd "$RESOURCES_DIR"
    
    # Download
    if command -v wget &> /dev/null; then
        wget -q --show-progress "$url" -O "$name.zip"
    elif command -v curl &> /dev/null; then
        curl -L --progress-bar "$url" -o "$name.zip"
    else
        echo "Error: Neither wget nor curl is installed"
        exit 1
    fi
    
    # Extract
    echo "Extracting $name..."
    unzip -q "$name.zip"
    
    # Remove zip file
    rm "$name.zip"
    
    echo "Model $name downloaded successfully!"
}

# Download models
download_model "$RU_MODEL_URL" "$RU_MODEL_NAME"
download_model "$EN_MODEL_URL" "$EN_MODEL_NAME"

echo ""
echo "All models downloaded to: $RESOURCES_DIR"
echo ""
echo "Available models:"
ls -la "$RESOURCES_DIR"
