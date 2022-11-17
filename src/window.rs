use sdl2;
use sdl2::ttf;
use sdl2::ttf::Font;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::mouse::MouseButton;

use super::tac::*;
use super::ai::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

struct PopUp {
    text: String,
    visible: bool,
}

impl PopUp {
    fn new() -> Self {
        Self { text: "".to_string(), visible: false }
    }

    fn show(&mut self, text: String) {
        self.text = text;
        self.visible = true;
    }

    fn hide(&mut self) {
        self.text = "".to_string();
        self.visible = false;
    }
}

fn draw_x(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, size: i32) -> Result<(), String> {
    canvas.draw_line((center_x - size / 2, center_y - size / 2), (center_x + size / 2, center_y + size / 2))?;
    canvas.draw_line((center_x - size / 2, center_y + size / 2), (center_x + size / 2, center_y - size / 2))?;
    Ok(())
}

fn draw_circle(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32) -> Result<(), String> {
   let diameter: i32 = radius * 2;

   let mut x: i32 = radius - 1;
   let mut y: i32 = 0;
   let mut tx: i32 = 1;
   let mut ty: i32 = 1;
   let mut error: i32 = tx - diameter;

    while x >= y {
        canvas.draw_point((center_x + x, center_y - y))?;
        canvas.draw_point((center_x + x, center_y + y))?;
        canvas.draw_point((center_x - x, center_y - y))?;
        canvas.draw_point((center_x - x, center_y + y))?;
        canvas.draw_point((center_x + y, center_y - x))?;
        canvas.draw_point((center_x + y, center_y + x))?;
        canvas.draw_point((center_x - y, center_y - x))?;
        canvas.draw_point((center_x - y, center_y + x))?;

        if error <= 0 {
            y += 1;
            error += ty;
            ty += 2;
        }

        if error > 0 {
            x -= 1;
            tx += 2;
            error += tx - diameter;
        }
    }

    Ok(())
}

fn draw_text(canvas: &mut Canvas<Window>, font: &Font, text: &str, center_x: i32, center_y: i32) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended((255, 255, 255))
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let (width, height) = surface.size();
    let target = Rect::new(
        center_x - width as i32 / 2,
        center_y - height as i32 / 2,
        width, height
    );

    canvas.copy(&texture, None, Some(target))?;
    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Tic Tac Toe", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut tic_tac_toe = TicTacToe::new(3, 3);
    let mut ai = TicTacToeAi::new();

    let mut pop_up = PopUp::new();
    let font = ttf_context.load_font("./font/Iosevka Nerd.ttf", 76)?;
    let small_font = ttf_context.load_font("./font/Iosevka Nerd.ttf", 32)?;

    let field_width = WINDOW_WIDTH as i32 / tic_tac_toe.width() as i32;
    let field_height = WINDOW_HEIGHT as i32 / tic_tac_toe.height() as i32;

    'main: loop {
        match tic_tac_toe.check_winner() {
            State::X => pop_up.show("X win!".to_string()),
            State::O => pop_up.show("O win!".to_string()),
            _ if tic_tac_toe.is_full() => pop_up.show("Draw!".to_string()),
            _ => {}
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if !pop_up.visible {
                        let x = (x / field_width) as usize;
                        let y = (y / field_height) as usize;

                        if tic_tac_toe.is_cell_empty(x, y) {
                            tic_tac_toe.next(x, y);

                            match ai.next_move(&tic_tac_toe) {
                                Some((x, y)) => tic_tac_toe.next(x, y),
                                None => {}
                            }
                        }
                    }
                    else {
                        tic_tac_toe.reset();
                        pop_up.hide();
                    }
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.set_draw_color((255, 255, 255));

        for x in 1..tic_tac_toe.width() as i32 {
            canvas.draw_line((x * field_width, 0), (x * field_width, WINDOW_HEIGHT as i32))?;
        }

        for y in 1..tic_tac_toe.height() as i32 {
            canvas.draw_line((0, y * field_height), (WINDOW_WIDTH as i32, y * field_height))?;
        }

        for y in 0..tic_tac_toe.height() {
            for x in 0..tic_tac_toe.width() {
                let xpos = x as i32 * field_width + (field_width / 2);
                let ypos = y as i32 * field_height + (field_height / 2);
                let size = (f32::min(field_width as f32, field_height as f32) * 0.7) as i32;

                match tic_tac_toe.get_cell(x, y) {
                    State::X => draw_x(&mut canvas, xpos, ypos, size)?,
                    State::O => draw_circle(&mut canvas, xpos, ypos, size / 2)?,
                    _ => {}
                }
            }
        }

        if pop_up.visible {
            let popup_width: u32 = 725;
            let popup_height: u32 = 325;
            let popup_x = ((WINDOW_WIDTH - popup_width) / 2) as i32;
            let popup_y = ((WINDOW_HEIGHT - popup_height) / 2) as i32;

            canvas.set_draw_color((100, 100, 100));
            canvas.fill_rect(Rect::new(popup_x, popup_y, popup_width, popup_height))?;
            canvas.set_draw_color((0, 0, 0));

            draw_text(
                &mut canvas,
                &font,
                pop_up.text.as_str(),
                popup_x + popup_width as i32 / 2,
                popup_y + popup_height as i32 / 2 - 32,
            )?;

            draw_text(
                &mut canvas,
                &small_font,
                "Click anywhere to restart",
                popup_x + popup_width as i32 / 2,
                popup_y + popup_height as i32 / 2 + 40,
            )?;
        }

        canvas.set_draw_color((0, 0, 0));
        canvas.present();
    }

    Ok(())
}
