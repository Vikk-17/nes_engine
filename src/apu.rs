// APU Register addresses
const APU_PULSE1_DUTY: u16 = 0x4000;
const APU_PULSE1_SWEEP: u16 = 0x4001;
const APU_PULSE1_TIMER_LOW: u16 = 0x4002;
const APU_PULSE1_TIMER_HIGH: u16 = 0x4003;
const APU_PULSE2_DUTY: u16 = 0x4004;
const APU_PULSE2_SWEEP: u16 = 0x4005;
const APU_PULSE2_TIMER_LOW: u16 = 0x4006;
const APU_PULSE2_TIMER_HIGH: u16 = 0x4007;
const APU_TRIANGLE_LINEAR: u16 = 0x4008;
const APU_TRIANGLE_TIMER_LOW: u16 = 0x400A;
const APU_TRIANGLE_TIMER_HIGH: u16 = 0x400B;
const APU_NOISE_VOLUME: u16 = 0x400C;
const APU_NOISE_PERIOD: u16 = 0x400E;
const APU_NOISE_LENGTH: u16 = 0x400F;
const APU_DMC_FREQ: u16 = 0x4010;
const APU_DMC_RAW: u16 = 0x4011;
const APU_DMC_START: u16 = 0x4012;
const APU_DMC_LENGTH: u16 = 0x4013;
const APU_STATUS: u16 = 0x4015;
const APU_FRAME_COUNTER: u16 = 0x4017;

// Sample rate and buffer size
const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: usize = 1024;

// Duty cycles for pulse waves
const DUTY_CYCLES: [[f32; 8]; 4] = [
    [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], // 12.5%
    [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], // 25%
    [0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0], // 50%
    [1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0], // 75%
];

// Triangle wave lookup table
const TRIANGLE_WAVE: [f32; 32] = [
    15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0,
    7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0,
    0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0,
    8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
];

pub struct PulseChannel {
    enabled: bool,
    duty_cycle: u8,
    duty_step: u8,
    timer: u16,
    timer_value: u16,
    length_counter: u8,
    volume: u8,
    constant_volume: bool,
    sweep_enabled: bool,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_negate: bool,
    sweep_reload: bool,
    sweep_counter: u8,
}

impl PulseChannel {
    fn new() -> Self {
        PulseChannel {
            enabled: false,
            duty_cycle: 0,
            duty_step: 0,
            timer: 0,
            timer_value: 0,
            length_counter: 0,
            volume: 0,
            constant_volume: false,
            sweep_enabled: false,
            sweep_period: 0,
            sweep_shift: 0,
            sweep_negate: false,
            sweep_reload: false,
            sweep_counter: 0,
        }
    }

    fn write_duty(&mut self, value: u8) {
        self.duty_cycle = (value >> 6) & 0x03;
        self.constant_volume = (value & 0x10) != 0;
        self.volume = value & 0x0F;
    }

    fn write_sweep(&mut self, value: u8) {
        self.sweep_enabled = (value & 0x80) != 0;
        self.sweep_period = (value >> 4) & 0x07;
        self.sweep_negate = (value & 0x08) != 0;
        self.sweep_shift = value & 0x07;
        self.sweep_reload = true;
    }

    fn write_timer_low(&mut self, value: u8) {
        self.timer = (self.timer & 0xFF00) | value as u16;
    }

    fn write_timer_high(&mut self, value: u8) {
        self.timer = (self.timer & 0x00FF) | ((value & 0x07) as u16) << 8;
        self.length_counter = if (value & 0xF8) != 0 {
            ((value >> 3) & 0x1F) + 1
        } else {
            0
        };
        self.duty_step = 0;
    }

    fn tick(&mut self) -> f32 {
        if !self.enabled || self.length_counter == 0 {
            return 0.0;
        }

        self.timer_value = self.timer_value.wrapping_sub(1);
        if self.timer_value == 0 {
            self.timer_value = self.timer;
            self.duty_step = (self.duty_step + 1) % 8;
        }

        let duty_value = DUTY_CYCLES[self.duty_cycle as usize][self.duty_step as usize];
        let volume = if self.constant_volume {
            self.volume as f32
        } else {
            self.volume as f32
        };

        duty_value * volume / 15.0
    }

