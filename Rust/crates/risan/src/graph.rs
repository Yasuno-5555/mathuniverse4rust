use std::collections::HashMap;

/// Unique identifier for a Node in the Graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Unique identifier for an Edge in the Graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub usize);

#[derive(Debug, Clone)]
pub struct Node<N> {
    pub data: N,
    pub outgoing_edges: Vec<EdgeId>,
    pub incoming_edges: Vec<EdgeId>,
}

#[derive(Debug, Clone)]
pub struct Edge<E> {
    pub data: E,
    pub from: NodeId,
    pub to: NodeId,
}

#[derive(Debug, Clone, Default)]
pub struct Graph<N, E> {
    pub nodes: Vec<Node<N>>,
    pub edges: Vec<Edge<E>>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, data: N) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Node {
            data,
            outgoing_edges: Vec::new(),
            incoming_edges: Vec::new(),
        });
        id
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) -> Result<EdgeId, String> {
        if from.0 >= self.nodes.len() || to.0 >= self.nodes.len() {
            return Err("Node index out of bounds".to_string());
        }

        let id = EdgeId(self.edges.len());
        self.edges.push(Edge { data, from, to });

        // Update connectivity
        self.nodes[from.0].outgoing_edges.push(id);
        self.nodes[to.0].incoming_edges.push(id);

        Ok(id)
    }

    pub fn get_node(&self, id: NodeId) -> Option<&N> {
        self.nodes.get(id.0).map(|n| &n.data)
    }
    
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut N> {
        self.nodes.get_mut(id.0).map(|n| &mut n.data)
    }

    pub fn get_edge(&self, id: EdgeId) -> Option<&E> {
        self.edges.get(id.0).map(|e| &e.data)
    }

    /// Iterator over neighbors (outgoing).
    pub fn neighbors(&self, id: NodeId) -> Option<impl Iterator<Item = NodeId> + '_> {
        self.nodes.get(id.0).map(|node| {
            node.outgoing_edges.iter().map(move |&eid| {
                // We assume edges are valid if they exist in the list, but for safety we could check
                // However, internal consistency should be maintained by add_edge.
                // The edge stores 'to', so we return that.
                self.edges[eid.0].to
            })
        })
    }
}
