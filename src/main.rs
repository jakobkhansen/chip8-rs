use std::{env, fs::File};

use chip8_rs::emulator::{
    chip8_context::{HEIGHT, SCALE, WIDTH},
    emulator::{Chip8Emulator, EmulatorMode},
};
use sdl2::{event::Event, keyboard::Keycode, sys::KeyCode};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let romfile = args.get(1).expect("No ROM arg given");
    let file = File::open(romfile).expect("ROM file not found");

    println!("Running: {}", romfile);

    let mut chip8 = Chip8Emulator::new(EmulatorMode::Step);
    chip8
        .read_rom_into_memory(file)
        .expect("Could not read ROM into memory");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "CHIP-8 Emulator",
            WIDTH as u32 * SCALE,
            HEIGHT as u32 * SCALE,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if let EmulatorMode::Step = chip8.mode {
                        chip8.execute_instruction();
                    }
                }
                _ => {}
            }
        }

        if let EmulatorMode::Run = chip8.mode {
            chip8.execute_instruction();
        }

        chip8.context.frame_buffer.render(&mut canvas);
    }

    Ok(())
}