    fn quarter_frame(&mut self) {
        // Length counter and sweep logic
        if self.sweep_reload {
            self.sweep_counter = self.sweep_period;
            self.sweep_reload = false;
        } else if self.sweep_counter > 0 {
            self.sweep_counter -= 1;
        } else {
            self.sweep_counter = self.sweep_period;
            if self.sweep_enabled && self.sweep_shift > 0 {
                let change = self.timer >> self.sweep_shift;
                if self.sweep_negate {
                    self.timer = self.timer.wrapping_sub(change);
                } else {
                    self.timer = self.timer.wrapping_add(change);
                }
            }
        }
    }

    fn half_frame(&mut self) {
        // Length counter decrement
        if self.length_counter > 0 && !self.constant_volume {
            self.length_counter -= 1;
        }
    }
}

pub struct TriangleChannel {
    enabled: bool,
    timer: u16,
    timer_value: u16,
    length_counter: u8,
    linear_counter: u8,
    linear_counter_reload: u8,
    linear_counter_reload_flag: bool,
    triangle_step: u8,
}

impl TriangleChannel {
    fn new() -> Self {
        TriangleChannel {
            enabled: false,
            timer: 0,
            timer_value: 0,
            length_counter: 0,
            linear_counter: 0,
            linear_counter_reload: 0,
            linear_counter_reload_flag: false,
            triangle_step: 0,
        }
    }

    fn write_linear(&mut self, value: u8) {
        self.linear_counter_reload = value & 0x7F;
        self.linear_counter_reload_flag = (value & 0x80) != 0;
    }

    fn write_timer_low(&mut self, value: u8) {
        self.timer = (self.timer & 0xFF00) | value as u16;
    }

    fn write_timer_high(&mut self, value: u8) {
        self.timer = (self.timer & 0x00FF) | ((value & 0x07) as u16) << 8;
        self.length_counter = if (value & 0xF8) != 0 {
            ((value >> 3) & 0x1F) + 1
        } else {
            0
        };
        self.linear_counter_reload_flag = true;
    }

    fn tick(&mut self) -> f32 {
        if !self.enabled || self.length_counter == 0 || self.linear_counter == 0 {
            return 0.0;
        }

        self.timer_value = self.timer_value.wrapping_sub(1);
        if self.timer_value == 0 {
            self.timer_value = self.timer;
            self.triangle_step = (self.triangle_step + 1) % 32;
        }

        TRIANGLE_WAVE[self.triangle_step as usize] / 15.0
    }

    fn quarter_frame(&mut self) {
        if self.linear_counter_reload_flag {
            self.linear_counter = self.linear_counter_reload;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }
        if !self.linear_counter_reload_flag {
            self.linear_counter_reload_flag = false;
        }
    }

    fn half_frame(&mut self) {
        if self.length_counter > 0 && !self.linear_counter_reload_flag {
            self.length_counter -= 1;
        }
    }
}

pub struct NoiseChannel {
    enabled: bool,
    timer: u16,
    timer_value: u16,
    length_counter: u8,
    volume: u8,
    constant_volume: bool,
    shift_register: u16,
    mode: bool,
}

impl NoiseChannel {
    fn new() -> Self {
        NoiseChannel {
            enabled: false,
            timer: 0,
            timer_value: 0,
            length_counter: 0,
            volume: 0,
            constant_volume: false,
            shift_register: 1,
            mode: false,
        }
    }

    fn write_volume(&mut self, value: u8) {
        self.constant_volume = (value & 0x10) != 0;
        self.volume = value & 0x0F;
    }

    fn write_period(&mut self, value: u8) {
        self.mode = (value & 0x80) != 0;
        let period_index = value & 0x0F;
        // Noise period lookup table
        let periods = [
            4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
        ];
        self.timer = periods[period_index as usize] as u16;
    }

    fn write_length(&mut self, value: u8) {
        self.length_counter = if (value & 0xF8) != 0 {
            ((value >> 3) & 0x1F) + 1
        } else {
            0
        };
    }

