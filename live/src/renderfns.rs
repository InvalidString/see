use std::marker::PhantomData;

use padding::*;

use crate::{Expr, Tree, Ui, needed_space::*, iterer::*};

use crate::graphics::*;

type Response = ();
type Pos2 = Vec2;

mod padding;

#[derive(Clone, Copy)]
pub struct RenderFn<Data, L, D>
where
L: LayoutFn<Data>,
D: DrawFn<Data>,
{
    pub layout: L,
    pub draw: D,
    ph: PhantomData<Data>
}
pub type PointerRenderFn<Data> =
RenderFn<
    Data,
    fn(&Expr, &Ui) -> Tree<NeededSpace>,
    fn(&Expr, &mut Ui, Rect, &Tree<NeededSpace>) -> Response
>;

// trait alias
pub trait DrawFn<Data> : Fn(Data, &mut Ui, Rect, &Tree<NeededSpace>) -> Response{}
impl <Data, T> DrawFn<Data> for T
where T: Fn(Data, &mut Ui, Rect, &Tree<NeededSpace>) -> Response{}

// trait alias
pub trait LayoutFn<Data> : Fn(Data, &Ui) -> Tree<NeededSpace>{}
impl <T, Data> LayoutFn<Data> for T
where T: Fn(Data, &Ui) -> Tree<NeededSpace>{}

impl Expr {
    fn car(&self) -> Option<&Expr>{
        match self {
            Expr::Atom(_) => None,
            Expr::List(lst) => lst.get(0),
        }
    }
    fn cdr(&self) -> Option<&[Expr]>{
        match self {
            Expr::Atom(_) => None,
            Expr::List(lst) => lst.get(1..),
        }
    }
    fn lst(&self) -> Option<&[Expr]>{
        match self {
            Expr::Atom(_) => None,
            Expr::List(lst) => lst.get(0..),
        }
    }
    fn cdr_unwrap(&self) -> &[Expr]{
        self.cdr().unwrap()
    }
    pub fn sym(&self) -> Option<&String>{
        if let Expr::Atom(sym) = self {
            Some(sym)
        }else{
            None
        }
    }

    pub fn render_fn(&self) -> PointerRenderFn<&Self>{
        match self {
            Expr::Atom(_) => SYMBOL,
            Expr::List(_) => {
                //match (||{
                //    let sym = self.car().and_then(Self::sym)?.as_str();
                //    Some(match sym {
                //        "c" => COL,
                //        "/" => FRACT,
                //        "r" => ROW,
                //        "rc" => ROW_CENTERED,
                //        "rcp" => ROW_CP,
                //        "rp" => ROW_PADDED,
                //        "cc" => COL_CENTERED,
                //        "b" => BOX,
                //        "s" => SEXPR,
                //        //"sqrt" => SQRT,
                //        _ => ERROR
                //    })
                //})(){
                //    Some(fun) => fun,
                //    None => ERROR
                //}
                SEXPR
            },
        }
    }
}

macro_rules! define_render {
    ($name: ident (
            layout($($l_ident: ident),*) $limpl: block
            draw($($d_ident: ident),*) $dimpl: block
    )) => {
        pub const $name: PointerRenderFn<&Expr> =
        RenderFn{
            layout: |$($l_ident),*|{
                $limpl
            },
            draw: |$($d_ident),*|{
                $dimpl
            },
            ph: PhantomData
        };
    };
}

macro_rules! pointerify {
    ($name: ident = $impl: expr) => {
        pub const $name: PointerRenderFn<&Expr> =
        RenderFn{
            layout: |data, ui|($impl.layout)(data, ui),
            draw: |data, ui, rect, space|($impl.draw)(data, ui, rect, space),
            ph: PhantomData
        };
    };
}

const DBG_BOXES_ENABLED: bool = false;

fn dbg_rect(ui: &mut Ui, rect: Rect){
    if DBG_BOXES_ENABLED{
        ui.draw_rect_rounded_lines(rect, 0.0, 1, 1.0, colors::GREEN);
    }
}
fn dbg_point(ui: &mut Ui, pos: Pos2){
    if DBG_BOXES_ENABLED{
        todo!()
        //ui.painter()
        //    .add(Shape::Circle(CircleShape::filled(pos, 2.0, Color32::DARK_RED)));
    }
}

impl Expr {
    pub fn needed_space(&self, ui: &Ui) -> Tree<NeededSpace>{
        let rfn = self.render_fn();
        (rfn.layout)(self, ui)
    }
    pub fn draw(&self, ui: &mut Ui, rect: Rect, space: &Tree<NeededSpace>) -> Response{
        let rfn = self.render_fn();
        (rfn.draw)(self, ui, rect, space)
    }

}

