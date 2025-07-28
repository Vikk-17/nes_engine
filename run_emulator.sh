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
    echo "Usage: $0 [ROM_FILE] [OPTIONS]"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run with default ROM (pacman.nes)"
    echo "  $0 snake.nes         # Run with snake.nes"
    echo "  $0 Super.nes         # Run with Super.nes"
    echo "  $0 --list            # List available ROMs"
    echo "  $0 --help            # Show this help"
    echo "  $0 --interactive     # Interactive ROM selection"
    echo ""
    echo "Options:"
    echo "  --no-audio           # Disable audio output"
    echo "  --interactive        # Interactive ROM selection dialog"
    echo "  --list               # List available ROM files"
    echo "  --help               # Show this help message"
    echo ""
    echo "Controls:"
    echo "  Arrow Keys: D-pad"
    echo "  A/S: A/B buttons"
    echo "  Space: Select"
    echo "  Enter: Start"
    echo "  I: Pause/Resume"
    echo "  Escape: Quit"
    echo ""
    echo "Audio Features:"
    echo "  - Full NES APU emulation with 5 sound channels"
    echo "  - Pulse, Triangle, Noise, and DMC channels"
    echo "  - Real-time audio processing"
}

# Function for interactive ROM selection
interactive_selection() {
    local roms=()
    local rom_names=("pacman.nes" "snake.nes" "Super.nes")
    
    echo ""
    echo "=== NES ROM Selection ==="
    echo "Available ROM files:"
    
    local count=0
    for rom in "${rom_names[@]}"; do
        if [ -f "$rom" ]; then
            count=$((count + 1))
            roms+=("$rom")
            echo "  $count: $rom"
        fi
    done
    
    if [ $count -eq 0 ]; then
        echo "  No ROM files found in current directory!"
        return 1
    fi
    
    echo "  0: Exit"
    echo ""
    
    while true; do
        read -p "Select a ROM file (0-$count): " choice
        
        if [ "$choice" = "0" ]; then
            echo "Exiting..."
            return 1
        fi
        
        if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "$count" ]; then
            local selected_rom="${roms[$((choice - 1))]}"
            echo "Selected: $selected_rom"
            ROM_FILE="$selected_rom"
            return 0
        else
            echo "Invalid selection. Please enter a number between 0 and $count."
        fi
    done
}

# Parse command line arguments
ROM_FILE="pacman.nes"
CARGO_ARGS=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            show_usage
            exit 0
            ;;
        --list|-l)
            list_roms
            exit 0
            ;;
        --interactive|-i)
            if interactive_selection; then
                # ROM_FILE is set by interactive_selection
                shift
            else
                exit 1
            fi
            ;;
        --no-audio)
            CARGO_ARGS="$CARGO_ARGS --no-audio"
            shift
            ;;
        -*)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
        *)
            ROM_FILE="$1"
            shift
            ;;
    esac
done

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
cargo run "$ROM_FILE" $CARGO_ARGS 