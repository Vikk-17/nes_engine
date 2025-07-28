pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod joypad;
pub mod opcodes;
pub mod ppu;
pub mod render;
pub mod trace;
pub mod apu;

use bus::Bus;
use cartridge::Rom;
use cpu::CPU;
use ppu::NesPPU;
use render::frame::Frame;
use clap::Parser;
// use trace::trace;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashMap;
use std::path::Path;
use std::io::{self, Write};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

#[derive(Parser)]
#[command(name = "NES Emulator")]
#[command(about = "A NES emulator with interactive controls")]
#[command(version)]
struct Args {
    /// ROM file to load
    #[arg(default_value = "pacman.nes")]
    rom_file: String,
    
    /// List available ROM files
    #[arg(short, long)]
    list: bool,
    
    /// Disable audio
    #[arg(short, long)]
    no_audio: bool,
    
    /// Interactive ROM selection
    #[arg(short, long)]
    interactive: bool,
}

fn list_available_roms() -> Vec<String> {
    let rom_files = ["pacman.nes", "snake.nes", "Super.nes"];
    let mut available_roms = Vec::new();
    
    for file in &rom_files {
        if Path::new(file).exists() {
            available_roms.push(file.to_string());
        }
    }
    
    available_roms
}

fn print_available_roms() {
    println!("Available ROM files:");
    let roms = list_available_roms();
    
    if roms.is_empty() {
        println!("  No ROM files found in current directory");
        return;
    }
    
    for (i, rom) in roms.iter().enumerate() {
        println!("  {}: {}", i + 1, rom);
    }
}