define_render!(GENERAL(
    layout(data, ui){
        data.needed_space(ui)
    }
    draw(data, ui, rect, space){
        data.draw(ui, rect, space)
    }
));

fn row<'a, Data, L, D>(f: RenderFn<&'a Data, L, D>)
-> RenderFn<&[Data], impl LayoutFn<&[Data]>, impl DrawFn<&[Data]>>
where
L: LayoutFn<&'a Data>,
D: DrawFn<&'a Data>,
{
    RenderFn {
        layout: move|data, ui|{
            let child_space: Vec<_> = data
            .iter()
            .map(|x|(f.layout)(x, ui))
            .collect();
            let total = child_space
            .iter()
            .fold(NeededSpace::ZERO,
                  |acc, x| acc.add_x(x.value));

            Tree::new(total, child_space)
        },
        draw: move|data, ui, rect, space|{
            let row_size = space.value;

            let mut cursor_top_left = rect.min;
            for (child, space) in data.iter().zip(space.children.iter()){
                let size = space.value;
                let min =
                    cursor_top_left
                    + Vec2::DOWN
                      * (row_size.size_y_above - size.size_y_above);
                let size = size.size();
                let rect = Rect::from_min_size(min, size);
                //dbg_rect(ui, rect);
                (f.draw)(child, ui, rect, space);
                cursor_top_left.x += size.x;
            }
            //ui.allocate_rect(rect, Sense::hover())
        },
        ph: PhantomData,
    }
}


fn row_centered<'a, Data, L, D>(f: RenderFn<&'a Data, L, D>)
-> RenderFn<&[Data], impl LayoutFn<&[Data]>, impl DrawFn<&[Data]>>
where
L: LayoutFn<&'a Data>,
D: DrawFn<&'a Data>,
{
    RenderFn {
        layout: move|data, ui|{
            let children_space: Vec<_> =
                data
                .iter()
                .map(|x|(f.layout)(x, ui))
                .collect();

            let space = children_space
                .iter()
                .fold(NeededSpace::ZERO,
                      |acc, x|
                        acc.center_y().add_x(x.value.center_y()));

            Tree::new(space, children_space)
        },
        draw: move|data, ui, rect, space|{

            let row_size = space.value;

            let mut cursor_top_left = rect.min;
            for (child, space) in data.iter().zip(space.children.iter()){
                let size = space.value.center_y();
                let min =
                    cursor_top_left
                    + Vec2::DOWN*(row_size.size_y_above - size.size_y_above);
                let size = size.size();
                let rect = Rect::from_min_size(min, size);
                (f.draw)(child, ui, rect, space);
                cursor_top_left.x += size.x;
            }
        },
        ph: PhantomData,
    }
}


fn col<'a, Data, L, D>(f: RenderFn<&'a Data, L, D>)
-> RenderFn<&[Data], impl LayoutFn<&[Data]>, impl DrawFn<&[Data]>>
where
L: LayoutFn<&'a Data>,
D: DrawFn<&'a Data>,
{
    RenderFn {
        layout: move|data, ui|{
            let children_space: Vec<_> =
                data
                .iter()
                .map(|x|(f.layout)(x, ui))
                .collect();

            let space =
                children_space
                .iter()
                .map(|x|x.value)
                .reduce(|acc, x| acc.stack_below(x))
                .unwrap_or(NeededSpace::ZERO);

            Tree::new(space, children_space)
        },
        draw: move|data, ui, rect, space|{
            let mut cursor_top_left = rect.min;
            for (child, space) in data.iter().zip(space.children.iter()){
                let size = space.value.size();
                let rect = Rect::from_min_size(cursor_top_left, size);
                (f.draw)(child, ui, rect, space);
                cursor_top_left.y += size.y;
            }
        },
        ph: PhantomData,
    }
}
fn col_centered<'a, Data, L, D>(f: RenderFn<&'a Data, L, D>)
-> RenderFn<&[Data], impl LayoutFn<&[Data]>, impl DrawFn<&[Data]>>
where
L: LayoutFn<&'a Data>,
D: DrawFn<&'a Data>,
{
    RenderFn {
        layout: move|data, ui|{
            let children_space: Vec<_> = data
                .iter()
                .map(|x|(f.layout)(x, ui))
                .collect();

            let space = children_space
                .iter()
                .map(|x|x.value)
                .reduce(|acc, x| acc.stack_below(x))
                .unwrap_or(NeededSpace::ZERO);

            Tree::new(space, children_space)
        },
        draw: move|data, ui, rect, space|{

            let total = rect.size();

            let mut cursor_top_left = rect.min;
            for (child, space) in data.iter().zip(space.children.iter()){
                let size = space.value.size();

                let overhang = (total.x - size.x) * 0.5;

                let rect = Rect::from_min_size(cursor_top_left + overhang * Vec2::RIGHT, size);
                (f.draw)(child, ui, rect, space);
                cursor_top_left.y += size.y;
            }
        },
        ph: PhantomData,
    }
}