    fn tick(&mut self) -> f32 {
        if !self.enabled || self.length_counter == 0 {
            return 0.0;
        }

        self.timer_value = self.timer_value.wrapping_sub(1);
        if self.timer_value == 0 {
            self.timer_value = self.timer;
            
            let feedback = if self.mode {
                (self.shift_register & 0x40) != 0
            } else {
                (self.shift_register & 0x02) != 0
            } ^ ((self.shift_register & 0x01) != 0);
            
            self.shift_register >>= 1;
            if feedback {
                self.shift_register |= 0x4000;
            }
        }

        let noise_bit = (self.shift_register & 0x01) as f32;
        let volume = if self.constant_volume {
            self.volume as f32
        } else {
            self.volume as f32
        };

        noise_bit * volume / 15.0
    }

    fn half_frame(&mut self) {
        if self.length_counter > 0 && !self.constant_volume {
            self.length_counter -= 1;
        }
    }
}

pub struct DMCChannel {
    enabled: bool,
    timer: u16,
    timer_value: u16,
    sample_buffer: u8,
    sample_buffer_empty: bool,
    shift_register: u8,
    bits_remaining: u8,
    sample_address: u16,
    sample_length: u16,
    current_address: u16,
    bytes_remaining: u16,
    loop_flag: bool,
    irq_enabled: bool,
    output_level: u8,
}

impl DMCChannel {
    fn new() -> Self {
        DMCChannel {
            enabled: false,
            timer: 0,
            timer_value: 0,
            sample_buffer: 0,
            sample_buffer_empty: true,
            shift_register: 0,
            bits_remaining: 0,
            sample_address: 0,
            sample_length: 0,
            current_address: 0,
            bytes_remaining: 0,
            loop_flag: false,
            irq_enabled: false,
            output_level: 0,
        }
    }

    fn write_freq(&mut self, value: u8) {
        self.loop_flag = (value & 0x40) != 0;
        self.irq_enabled = (value & 0x80) != 0;
        let rate_index = value & 0x0F;
        // DMC rate lookup table
        let rates = [
            428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
        ];
        self.timer = rates[rate_index as usize] as u16;
    }

    fn write_raw(&mut self, value: u8) {
        self.output_level = value & 0x7F;
    }

    fn write_start(&mut self, value: u8) {
        self.sample_address = 0xC000 | ((value as u16) << 6);
    }

    fn write_length(&mut self, value: u8) {
        self.sample_length = ((value as u16) << 4) | 1;
    }

    fn tick(&mut self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        self.timer_value = self.timer_value.wrapping_sub(1);
        if self.timer_value == 0 {
            self.timer_value = self.timer;
            
            if self.bits_remaining == 0 {
                if self.sample_buffer_empty {
                    // TODO: Implement sample loading from memory
                    return 0.0;
                }
                self.shift_register = self.sample_buffer;
                self.bits_remaining = 8;
                self.sample_buffer_empty = true;
            }
            
            let bit = (self.shift_register & 0x01) != 0;
            self.shift_register >>= 1;
            self.bits_remaining -= 1;
            
            if bit && self.output_level < 126 {
                self.output_level += 2;
            } else if !bit && self.output_level > 1 {
                self.output_level -= 2;
            }
        }

        (self.output_level as f32 - 64.0) / 64.0
    }
}

pub struct APU {
    pulse1: PulseChannel,
    pulse2: PulseChannel,
    triangle: TriangleChannel,
    noise: NoiseChannel,
    dmc: DMCChannel,
    frame_counter: u16,
    frame_counter_mode: bool,
    irq_inhibit: bool,
    audio_buffer: Vec<f32>,
}

impl APU {
    pub fn new() -> Self {
        APU {
            pulse1: PulseChannel::new(),
            pulse2: PulseChannel::new(),
            triangle: TriangleChannel::new(),
            noise: NoiseChannel::new(),
            dmc: DMCChannel::new(),
            frame_counter: 0,
            frame_counter_mode: false,
            irq_inhibit: false,
            audio_buffer: Vec::new(),
        }
    }

