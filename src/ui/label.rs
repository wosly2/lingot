use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use crate::draw;
use crate::ui::{Element, ElementTraits, get_child};

pub struct Label<'a> {
    pub text: &'a str,
    pub pos: Vector2,
    pub color: Color,
    pub children: HashMap<String, Rc<RefCell<Element<'a>>>>
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str, x: i32, y: i32, color: Color) -> Label<'a> {
        Label {
            text,
            pos: Vector2::new(x as f32, y as f32),
            color,
            children: HashMap::new(),
        }
    }
}

impl<'a> ElementTraits<'a> for Label<'a> {
    fn render(&self, parent_x: i32, parent_y: i32, context: &mut draw::Context, draw_handle: &mut RaylibDrawHandle) {
        context.draw_text(draw_handle, self.text, self.pos.x as i32 + parent_x, self.pos.y as i32 + parent_y, self.color);
    }

    fn get_child(&self, id: &str) -> Option<Rc<RefCell<Element<'a>>>>{
        get_child(&self.children, id)
    }

    fn add_child(&mut self, child: Element<'a>, id: &str) {
        self.children.insert(id.to_string(), Rc::new(RefCell::new(child)));
    }
}