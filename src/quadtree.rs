use std::{collections::BinaryHeap, convert::identity};

use crate::{creature::Creature, math::Vec2};

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
    NE,
    NW,
    SW,
    SE,
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

#[derive(Clone, PartialEq)]
enum Node {
    Internal {
        // non terminal nodes
        x_bounds: (f64, f64),
        y_bounds: (f64, f64),
        children: [NodeId; 4],
        creature: Creature,
    },
    Leaf {
        // terminal nodes that have associated data
        x_bounds: (f64, f64),
        y_bounds: (f64, f64),
        creature: Creature,
    },
    Spatial {
        // terminal nodes that do not have associated data
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

    fn creature(&self) -> Option<Creature> {
        match self {
            Node::Internal { creature, .. } => Some(creature.clone()),
            Node::Leaf { creature, .. } => Some(creature.clone()),
            Node::Spatial { .. } => None,
        }
    }

    fn children(&self) -> Option<[NodeId; 4]> {
        match self {
            Node::Internal { children, .. } => Some(children.clone()),
            _ => None,
        }
    }
}

pub fn create_quadrants(x_bounds: &(f64, f64), y_bounds: &(f64, f64)) -> [Node; 4] {
    let x_mid = (x_bounds.0 + x_bounds.1) / 2f64;
    let y_mid = (y_bounds.0 + y_bounds.1) / 2f64;
    [
        Node::Spatial {
            x_bounds: (x_mid, x_bounds.1),
            y_bounds: (y_mid, y_bounds.1),
        }, // NE
        Node::Spatial {
            x_bounds: (x_bounds.0, x_mid),
            y_bounds: (y_mid, y_bounds.1),
        }, // NW
        Node::Spatial {
            x_bounds: (x_bounds.0, x_mid),
            y_bounds: (y_bounds.0, y_mid),
        }, // SW
        Node::Spatial {
            x_bounds: (x_mid, x_bounds.1),
            y_bounds: (y_bounds.0, y_mid),
        }, // SE
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

#[derive(PartialEq)]
enum QueriedSpatialElement {
    Spatial { distance: f64, node: Node },
    Creature { distance: f64, creature: Creature },
}

impl QueriedSpatialElement {
    fn from_node(node: &Node, query: &Vec2) -> Option<Self> {
        match node {
            Node::Internal {
                x_bounds, y_bounds, ..
            } => {
                let dx = f64::max(x_bounds.0 - query.x, f64::max(0f64, query.x - x_bounds.1));
                let dy = f64::max(y_bounds.0 - query.y, f64::max(0f64, query.y - y_bounds.1));
                Some(QueriedSpatialElement::Spatial {
                    distance: dx * dx + dy * dy,
                    node: node.clone(),
                })
            }
            Node::Leaf { creature, .. } => Some(QueriedSpatialElement::Creature {
                distance: (&creature.position - query).squared_norm(),
                creature: creature.clone(),
            }),
            Node::Spatial { .. } => None,
        }
    }

    fn from_creature(creature: &Creature, query: &Vec2) -> Self {
        QueriedSpatialElement::Creature {
            distance: (&creature.position - query).squared_norm(),
            creature: creature.clone(),
        }
    }

    fn distance(&self) -> f64 {
        match self {
            QueriedSpatialElement::Spatial { distance, .. } => *distance,
            QueriedSpatialElement::Creature { distance, .. } => *distance,
        }
    }
}

impl Eq for QueriedSpatialElement {}

impl PartialOrd for QueriedSpatialElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance().partial_cmp(&self.distance())
    }
}

impl Ord for QueriedSpatialElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance().total_cmp(&self.distance())
    }
}

struct CreatureQuadtree {
    nodes: Vec<Node>,
    width: f64,
    height: f64,
}

impl CreatureQuadtree {
    pub fn new(width: f64, height: f64) -> Self {
        CreatureQuadtree {
            nodes: vec![],
            width,
            height,
        }
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
            Node::Internal {
                x_bounds,
                y_bounds,
                children,
                ..
            } => match get_quadrant(x_bounds, y_bounds, &creature.position) {
                NodeQuadrant::NE => self.insert_at(
                    creature,
                    children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::NE)],
                ),
                NodeQuadrant::NW => self.insert_at(
                    creature,
                    children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::NW)],
                ),
                NodeQuadrant::SW => self.insert_at(
                    creature,
                    children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::SW)],
                ),
                NodeQuadrant::SE => self.insert_at(
                    creature,
                    children[<NodeQuadrant as Into<usize>>::into(NodeQuadrant::SE)],
                ),
            },
            Node::Spatial { x_bounds, y_bounds } => {
                // replace spatial node with a leaf
                let new_leaf = Node::Leaf {
                    x_bounds: *x_bounds,
                    y_bounds: *y_bounds,
                    creature,
                };

                let _ = std::mem::replace(&mut self.nodes[parent_id], new_leaf);
            }
            Node::Leaf {
                x_bounds,
                y_bounds,
                creature: parent_creature,
            } => {
                // create the children of what will become the parent node
                let mut children = create_quadrants(x_bounds, y_bounds);
                let replace_idx: usize =
                    get_quadrant(x_bounds, y_bounds, &creature.position).into();
                let leaf = Node::Leaf {
                    x_bounds: children[replace_idx].x_bounds(),
                    y_bounds: children[replace_idx].y_bounds(),
                    creature,
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
                let _ = std::mem::replace(&mut self.nodes[parent_id], new_parent);

                // add in new children
                for child in children {
                    self.nodes.push(child);
                }
            }
        }
    }

    pub fn get_closest(&self, query: &Vec2) -> Option<Creature> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(QueriedSpatialElement::from_node(&self.nodes[0], query).unwrap());

        while let Some(ele) = priority_queue.pop() {
            match ele {
                QueriedSpatialElement::Spatial { node, .. }
                    if matches!(node, Node::Internal { .. }) =>
                {
                    priority_queue.push(QueriedSpatialElement::from_creature(
                        &node.creature().unwrap(),
                        query,
                    ));
                    node.children()
                        .unwrap()
                        .iter()
                        .map(|child_id| {
                            QueriedSpatialElement::from_node(&self.nodes[*child_id], query)
                        })
                        .filter_map(identity)
                        .for_each(|e| priority_queue.push(e));
                }
                QueriedSpatialElement::Creature { creature, .. } if creature.position != *query => {
                    return Some(creature);
                }
                _ => (),
            };
        }

        None
    }
}
