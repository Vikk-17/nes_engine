# NES Emulator

A NES emulator with interactive controls and features.

## Features

- **Multiple ROM Support**: Load different NES ROM files from command line
- **Pause/Resume**: Pause and resume gameplay with the 'I' key
- **Interactive Controls**: Full controller mapping with keyboard
- **Command Line Interface**: Easy ROM selection and help system

## Available ROMs

The emulator comes with several ROM files:
- `pacman.nes` - Pac-Man game
- `snake.nes` - Snake game  
- `Super.nes` - Super Mario Bros

## Usage

### Basic Usage
```bash
# Run with default ROM (pacman.nes)
cargo run

# Run with specific ROM
cargo run snake.nes
cargo run Super.nes

# List available ROMs
cargo run -- --list

# Show help
cargo run -- --help
```

### Controls

**Game Controls:**
- **Arrow Keys**: D-pad (Up, Down, Left, Right)
- **A**: A button
- **S**: B button  
- **Space**: Select button
- **Enter**: Start button

**Emulator Controls:**
- **I**: Pause/Resume game
- **Escape**: Quit emulator

## Building

```bash
cargo build --release
```

## Requirements

- Rust 1.70+
- SDL2 development libraries
- NES ROM files (included in the project)

## Architecture

The emulator consists of several key components:

- **CPU**: 6502 processor emulation
- **PPU**: Picture Processing Unit for graphics
- **Bus**: Memory bus and system communication
- **Cartridge**: ROM loading and memory mapping
- **Joypad**: Input handling and controller emulation
- **Render**: Graphics rendering and display

## Interactive Features

### Pause/Resume System
The emulator now supports pausing and resuming gameplay:
- Press 'I' to toggle pause state
- When paused, a visual overlay appears with "PAUSED" text
- Game state is preserved during pause
- Console output indicates pause/resume status

### ROM Selection
- Command-line argument parsing with `clap`
- Automatic ROM file validation
- Help system with usage instructions
- List available ROMs with `--list` flag

### Enhanced UI
- Dynamic window title showing loaded ROM
- Visual pause overlay
- Better error handling and user feedback
- Console status messages

## Development

The main interactive features were added to `src/main.rs`:
- Command-line argument parsing
- Pause/resume functionality
- Enhanced event handling
- Visual pause overlay
- Better user feedback

## License

This project is based on the NES emulator tutorial and includes additional interactive features. 