fn map<A, B, L, D>(map_fn: impl Fn(B)->A + Copy, f: RenderFn<A, L, D>)
-> RenderFn<B, impl LayoutFn<B>, impl DrawFn<B>>
where
L: LayoutFn<A>,
D: DrawFn<A>,
{
    RenderFn {
        layout: move|b, ui|(f.layout)(map_fn(b), ui),
        draw: move|b, ui, rect, space|(f.draw)(map_fn(b), ui, rect, space),
        ph: PhantomData,
    }
}

fn try_map<A, B, L, D>(map_fn: impl Fn(B)->Option<A> + Copy, f: RenderFn<A, L, D>)
-> RenderFn<B, impl LayoutFn<B>, impl DrawFn<B>>
where
L: LayoutFn<A>,
D: DrawFn<A>,
B: Into<String> + Copy,
{
    RenderFn {
        layout: move|b, ui|{
            let a = map_fn(b);
            match a {
                Some(a) => (f.layout)(a, ui),
                None => {
                    let text: String = b.into();
                    let size = ui.measure(ui.font, &text, ui.text_scale);

                    Tree::leaf(NeededSpace::above(size))
                },
            }
        },
        draw: move|b, ui, rect, space|{
            let a = map_fn(b);
            match a {
                Some(a) => (f.draw)(a, ui, rect, space),
                None => {
                    let text: String = b.into();

                    ui.g.draw_text_ex(ui.font, ui.sdf_shader, &text, rect.min, ui.text_scale, colors::RED);
                },
            }
        },
        ph: PhantomData,
    }
}

const PADDING: f32 = 4.0;

pointerify!(ROW_CP = map(Expr::cdr_unwrap, row_centered(pad(GENERAL, PADDING))));
pointerify!(ROW_PADDED = map(Expr::cdr_unwrap, row(pad(GENERAL, PADDING))));
pointerify!(ROW = map(Expr::cdr_unwrap, row(GENERAL)));
pointerify!(ROW_CENTERED = map(Expr::cdr_unwrap, row_centered(GENERAL)));
pointerify!(COL = map(Expr::cdr_unwrap, col(GENERAL)));
pointerify!(COL_CENTERED = map(Expr::cdr_unwrap, col_centered(GENERAL)));

const FRACT_OVERHANG: f32 = 10.0;

/*
pointerify!(FRACT = map(
        Expr::cdr_unwrap,
        col_centered(wrap(GENERAL, 0.0, |ui, rect, _|{

        }))));
        */

define_render!(FRACT(
    layout(data, ui){
        let children = data.cdr().unwrap();

        let children_space: Vec<_> = children
            .iter()
            .map(|x|x.needed_space(ui))
            .collect();

        let space = children_space
            .iter()
            .map(|x|x.value)
            .reduce(|acc, x| acc.stack_below(x))
            .unwrap_or(NeededSpace::ZERO).expand_x(FRACT_OVERHANG);

        Tree::new(space, children_space)
    }
    draw(data, ui, rect, space){

        let total = rect.size();

        //dbg_rect(ui, rect);

        let children = data.cdr().unwrap();

        let mut cursor_top_left = rect.min;
        for (i,(child, space)) in children.iter().zip(space.children.iter()).enumerate(){
            let size = space.value.size();

            let overhang = (total.x - size.x) * 0.5;

            let rect = Rect::from_min_size(cursor_top_left + overhang * Vec2::RIGHT, size);
            let p1 = rect.left_bottom() + overhang * Vec2::LEFT;
            let p2 = rect.right_bottom() + overhang * Vec2::RIGHT;
            //let rect = rect.translate(vec2((total.x - size.x)*0.5, 0.0));
            //let rect = rect.shrink2(vec2(2.0, 0.0));
            if i != children.len()-1{
                todo!()
                //ui.painter().add(Shape::line_segment([p1, p2], Stroke::new(1.0, Color32::WHITE)));
            }
            child.draw(ui, rect, space);
            cursor_top_left.y += size.y;
        }
        //ui.allocate_rect(rect, Sense::hover())
    }
));

