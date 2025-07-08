use crate::{creature::Creature, vec::Vec2};

type NodeId = usize;

// oriented as such
//
// ↑ +y
// ┌────────┐
// │NW    NE│
// │        │
// │        │
// │SW    SE│
// └────────┘ → +x
enum NodeQuadrant {
    NE, NW, SW, SE,
}

impl Into<usize> for NodeQuadrant {
    fn into(self) -> usize {
        match self {
            NodeQuadrant::NE => 0,
            NodeQuadrant::NW => 1,
            NodeQuadrant::SW => 2,
            NodeQuadrant::SE => 3,
        }
    }
}

enum Node {
    Internal { // non terminal nodes
        x_bounds: (f64, f64),
        y_bounds: (f64, f64),
        children: [NodeId; 4],
        creature: Creature,
    },
    Leaf { // terminal nodes that have associated data
        x_bounds: (f64, f64),
        y_bounds: (f64, f64),
        creature: Creature,
    },
    Spatial { // terminal nodes that do not have associated data
        x_bounds: (f64, f64),
        y_bounds: (f64, f64),
    },
}

impl Node {
    fn x_bounds(&self) -> (f64, f64) {
        match self {
            Node::Internal { x_bounds, .. } => *x_bounds,
            Node::Leaf { x_bounds, .. } => *x_bounds,
            Node::Spatial { x_bounds, .. } => *x_bounds,
        }
    }

    fn y_bounds(&self) -> (f64, f64) {
        match self {
            Node::Internal { y_bounds, .. } => *y_bounds,
            Node::Leaf { y_bounds, .. } => *y_bounds,
            Node::Spatial { y_bounds, .. } => *y_bounds,
        }
    }
}

pub fn create_quadrants(x_bounds: &(f64, f64), y_bounds: &(f64, f64)) -> [Node; 4] {
    let x_mid = (x_bounds.0 + x_bounds.1) / 2f64;
    let y_mid = (y_bounds.0 + y_bounds.1) / 2f64;
    [
        Node::Spatial { x_bounds: (x_mid, x_bounds.1), y_bounds: (y_mid, y_bounds.1) }, // NE
        Node::Spatial { x_bounds: (x_bounds.0, x_mid), y_bounds: (y_mid, y_bounds.1) }, // NW
        Node::Spatial { x_bounds: (x_bounds.0, x_mid), y_bounds: (y_bounds.0, y_mid) }, // SW
        Node::Spatial { x_bounds: (x_mid, x_bounds.1), y_bounds: (y_bounds.0, y_mid) }, // SE
    ]
}

pub fn get_quadrant(x_bounds: &(f64, f64), y_bounds: &(f64, f64), position: &Vec2) -> NodeQuadrant {
    // does not implicitly check for bounds
    let lr = position.x < (x_bounds.0 + x_bounds.1) / 2f64;
    let ud = position.y < (y_bounds.0 + y_bounds.1) / 2f64;

    match (lr, ud) {
        (true, true) => NodeQuadrant::SW,
        (true, false) => NodeQuadrant::NW,
        (false, true) => NodeQuadrant::SE,
        (false, false) => NodeQuadrant::NE,
    }
}

struct CreatureQuadtree {
    nodes: Vec<Node>,
    width: f64,
    height: f64,
}

impl CreatureQuadtree {
    pub fn new(width: f64, height: f64) -> Self {
        CreatureQuadtree { nodes: vec![], width, height }
    }

    pub fn add(&mut self, creature: Creature) {
        if self.nodes.is_empty() {
            let node = Node::Leaf {
                x_bounds: (0f64, self.width),
                y_bounds: (0f64, self.height),
                creature,
            };
            self.nodes.push(node);
            return;
        }

        self.insert_at(creature, 0);
    }

    fn insert_at(&mut self, creature: Creature, parent_id: usize) {
        match &self.nodes.get(parent_id).unwrap() {
            Node::Internal { x_bounds, y_bounds, children, .. } => {
                match get_quadrant(x_bounds, y_bounds, &creature.position) {
                    NodeQuadrant::NE => self.insert_at(creature, children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::NE)]),
                    NodeQuadrant::NW => self.insert_at(creature, children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::NW)]),
                    NodeQuadrant::SW => self.insert_at(creature, children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::SW)]),
                    NodeQuadrant::SE => self.insert_at(creature, children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::SE)]),
                }
            },
            Node::Spatial { x_bounds, y_bounds } => {
                // replace spatial node with a leaf
                let new_leaf = Node::Leaf {
                    x_bounds: *x_bounds,
                    y_bounds: *y_bounds,
                    creature,
                };

                std::mem::replace(&mut self.nodes[parent_id], new_leaf);
            },
            Node::Leaf { x_bounds, y_bounds, creature: parent_creature } => {
                // create the children of what will become the parent node
                let mut children = create_quadrants(x_bounds, y_bounds);
                let replace_idx: usize = get_quadrant(x_bounds, y_bounds, &creature.position).into();
                let leaf = Node::Leaf {
                    x_bounds: children[replace_idx].x_bounds(),
                    y_bounds: children[replace_idx].y_bounds(),
                    creature
                };
                children[replace_idx] = leaf;

                // create and replace old leaf with an internal version of itself
                let base_idx = self.nodes.len();
                let new_parent = Node::Internal {
                    x_bounds: *x_bounds,
                    y_bounds: *y_bounds,
                    children: [
                        base_idx + <NodeQuadrant as Into<usize>>::into(NodeQuadrant::NE),
                        base_idx + <NodeQuadrant as Into<usize>>::into(NodeQuadrant::NW),
                        base_idx + <NodeQuadrant as Into<usize>>::into(NodeQuadrant::SW),
                        base_idx + <NodeQuadrant as Into<usize>>::into(NodeQuadrant::SE),
                    ],
                    creature: parent_creature.clone(),
                };
                std::mem::replace(&mut self.nodes[parent_id], new_parent);

                // add in new children
                for child in children {
                    self.nodes.push(child);
                }
            },
        }
    }

    pub fn get_closest(&self, creature: &Creature) {
        todo!();
    }
}
