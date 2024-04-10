
#[derive(PartialEq, Eq, Clone)]
enum NodeState {
    Bad,
    Good
}

#[derive(PartialEq, Eq, Clone)]
struct Node {
    state: NodeState, 

    son: Vec<Box<Node>>,
    go: Vec<Box<Node>>,

    parent: Box<Node>,
    suff_link: Box<Node>,

    up: Box<Node>,

    char_to_parent: char,
    is_leaf: bool,

    leaf_pattern_number: Vec<usize>,
}

struct AhoCorasik {
    root: Box<Node>
}

impl AhoCorasik {
    fn get_suff_link(&self, mut n: Box<Node>) {
        if n.suff_link.state == NodeState::Bad {
            if n == self.root {
                n.suff_link = self.root.clone();
            }
        } else {


        }
    }
}