pointerify!(BOX = try_map(
        |data:&Expr| data.cdr()?.iter().next(),
        wrap(GENERAL,
             PADDING,
             |ui, outer, _inner|{
                let rounding = 5.0;
                ui.draw_rect_rounded_lines(outer, rounding, 10, 2.0, colors::GRAY);
             })
        ));

const SQRT_SIZE: f32 = 7.0;
const SQRT_PADDING: f32 = 5.0;

//pointerify!(SQRT =
//    try_map(
//        |data:&Expr| data.cdr()?.iter().next(),
//        wrap(pad(GENERAL, SQRT_PADDING), SQRT_SIZE,
//             |ui, outer, inner|{
//                let stroke = Stroke::new(1.0, Color32::WHITE);
//                let points =
//                    vec![ outer.left_top().lerp(outer.left_bottom(), 0.6)
//                        , inner.left_bottom().lerp(pos2(outer.left(), inner.bottom()), 0.5)
//                        , inner.left_top()
//                        , inner.right_top()
//                        //, rect.right_top().lerp(rect.right_bottom(), 0.2)
//                    ];
//                ui.painter()
//                    .add(Shape::Path(PathShape::line(points, stroke)));
//             })));

const INDENT: f32 = 10.0;
const SEXPR_PADDING: f32 = 0.0;
define_render!(
    SEXPR(
        layout(data, ui){

            let pad_layout = pad(GENERAL, PADDING).layout;

            let children = data.lst().unwrap();
            let layouts: Vec<_> = children
                .into_iter()
                .map(|child|
                     pad_layout(child, ui))
                .collect();


            let (row_size, row_elem_count) = layouts
                .iter()
                .enumerate()
                .folding((NeededSpace::ZERO,0), |(acc,_), (i, size)|{
                    (acc.add_x(size.value),i+1)})
                .take_while(|(size,_)| size.size_x < 200.0)
                .last()
                .unwrap_or_default();

            let column_size = layouts
                .iter()
                .skip(row_elem_count)
                .fold(NeededSpace::ZERO, |acc, size|{
                acc.stack_below(size.value)
            });
            let total_size = row_size
                .stack_below(column_size.expand_x(INDENT))
                .expand(SEXPR_PADDING);

            Tree::new(total_size, layouts)
        }
        draw(data, ui, rect, space){

            let pad_draw = pad(GENERAL, PADDING).draw;

            let children = data.lst().unwrap();

            let layouts = &space.children;
            let rect = rect.shrink(SEXPR_PADDING);

            let rounding = 10.0;
            //let stroke = Stroke::new(2.0, Color32::from_gray(80));

            let (row_size, row_elem_count) = layouts
                .iter()
                .enumerate()
                .folding((NeededSpace::ZERO,0), |(acc,_), (i, size)|{
                    (acc.add_x(size.value),i+1)
            })
            .take_while(|(size,_)| size.size_x < 200.0)
            .last()
            .unwrap_or_default();

            let mut cursor_top_left = rect.min;
            let mut layout_iter = layouts.into_iter().zip(children.iter());
            for (space, child) in layout_iter.by_ref().take(row_elem_count){
                let min =
                    cursor_top_left
                    + Vec2::DOWN
                      * (row_size.size_y_above - space.value.size_y_above);
                let size = space.value.size();
                let rect = Rect::from_min_size(min, size);
                cursor_top_left.x += size.x;
                pad_draw(child, ui, rect, &space);
            }
            let mut cursor_top_left = rect.min;
            cursor_top_left.y += row_size.size().y;
            cursor_top_left.x += INDENT;
            for (space, child) in layout_iter{
                let size = space.value.size();
                let rect = Rect::from_min_size(cursor_top_left, size);
                pad_draw(child, ui, rect, &space);
                cursor_top_left.y += size.y;
            }


            ui.draw_rect_rounded_lines(rect, rounding / rect.size().smaller_comp(), 10, 0.5, colors::WHITE);
            //ui.painter()
            //    .add(Shape::Rect(RectShape::stroke(rect, rounding, stroke)));

            //ui.allocate_rect(rect, Sense::hover())
        }
    )
);



define_render!(
    SYMBOL(
        layout(data, ui){
            let size = ui.measure(ui.font, &String::from(data), ui.text_scale);
            Tree::leaf(NeededSpace::above(size))
        }
        draw(data, ui, rect, _space){
            //dbg_rect(ui, rect);
            let scale = ui.text_scale;
            let shader = ui.sdf_shader.clone();
            let font = ui.font;
            ui.draw_text_ex(font, shader, &String::from(data), rect.min, scale, colors::WHITE);
        }
    )
);
pointerify!(ERROR = try_map(|_|None, GENERAL));
