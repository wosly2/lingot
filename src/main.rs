use raylib::prelude::*;

use crate::{lang::{to_object, Lang}, ui::{Element, ElementTraits, Label}};

pub mod draw;
pub mod settings;
pub mod ui;
pub mod lang;

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

    let mut screen_panel = ui::Panel::new(0, 0, rl.get_screen_width(), rl.get_screen_height(), Color::BLACK);
    screen_panel.add_child(
        ui::Element::Label(
            Label::new(
                "don't you dare press space",
                10, 10, Color::WHITE,
            )
        ), 
        "label1"
    );

    // let objects = lang::to_object("artIndef{prox} nom{young 1} verb{run past} prep{close} artDef{dist} nom{water 0} adj{run}");
    // dbg!(objects);
    let lang = match Lang::load("assets/lang") {
        Ok(l) => l,
        Err(e) => panic!("{}", e)
    };
    // println!("{}", lang.render("artIndef{prox} nom{young 1} verb{run past} prep{close} artDef{dist} nom{water 0} adj{run}"));

    let rendered_text = format!(
        "{}\n{}\n{}\n\n{}\n{}\n{}",
        "some children have ran toward that river",
        "artIndef{prox} nom{young 1} aspPerf verb{run past} prep{face} artDef{dist} nom{water 0} adj{run}",
        lang.render("artIndef{prox} nom{young 1} aspPerf verb{run past} prep{face} artDef{dist} nom{water 0} adj{run}"),
        "the cat sleeps near the fire",
        "artDef{nspac} nom{cat 0} aspProg verb{sleep pres} prep{close} artDef{nspac} nom{fire 0}",
        lang.render("artDef{nspac} nom{cat 0} aspProg verb{sleep pres} prep{close} artDef{nspac} nom{fire 0}")
    );
    println!("{}", rendered_text);

    println!("\n\n");
    dbg!(to_object("artIndef{prox} nom{young 1} aspPerf verb{run past} prep{face} artDef{dist} nom{water 0} adj{run}"));

    while !rl.window_should_close() {
        // logic ---------------------------------------------------------------------------------

        if rl.is_window_resized() {
            context.font.resize(&rl);
            screen_panel.dim.z = rl.get_screen_width() as f32;
            screen_panel.dim.w = rl.get_screen_height() as f32;
        }

        ui::with_element::<ui::Label, _>(&screen_panel.children, "label1", ui::matcher::label, |label| {
            label.text = rendered_text.as_str()
        });
        
        // drawing -------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RED); // red so i can see when things fuck up
        
        screen_panel.render(0, 0, &mut context, &mut d);
    }
}
