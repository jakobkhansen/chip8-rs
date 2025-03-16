use chip8_rs::context::chip8_context::{Chip8Context, HEIGHT, SCALE, WIDTH};
use sdl2::{event::Event, keyboard::Keycode};

fn main() -> Result<(), String> {
    let mut chip8 = Chip8Context::new();

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

    chip8.frame_buffer.set_pixel(1, 1, true);
    chip8.frame_buffer.set_pixel(1, 3, true);
    chip8.frame_buffer.set_pixel(1, 4, true);
    chip8.frame_buffer.set_pixel(2, 4, true);
    chip8.frame_buffer.set_pixel(3, 4, true);
    chip8.frame_buffer.set_pixel(4, 4, true);
    chip8.frame_buffer.set_pixel(4, 3, true);
    chip8.frame_buffer.set_pixel(4, 1, true);

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
                _ => {}
            }
        }

        chip8.frame_buffer.render(&mut canvas);
    }

    Ok(())
}
