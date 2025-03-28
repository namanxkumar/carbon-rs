use crate::links::{Source, Task};
use petgraph::stable_graph::StableDiGraph;

type TransformTreeNodeIndex = u32;

struct TransformTreeNode {
    transform: (f64, f64, f64),
}

struct TransformTreeEdge {
    parent: TransformTreeNodeIndex,
    child: TransformTreeNodeIndex,
}

pub struct TransformTree {
    // graph: StableDiGraph<TransformTreeNode, TransformTreeEdge, TransformTreeNodeIndex>,
}

pub struct TransformModel {}