    pub fn init_audio(&mut self, _sdl_context: &sdl2::Sdl) -> Result<(), String> {
        // For now, just return success - we'll implement actual audio later
        Ok(())
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            APU_PULSE1_DUTY => self.pulse1.write_duty(value),
            APU_PULSE1_SWEEP => self.pulse1.write_sweep(value),
            APU_PULSE1_TIMER_LOW => self.pulse1.write_timer_low(value),
            APU_PULSE1_TIMER_HIGH => self.pulse1.write_timer_high(value),
            
            APU_PULSE2_DUTY => self.pulse2.write_duty(value),
            APU_PULSE2_SWEEP => self.pulse2.write_sweep(value),
            APU_PULSE2_TIMER_LOW => self.pulse2.write_timer_low(value),
            APU_PULSE2_TIMER_HIGH => self.pulse2.write_timer_high(value),
            
            APU_TRIANGLE_LINEAR => self.triangle.write_linear(value),
            APU_TRIANGLE_TIMER_LOW => self.triangle.write_timer_low(value),
            APU_TRIANGLE_TIMER_HIGH => self.triangle.write_timer_high(value),
            
            APU_NOISE_VOLUME => self.noise.write_volume(value),
            APU_NOISE_PERIOD => self.noise.write_period(value),
            APU_NOISE_LENGTH => self.noise.write_length(value),
            
            APU_DMC_FREQ => self.dmc.write_freq(value),
            APU_DMC_RAW => self.dmc.write_raw(value),
            APU_DMC_START => self.dmc.write_start(value),
            APU_DMC_LENGTH => self.dmc.write_length(value),
            
            APU_STATUS => {
                self.pulse1.enabled = (value & 0x01) != 0;
                self.pulse2.enabled = (value & 0x02) != 0;
                self.triangle.enabled = (value & 0x04) != 0;
                self.noise.enabled = (value & 0x08) != 0;
                self.dmc.enabled = (value & 0x10) != 0;
            }
            
            APU_FRAME_COUNTER => {
                self.frame_counter_mode = (value & 0x80) != 0;
                self.irq_inhibit = (value & 0x40) != 0;
                self.frame_counter = 0;
            }
            
            _ => {}
        }
    }

    pub fn read_register(&mut self, addr: u16) -> u8 {
        match addr {
            APU_STATUS => {
                let mut status = 0;
                if self.pulse1.length_counter > 0 { status |= 0x01; }
                if self.pulse2.length_counter > 0 { status |= 0x02; }
                if self.triangle.length_counter > 0 { status |= 0x04; }
                if self.noise.length_counter > 0 { status |= 0x08; }
                if self.dmc.bytes_remaining > 0 { status |= 0x10; }
                // TODO: Add IRQ status
                status
            }
            _ => 0
        }
    }

    pub fn tick(&mut self) {
        // Tick all channels
        let pulse1_out = self.pulse1.tick();
        let pulse2_out = self.pulse2.tick();
        let triangle_out = self.triangle.tick();
        let noise_out = self.noise.tick();
        let dmc_out = self.dmc.tick();

        // Mix audio (simple mixing for now)
        let mixed = (pulse1_out + pulse2_out) * 0.3 + triangle_out * 0.5 + noise_out * 0.2 + dmc_out * 0.1;
        
        // Clamp to valid range
        let clamped = mixed.max(-1.0).min(1.0);
        
        self.audio_buffer.push(clamped);
        
        // Keep buffer size manageable
        if self.audio_buffer.len() > BUFFER_SIZE * 2 {
            self.audio_buffer.drain(0..BUFFER_SIZE);
        }

        // Frame counter logic
        self.frame_counter += 1;
        
        // Quarter frame (every 7457 CPU cycles)
        if self.frame_counter % 7457 == 0 {
            self.pulse1.quarter_frame();
            self.pulse2.quarter_frame();
            self.triangle.quarter_frame();
        }
        
        // Half frame (every 14915 CPU cycles)
        if self.frame_counter % 14915 == 0 {
            self.pulse1.half_frame();
            self.pulse2.half_frame();
            self.triangle.half_frame();
            self.noise.half_frame();
        }
        
        // Reset frame counter
        if self.frame_counter >= 29830 {
            self.frame_counter = 0;
        }
    }

    pub fn get_audio_buffer(&mut self) -> Vec<f32> {
        let buffer = self.audio_buffer.clone();
        self.audio_buffer.clear();
        buffer
    }
} 