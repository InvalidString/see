

pub enum Tree<LeafData, NodeData>{
    Leaf(LeafData),
    Node(NodeData, Vec<Tree<LeafData, NodeData>>)
}

impl<LeafData, NodeData> Tree<LeafData, NodeData>{
    pub fn map_leafs<NewLeafData, F: Fn(LeafData)-> NewLeafData>(self, f: &F) -> Tree<NewLeafData, NodeData>{
        match self{
            Tree::Leaf(v) => Tree::Leaf(f(v)),
            Tree::Node(v, children) => Tree::Node(v, children.into_iter().map(|t|t.map_leafs(&f)).collect()),
        }
    }
}
