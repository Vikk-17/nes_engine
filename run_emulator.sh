#!/bin/bash

# NES Emulator Runner Script
# This script helps you run the NES emulator with different ROMs

echo "NES Emulator Runner"
echo "=================="

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust first."
    exit 1
fi

# Function to list available ROMs
list_roms() {
    echo "Available ROM files:"
    for rom in pacman.nes snake.nes Super.nes; do
        if [ -f "$rom" ]; then
            echo "  ✓ $rom"
        else
            echo "  ✗ $rom (missing)"
        fi
    done
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [ROM_FILE]"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run with default ROM (pacman.nes)"
    echo "  $0 snake.nes         # Run with snake.nes"
    echo "  $0 Super.nes         # Run with Super.nes"
    echo "  $0 --list            # List available ROMs"
    echo "  $0 --help            # Show this help"
    echo ""
    echo "Controls:"
    echo "  Arrow Keys: D-pad"
    echo "  A/S: A/B buttons"
    echo "  Space: Select"
    echo "  Enter: Start"
    echo "  I: Pause/Resume"
    echo "  Escape: Quit"
}

# Parse command line arguments
case "${1:-}" in
    --help|-h)
        show_usage
        exit 0
        ;;
    --list|-l)
        list_roms
        exit 0
        ;;
    "")
        # No argument provided, use default
        ROM_FILE="pacman.nes"
        ;;
    *)
        ROM_FILE="$1"
        ;;
esac

# Check if ROM file exists
if [ ! -f "$ROM_FILE" ]; then
    echo "Error: ROM file '$ROM_FILE' not found!"
    echo ""
    list_roms
    exit 1
fi

echo "Starting NES Emulator with ROM: $ROM_FILE"
echo "Press 'I' to pause/resume, 'Escape' to quit"
echo ""

# Run the emulator
cargo run "$ROM_FILE" 