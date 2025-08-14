use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use crate::draw;
use crate::ui::{Element, ElementTraits, get_child};

pub struct Label<'a> {
    pub text: String,
    pub pos: Vector2,
    pub color: Color,
    pub children: HashMap<String, Rc<RefCell<Element<'a>>>>,
    pub max_chars_per_line: Option<usize>,
}

impl<'a> Label<'a> {
    pub fn new(text: String, x: i32, y: i32, color: Color, charlen: Option<usize>) -> Label<'a> {
        Label {
            text,
            pos: Vector2::new(x as f32, y as f32),
            color,
            children: HashMap::new(),
            max_chars_per_line: charlen
        }
    }
}

impl<'a> ElementTraits<'a> for Label<'a> {
    fn render(&self, parent_x: i32, parent_y: i32, context: &mut draw::Context, draw_handle: &mut RaylibDrawHandle) {
        // split into lines
        let lines = match self.max_chars_per_line {
            None => self.text.clone(),
            Some(len) => {
                if self.text.len() == 0 {"".to_string()} else {
                    let mut tail = self.text.as_str();
                    let mut total = String::new();
                    while tail.len() > len {
                        let s = tail;
                        let n = s
                            .char_indices()
                            .map(|x| x.0)
                            .take(len)
                            .last()
                            .expect("empty sequence");
                        
                        let (head, ntail) = s.split_at(n);
                        tail = ntail;
                        total.push('\n');
                        total.push_str(head);
                    };
                    total.push('\n');
                    total.push_str(tail);
                    total
                }
            },
        };

        // draw
        context.draw_text(draw_handle, &lines, self.pos.x as i32 + parent_x, self.pos.y as i32 + parent_y, self.color);
    }

    fn get_child(&self, id: &str) -> Option<Rc<RefCell<Element<'a>>>>{
        get_child(&self.children, id)
    }

    fn add_child(&mut self, child: Element<'a>, id: &str) {
        self.children.insert(id.to_string(), Rc::new(RefCell::new(child)));
    }

    fn update(&mut self, draw_handle: &mut RaylibHandle) {
        for child in &self.children {
            child.1.borrow_mut().update(draw_handle);
        }
    }
}