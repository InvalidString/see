use std::marker::PhantomData;
use super::*;



pub fn wrap<Data, L, D>(f: RenderFn<Data, L, D>, padding: f32, paint: impl Fn(&mut Ui, Rect, Rect))
-> RenderFn<Data, impl LayoutFn<Data>, impl DrawFn<Data>>
where
L: LayoutFn<Data>,
D: DrawFn<Data>,
{
    let RenderFn { layout, draw, ph: _ } = f;
    RenderFn {
        layout: move|data, ui|{
            let tree = layout(data, ui);
            Tree::new(tree.value.expand(padding), vec![tree])
        },
        draw: move|data, ui, rect, layout|{
            let inner = rect.shrink(padding);
            paint(ui, rect, inner);
            draw(
                data,
                ui,
                rect.shrink(padding),
                layout.children.get(0).expect("pad was used wrong"),
            )
        },
        ph: PhantomData,
    }
}


pub fn pad<Data, L, D>(f: RenderFn<Data, L, D>, padding: f32)
-> RenderFn<Data, impl LayoutFn<Data>, impl DrawFn<Data>>
where
L: LayoutFn<Data>,
D: DrawFn<Data>,
{
    wrap(f, padding, |_, _, _|())
}
