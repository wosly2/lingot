use raylib::prelude::*;

use crate::{lang::{to_object, Lang}, ui::*};

pub mod draw;
pub mod settings;
pub mod ui;
pub mod lang;
pub mod debug;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .title("lingot debug")
        .build();

    let game_font = rl.load_font(&thread, "assets/font/dos2.ttf").unwrap();
    let mut game_font_options = draw::FontOptions::new(
        &rl,
        &game_font,
        20.0,
        3.0,
    );


    let mut context = draw::Context::new(&mut game_font_options);

    let mut parent_panel = Panel::new(0, 0, rl.get_screen_width(), rl.get_screen_height(), Color::WHITE);
    

    parent_panel.add_child(
        Element::TextEdit(TextEdit::new(
            Vector4 { x: 40.0, y: 30.0, z: 500.0, w: 200.0 }, 
            10, 
            Color::LIGHTGRAY, 
            Color::BLACK, 
            "artDef{nspac} nom{cat 0} aspPerf verb{run past} prep{face} artDef{dist} nom{water 0} adj{run}".to_string(),
            Some(38),
        )),
        "input_edit"
    );

    parent_panel.add_child(
        Element::Label(Label::new(
            "".to_string(),
            40, 230,
            Color::BLACK,
            Some(38),
        )),
        "output_label"
    );

    let clang = lang::Lang::load("assets/lang").unwrap();

    while !rl.window_should_close() {
        // logic ---------------------------------------------------------------------------------

        if rl.is_window_resized() {
            context.font.resize(&rl);
            parent_panel.dim.z = rl.get_screen_width() as f32;
            parent_panel.dim.w = rl.get_screen_height() as f32;
        }

        // get the string value from the input_edit
        let mut input = String::new();
        ui::with_element::<ui::TextEdit, _>(&parent_panel.children, "input_edit", ui::matcher::text_edit, |edit| {
            input = edit.text.clone();
        });

        // put the rendered version of that in the label
        ui::with_element::<ui::Label, _>(&parent_panel.children, "output_label", ui::matcher::label, |label| {
            label.text = format!("Output:\n{}", 
                match clang.render(input.clone().as_str()) {
                    Ok(s) => s,
                    Err(s) => s,
                }
            )
        });


        
        // drawing -------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        
        parent_panel.render(0, 0, &mut context, &mut d);
        parent_panel.update(&mut d);
    }
}


// syntax for child getting
// ui::with_element::<ui::Label, _>(&ELEMENT.children, "CHILD_NAME", ui::matcher::ELEMENT_MATCHING, |CHILD_ELEMENT| {
//     CHILD_ELEMENT
// });