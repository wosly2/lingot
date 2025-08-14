use std::time;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rand::distr::Alphanumeric;
use rand::Rng;
use raylib::prelude::*;

use crate::draw;
use crate::ui::{Element, ElementTraits};
use crate::ui::helpers::*;
use super::*;



pub struct TextEdit<'a> {
    pub panel: Panel<'a>,
    pub label: Label<'a>,

    pub text: String,
    pub children: HashMap<String, Rc<RefCell<Element<'a>>>>,

    caret_loc: usize,
}

impl<'a> TextEdit<'a> {
    pub fn new(
        dim: Vector4,
        text_pad: i32,
        background_color: Color,
        text_color: Color,
        starting_text: String,
        max_chars_per_line: Option<usize>,
    ) -> TextEdit<'a> {
        TextEdit {
            panel: Panel {
                dim: dim,
                color: background_color,
                children: HashMap::new(),
            },
            label: Label {
                text: starting_text.clone(),
                pos: Vector2 { x: dim.x + text_pad as f32, y: dim.y + text_pad as f32 },
                color: text_color,
                children: HashMap::new(),
                max_chars_per_line,
            },
            children: HashMap::new(),
            caret_loc: starting_text.clone().len(),
            text: starting_text,
        }
    }
}

impl<'a> ElementTraits<'a> for TextEdit<'a> {
    fn render(&self, parent_x: i32, parent_y: i32, context: &mut draw::Context, draw_handle: &mut RaylibDrawHandle) {
        self.panel.render(parent_x, parent_y, context, draw_handle);
        self.label.render(parent_x, parent_y, context, draw_handle);
    }

    fn get_child(&self, id: &str) -> Option<Rc<RefCell<Element<'a>>>> {
        get_child(&self.children, id)
    }

    fn add_child(&mut self, child: Element<'a>, id: &str) {
        self.children.insert(id.to_string(), Rc::new(RefCell::new(child)));
    }

    fn update(&mut self, draw_handle: &mut RaylibHandle) {
        fn gbidx(string: &String, idx: usize) -> usize {
            string.char_indices().nth(idx).map(|(i, _)| i).unwrap_or(string.len())
        }

        const SELECTED: bool = true;

        // update keys
        let mut typed_chars = String::new();
        let old_caret_loc = gbidx(&self.text, self.caret_loc);
        while let Some(c) = draw_handle.get_char_pressed() {
            typed_chars.push(c);
            self.caret_loc += 1;
        }
        self.text.insert_str(old_caret_loc, &typed_chars);

        // handle keys FIXME!! should be iskeypressedrepeat but that quit for no reason ??
        if draw_handle.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            //println!("backspace at {}", self.caret_loc);
            if self.text.len() != 0 {
                self.text.remove(gbidx(&self.text, self.caret_loc-1));
                if self.caret_loc != 0 { self.caret_loc -= 1; };
            }
        }
        if draw_handle.is_key_pressed(KeyboardKey::KEY_LEFT) && self.caret_loc != 0 {
            self.caret_loc -= 1; 
            //println!("l now {}", self.caret_loc);
        };
        if draw_handle.is_key_pressed(KeyboardKey::KEY_RIGHT) && self.caret_loc < self.text.len() { 
            self.caret_loc += 1;
            //println!("r now {}", self.caret_loc);
        };

        // clone to the label
        self.label.text = self.text.clone();

        // get the byte loc to draw the cursor
        let byte_index = gbidx(&self.text, self.caret_loc);

        self.label.text.insert_str(byte_index, "|");

        for child in &self.children {
            child.1.borrow_mut().update(draw_handle);
        }
    }
}
