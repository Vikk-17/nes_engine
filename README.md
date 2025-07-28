# NES Emulator

A NES emulator with interactive controls and features.

## Features

- **Multiple ROM Support**: Load different NES ROM files from command line
- **Interactive ROM Selection**: Browse and select ROM files with a terminal dialog
- **Pause/Resume**: Pause and resume gameplay with the 'I' key
- **Interactive Controls**: Full controller mapping with keyboard
- **Command Line Interface**: Easy ROM selection and help system
- **Audio Processing Unit (APU)**: Full NES audio emulation with all 5 sound channels

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

# Interactive ROM selection
cargo run -- --interactive

# List available ROMs
cargo run -- --list

# Show help
cargo run -- --help

# Run without audio
cargo run -- --no-audio
```

### Interactive ROM Selection
The emulator now features an interactive ROM selection dialog:

```bash
# Start interactive selection
cargo run -- --interactive

# Or use the shell script
./run_emulator.sh --interactive
```

This will display a numbered list of available ROM files and prompt you to select one:

```
=== NES ROM Selection ===
Available ROM files:
  1: pacman.nes
  2: snake.nes
  3: Super.nes
  0: Exit

Select a ROM file (0-3): 
```

### Shell Script Usage
```bash
# Interactive selection
./run_emulator.sh --interactive

# List available ROMs
./run_emulator.sh --list

# Run with specific ROM
./run_emulator.sh snake.nes

# Run without audio
./run_emulator.sh --no-audio
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
- SDL2 development libraries (with audio support)
- NES ROM files (included in the project)

## Architecture

The emulator consists of several key components:

- **CPU**: 6502 processor emulation
- **PPU**: Picture Processing Unit for graphics
- **APU**: Audio Processing Unit for sound
- **Bus**: Memory bus and system communication
- **Cartridge**: ROM loading and memory mapping
- **Joypad**: Input handling and controller emulation
- **Render**: Graphics rendering and display

## Interactive Features

### ROM Selection Dialog
The interactive ROM selection provides:
- **Numbered List**: Easy selection with number keys
- **File Validation**: Only shows existing ROM files
- **Exit Option**: Option 0 to cancel and exit
- **Error Handling**: Invalid input validation and retry
- **Clear Interface**: Clean, user-friendly terminal interface

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
- Interactive selection with `--interactive` flag

### Enhanced UI
- Dynamic window title showing loaded ROM
- Visual pause overlay
- Better error handling and user feedback
- Console status messages

## Audio Features

### APU Implementation
The emulator includes a complete NES Audio Processing Unit (APU) with all five sound channels:

1. **Pulse Channel 1** (0x4000-0x4003)
   - Square wave with 4 duty cycles (12.5%, 25%, 50%, 75%)
   - Frequency sweep capability
   - Volume control and length counter

2. **Pulse Channel 2** (0x4004-0x4007)
   - Identical to Pulse Channel 1
   - Used for stereo separation and harmony

3. **Triangle Channel** (0x4008-0x400B)
   - Triangle wave generator
   - Linear counter for note length
   - Used for bass and melody lines

4. **Noise Channel** (0x400C-0x400F)
   - White noise generator
   - 16 different noise periods
   - Used for sound effects and percussion

5. **DMC Channel** (0x4010-0x4013)
   - Delta Modulation Channel
   - 7-bit sample playback
   - Used for digital audio samples

### Audio Features
- **Frame Counter**: Synchronizes all audio channels
- **Length Counters**: Automatic note duration
- **Sweep Units**: Frequency modulation for pulse channels
- **Linear Counter**: Triangle wave control
- **Audio Mixing**: Proper channel mixing with volume balancing

### Audio Controls
- **--no-audio**: Disable audio output
- Audio automatically initializes with SDL2 audio subsystem
- Real-time audio processing during emulation

## Development

The main interactive features were added to `src/main.rs`:
- Command-line argument parsing
- Pause/resume functionality
- Enhanced event handling
- Visual pause overlay
- Better user feedback
- Interactive ROM selection dialog

The APU implementation is in `src/apu.rs`:
- Complete NES audio channel emulation
- Frame counter synchronization
- Audio mixing and processing
- SDL2 audio integration

## Technical Details

### APU Registers
- **0x4000-0x4003**: Pulse Channel 1
- **0x4004-0x4007**: Pulse Channel 2  
- **0x4008-0x400B**: Triangle Channel
- **0x400C-0x400F**: Noise Channel
- **0x4010-0x4013**: DMC Channel
- **0x4015**: Status Register
- **0x4017**: Frame Counter

### Audio Specifications
- **Sample Rate**: 44.1 kHz
- **Channels**: Mono
- **Buffer Size**: 1024 samples
- **Format**: 32-bit float

## License

This project is based on the NES emulator tutorial and includes additional interactive features and complete audio emulation. 