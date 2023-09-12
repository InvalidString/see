use crate::graphics::Rect;

use crate::tree::Tree;

struct Layout{
    rect: Rect,
    line_height: f32,
}

//fn layout(se: &Tree<String, ()>) -> Tree<&String, Layout>{
//    use Tree::*;
//    match se {
//        Leaf(text) => Leaf(text),
//        Node((), children) => {
//            let ch: Vec<_> = children.iter().map(|c|layout(c)).collect();
//
//
//            Node((), ())
//        },
//    }
//}
