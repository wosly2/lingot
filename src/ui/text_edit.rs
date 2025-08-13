use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use crate::draw;
use crate::ui::{Element, ElementTraits};
use crate::ui::helpers::*;
use super::*;



pub struct TextEdit<'a> {
    panel: Panel<'a>,
    label: Label<'a>,

    pub text: &'a str,
    pub children: HashMap<String, Rc<RefCell<Element<'a>>>>
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
}

impl<'a> TextEdit<'a> {
    pub fn new() -> TextEdit<'a> {
        return todo!()
    }

    pub fn update(&mut self) {
        self.label.text = self.text;
    }
}