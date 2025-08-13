use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use crate::draw;
use crate::ui::{Element, ElementTraits};
use crate::ui::helpers::*;

pub struct Panel<'a> {
    pub dim: Vector4,
    // todo: add scale?
    pub color: Color,
    pub children: HashMap<String, Rc<RefCell<Element<'a>>>>
}

impl<'a> Panel<'a> {
    pub fn new(x: i32, y: i32, w: i32, h: i32, color: Color) -> Panel<'a> {
        return Panel {
            dim: Vector4::new(x as f32, y as f32, w as f32, h as f32),
            color: color,
            children: HashMap::new(),
        }
    }
}

impl<'a> ElementTraits<'a> for Panel<'a> {
    fn render(&self, parent_x: i32, parent_y: i32, context: &mut draw::Context, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle
        (
            self.dim.x as i32 + parent_x, 
            self.dim.y as i32 + parent_y, 
            self.dim.z as i32, 
            self.dim.w as i32, 
            self.color
        );
        for child in &self.children {
            child.1.borrow().render(self.dim.x as i32 + parent_x, self.dim.y as i32 + parent_y, context, draw_handle);
        }
    }

    fn get_child(&self, id: &str) -> Option<Rc<RefCell<Element<'a>>>> {
        get_child(&self.children, id)
    }

    fn add_child(&mut self, child: Element<'a>, id: &str) {
        self.children.insert(id.to_string(), Rc::new(RefCell::new(child)));
    }
}