fn interactive_rom_selection() -> Option<String> {
    let roms = list_available_roms();
    
    if roms.is_empty() {
        println!("No ROM files found in current directory!");
        return None;
    }
    
    println!("\n=== NES ROM Selection ===");
    println!("Available ROM files:");
    
    for (i, rom) in roms.iter().enumerate() {
        println!("  {}: {}", i + 1, rom);
    }
    
    println!("  0: Exit");
    println!();
    
    loop {
        print!("Select a ROM file (0-{}): ", roms.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        
        if input == "0" {
            println!("Exiting...");
            return None;
        }
        
        match input.parse::<usize>() {
            Ok(choice) if choice >= 1 && choice <= roms.len() => {
                let selected_rom = roms[choice - 1].clone();
                println!("Selected: {}", selected_rom);
                return Some(selected_rom);
            }
            _ => {
                println!("Invalid selection. Please enter a number between 0 and {}.", roms.len());
            }
        }
    }
}

fn print_usage() {
    println!("NES Emulator Usage:");
    println!("  cargo run [ROM_FILE]");
    println!("  cargo run pacman.nes");
    println!("  cargo run snake.nes");
    println!("  cargo run Super.nes");
    println!();
    println!("Controls:");
    println!("  Arrow Keys: D-pad");
    println!("  A/S: A/B buttons");
    println!("  Space: Select");
    println!("  Enter: Start");
    println!("  I: Pause/Resume");
    println!("  Escape: Quit");
}

fn main() {
    let args = Args::parse();
    
    if args.list {
        print_available_roms();
        return;
    }
    
    let rom_file = if args.interactive {
        match interactive_rom_selection() {
            Some(rom) => rom,
            None => {
                println!("No ROM selected. Exiting.");
                return;
            }
        }
    } else {
        args.rom_file
    };
    
    // Check if ROM file exists
    if !Path::new(&rom_file).exists() {
        eprintln!("Error: ROM file '{}' not found!", rom_file);
        eprintln!();
        print_available_roms();
        std::process::exit(1);
    }
    
    println!("Loading ROM: {}", rom_file);

    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(&format!("NES Emulator - {}", rom_file), (256.0 * 4.0) as u32, (240.0 * 2.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256 * 2, 240)
        .unwrap();

    //load the game
    let bytes: Vec<u8> = std::fs::read(&rom_file).unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let mut frame = Frame::new();
    let mut paused = false;

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, joypad::JoypadButton::DOWN);
    key_map.insert(Keycode::Up, joypad::JoypadButton::UP);
    key_map.insert(Keycode::Right, joypad::JoypadButton::RIGHT);
    key_map.insert(Keycode::Left, joypad::JoypadButton::LEFT);
    key_map.insert(Keycode::Space, joypad::JoypadButton::SELECT);
    key_map.insert(Keycode::Return, joypad::JoypadButton::START);
    key_map.insert(Keycode::A, joypad::JoypadButton::BUTTON_A);
    key_map.insert(Keycode::S, joypad::JoypadButton::BUTTON_B);

    // run the game cycle
    let mut bus = Bus::new(rom, move |ppu: &NesPPU, joypad: &mut joypad::Joypad| {
        if !paused {
            render::render(ppu, &mut frame);
            texture.update(None, &frame.data, 256 *2 * 3).unwrap();
        }

        canvas.copy(&texture, None, None).unwrap();
        
        // Draw pause overlay if paused
        if paused {
            // Create a semi-transparent overlay
            let overlay_rect = sdl2::rect::Rect::new(0, 0, 256 * 2, 240);
            canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 128));
            canvas.fill_rect(overlay_rect).unwrap();
            
            // Draw a simple "PAUSED" indicator using rectangles
            // P
            canvas.set_draw_color(sdl2::pixels::Color::WHITE);
            canvas.fill_rect(sdl2::rect::Rect::new(80, 100, 8, 20)).unwrap(); // vertical line
            canvas.fill_rect(sdl2::rect::Rect::new(80, 100, 12, 8)).unwrap();  // top horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(80, 108, 12, 8)).unwrap();  // middle horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(88, 108, 8, 12)).unwrap();  // right vertical
            
            // A
            canvas.fill_rect(sdl2::rect::Rect::new(100, 100, 8, 20)).unwrap(); // left vertical
            canvas.fill_rect(sdl2::rect::Rect::new(100, 100, 12, 8)).unwrap();  // top horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(100, 108, 12, 8)).unwrap();  // middle horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(108, 100, 8, 20)).unwrap(); // right vertical
            
            // U
            canvas.fill_rect(sdl2::rect::Rect::new(120, 100, 8, 20)).unwrap(); // left vertical
            canvas.fill_rect(sdl2::rect::Rect::new(120, 116, 12, 4)).unwrap();  // bottom horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(128, 100, 8, 20)).unwrap(); // right vertical
            
            // S
            canvas.fill_rect(sdl2::rect::Rect::new(140, 100, 12, 8)).unwrap();  // top horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(140, 100, 8, 12)).unwrap();  // left vertical
            canvas.fill_rect(sdl2::rect::Rect::new(140, 108, 12, 8)).unwrap();  // middle horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(148, 108, 8, 12)).unwrap();  // right vertical
            canvas.fill_rect(sdl2::rect::Rect::new(140, 116, 12, 4)).unwrap();  // bottom horizontal
            
            // E
            canvas.fill_rect(sdl2::rect::Rect::new(160, 100, 8, 20)).unwrap(); // left vertical
            canvas.fill_rect(sdl2::rect::Rect::new(160, 100, 12, 8)).unwrap();  // top horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(160, 108, 12, 8)).unwrap();  // middle horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(160, 116, 12, 4)).unwrap();  // bottom horizontal
            
            // D
            canvas.fill_rect(sdl2::rect::Rect::new(180, 100, 8, 20)).unwrap(); // left vertical
            canvas.fill_rect(sdl2::rect::Rect::new(180, 100, 12, 8)).unwrap();  // top horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(180, 116, 12, 4)).unwrap();  // bottom horizontal
            canvas.fill_rect(sdl2::rect::Rect::new(188, 100, 8, 20)).unwrap(); // right vertical
        }
        
        canvas.present();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        match keycode {
                            Keycode::I => {
                                paused = !paused;
                                println!("Game {}!", if paused { "PAUSED" } else { "RESUMED" });
                            }
                            _ => {
                                if let Some(key) = key_map.get(&keycode) {
                                    joypad.set_button_pressed_status(*key, true);
                                }
                            }
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, false);
                    }
                }

                _ => { /* do nothing */ }
            }
        }
    });

    // Initialize audio if not disabled
    if !args.no_audio {
        match bus.init_audio(&sdl_context) {
            Ok(_) => println!("Audio initialized successfully"),
            Err(e) => eprintln!("Failed to initialize audio: {}", e),
        }
    } else {
        println!("Audio disabled");
    }

    let mut cpu = CPU::new(bus);
    cpu.reset();
    
    println!("Game started! Press 'I' to pause/resume, 'Escape' to quit.");
    cpu.run();
    /*
    cpu.run_with_callback(|cpu| {
        println!("{}", trace(cpu));
    });
    */
}
