use crate::graphics::Vec2;

fn vec2(x:f32, y:f32)->Vec2{
    Vec2 { x, y }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NeededSpace{
    pub size_x: f32,
    pub size_y_below: f32,
    pub size_y_above: f32
}

impl NeededSpace {
    pub const ZERO: NeededSpace = Self{ size_x: 0.0, size_y_below: 0.0, size_y_above: 0.0 };
    pub fn above(size: Vec2) -> Self{
        Self{
            size_x: size.x,
            size_y_below: 0.0,
            size_y_above: size.y,
        }
    }
    pub fn expand(self, amnt: f32) -> Self{
        Self{
            size_x: self.size_x + amnt * 2.0,
            size_y_below: self.size_y_below + amnt,
            size_y_above: self.size_y_above + amnt,
        }
    }
    pub fn expand_x(self, x: f32) -> Self{
        Self{
            size_x: self.size_x + x,
            ..self
        }
    }
    pub fn new(size_x: f32, size_y_above: f32, size_y_below: f32) -> Self{
        Self { size_x, size_y_below, size_y_above }
    }
    pub fn add_x(self, other: Self) -> Self{
        Self {
            size_x: self.size_x + other.size_x,
            size_y_below: self.size_y_below.max(other.size_y_below),
            size_y_above: self.size_y_above.max(other.size_y_above)
        }
    }
    pub fn size(&self) -> Vec2{
        vec2(self.size_x, self.size_y_below + self.size_y_above)
    }
    pub fn stack_below(self, other: Self) -> Self{
        Self{
            size_x: self.size_x.max(other.size_x),
            size_y_below: self.size_y_below + other.size().y,
            size_y_above: self.size_y_above,
        }
    }
    pub fn center_y(self) -> Self{
        let y = (self.size_y_below + self.size_y_above) / 2.0;
        Self{
            size_x: self.size_x,
            size_y_below: y,
            size_y_above: y,
        }
    }
}

