use std::ops::{Deref, DerefMut};

use graphics::*;
use needed_space::*;

mod iterer;
mod needed_space;
mod renderfns;
mod tree;
mod se_rendering;
pub mod graphics;

pub struct State<'g>{
    g: &'g mut Graphics,
    sdf_shader: Shader,
    font: PrettyFont,
}


#[no_mangle]
pub fn init(g: &mut Graphics) -> State{
    let font_data = FileData::load("DejaVuSansMono.ttf").unwrap();
        
    let font = g.font_from_file(&font_data);

    let shader = g.load_shader(None, "sdf.fs");
    State {
        g,
        font,
        sdf_shader: shader,
    }
}

#[no_mangle]
pub fn should_close(state: &mut State) -> bool{
    state.g.window_should_close()
}

pub struct Tree<T>{
    value: T,
    children: Vec<Tree<T>>,
}
impl <T> Tree<T>{
    fn leaf(value: T) -> Self{
        Self { value, children: vec![] }
    }
    fn new(value: T, children: Vec<Tree<T>>) -> Self{
        Self{value, children}
    }
}

pub struct Ui<'a, 'b, 'c>{
    g: &'c mut DrawHandle<'a>,
    font: PrettyFont,
    sdf_shader: &'b Shader,
    text_scale: f32,
}
impl<'a, 'b, 'c> Deref for Ui<'a, 'b, 'c> {
    type Target = DrawHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.g
    }
}
impl<'a, 'b, 'c> DerefMut for Ui<'a, 'b, 'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.g
    }
}

pub enum Expr{
    Atom(String),
    List(Vec<Expr>),
}
impl From<&Expr> for String {
    fn from(value: &Expr) -> Self {
        match value {
            Expr::Atom(s) => s.to_owned(),
            Expr::List(chidren) => {
                chidren.iter()
                       .enumerate()
                       .fold("(".to_owned(),
                             |acc, (i,v)| acc + if i != 0 {" "} else {""} + String::from(v).as_str()) + ")"
            },
        }
    }
}
macro_rules! sexpr {
    (($($i: tt)*)) => {
        Expr::List(vec![$(sexpr!($i)),*])
    };
    ($i: ident) => {
        Expr::Atom(stringify!($i).into())
    };
    ($i: expr) => {
        Expr::Atom($i.into())
    };
}


#[no_mangle]
pub fn should_reload(state: &mut State)->bool{
    state.g.is_key_pressed(Key::R)
}

#[no_mangle]
pub fn update(state: &mut State) {
    let font = state.font;
    let sdf_shader = &state.sdf_shader;
    state.g.draw_frame(|ui|{
        ui.clear_background(colors::BLACK);
        ui.draw_fps(10, 10);
        let text_scale = 14.0;

        let mut ui = Ui{
            g: ui,
            font,
            sdf_shader: &sdf_shader,
            text_scale,
        };

        let test = sexpr!(
            (defun fizbuz (zahl)
            (loop for x from "1" to zahl do
                (if ("=" "0" (mod x "15"))
                (print "fizbuz")
                (if ("=" "0" (mod x "3"))
                    (print "fiz")
                    (if ("=" "0" (mod x "5"))
                    (print "buzz")
                    (print x))))))
        );


        let expr = sexpr!(

            (define (f x)
             (if ("=" x "5")
                (display x)
                (display ("+" x "5"))))

        );
        let expr = test;

        let rfn = expr.render_fn();
        let size_tree = (rfn.layout)(&expr, &ui);

        let rect = Rect::from_min_size(vec2(50.0, 50.0), size_tree.value.size());
        (rfn.draw)(&expr, &mut ui, rect, &size_tree);
    });
}
