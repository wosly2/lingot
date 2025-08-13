use raylib::prelude::*;

use crate::settings as set;

pub struct FontOptions<'a> {
    pub font: &'a Font,
    pub size: f32,
    pub spacing: f32,

    pub char_dim: Vector2,
    pub grid_dim: Vector2,
}

impl<'a> FontOptions<'a> {
    pub fn new<'b>(handle: &'b RaylibHandle, font: &'a Font, size: f32, spacing: f32) -> FontOptions<'a> {
        // only works on monospace fonts
        let char_size = font.measure_text("X", size, spacing);
        FontOptions {
            font: font, 
            size: size, 
            spacing: spacing,

            char_dim: char_size,
            grid_dim: {
                let x = (handle.get_screen_width() as f32 / char_size.x).floor();
                let y = (handle.get_screen_height() as f32 / char_size.y).floor();
                if set::SETTINGS.debug_print_calculated_locations {
                    println!("font resize with grid dim {} {}", x, y)
                };
                Vector2 {
                    x: x,
                    y: y
                }
            }
        }
    }

    pub fn resize<'b>(&mut self, handle: &'b RaylibHandle){
        // only works on monospace fonts
        let char_size = self.font.measure_text("X", self.size, self.spacing);
        self.char_dim = char_size;
        self.grid_dim = {
            let x = (handle.get_screen_width() as f32 / char_size.x).floor();
            let y = (handle.get_screen_height() as f32 / char_size.y).floor();
            if set::SETTINGS.debug_print_calculated_locations {
                println!("font resize with grid dim {} {}", x, y)
            };
            Vector2 {
                x: x,
                y: y
            }
        };
    }
}

pub struct Context<'a> {
    pub font: &'a mut FontOptions<'a>,
    // pub draw_handle: Option<&'a mut RaylibDrawHandle<'a>>
}

impl<'a> Context<'a> {
    pub fn new(font: &'a mut FontOptions<'a>, /*draw_handle: Option<&'a mut RaylibDrawHandle<'a>>*/) -> Context<'a> {
        Context {
            font: font,
            // draw_handle
        }
    }

    // pub fn set_draw_handle(&mut self, draw_handle: &'a mut RaylibDrawHandle<'a>) {
    //     self.draw_handle = Some(draw_handle);
    // }

    pub fn draw_grid_text(&mut self, draw_handle: &mut RaylibDrawHandle, text: &str, x: i32, y: i32, tint: Color) {
        draw_handle.draw_text_ex(
            self.font.font, 
            text, 
            Vector2::new(x as f32 * self.font.char_dim.x, y as f32 * self.font.char_dim.y),
            self.font.size, 
            self.font.spacing,
            tint,
        );
    }

    pub fn draw_text(&mut self, draw_handle: &mut RaylibDrawHandle, text: &str, x: i32, y: i32, tint: Color) {
        draw_handle.draw_text_ex(
            self.font.font, 
            text, 
            Vector2::new(x as f32, y as f32),
            self.font.size, 
            self.font.spacing,
            tint,
        );
    }
}