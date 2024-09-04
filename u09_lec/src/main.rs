use std::collections::BTreeSet;
extern crate uom;
//------------------------------------------------- Binary Tree -----------------------------------------------------------------
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>), // With Box, we introduce a pointer with a fixed size that points to a heap-allocated value and its size.
                                // This means that the BinaryTree<T> enum only carries the size of a pointer instead of a TreeNode<T>
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn insert(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.insert(value);
                } else {
                    node.right.insert(value);
                }
            }
        }
    }

    fn width(&self) -> u32 {
        match self {
            Self::Empty => 0,
            Self::NonEmpty(t) => u32::max(1, t.left.width() + t.right.width()),
        }
    }
    fn project_preorder<'a>(&self) -> PreOrderProjection<T> {
        PreOrderProjection { stack: vec![self] }
    }
}

struct PreOrderProjection<'a, T> {
    stack: Vec<&'a BinaryTree<T>>,
}

impl<'a, T> Iterator for PreOrderProjection<'a, T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let root = self.stack.pop();
        match root {
            None => None,
            Some(t) => match t {
                BinaryTree::Empty => None,
                BinaryTree::NonEmpty(t) => {
                    if let BinaryTree::NonEmpty(_r) = &t.right {
                        self.stack.push(&t.right);
                    }
                    if let BinaryTree::NonEmpty(_l) = &t.left {
                        self.stack.push(&t.left);
                    }
                    Some(t.element)
                }
            },
        }
    }
}

//------------------------------------------------- Fault Tree -----------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tree {
    BasicEvent(Event),
    IntermediateEvent(String, Box<Tree>),
    Gate(Gate),
}

// Gates store sub-trees and the gate-function itself:
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gate {
    Or(Vec<Tree>),
    And(Vec<Tree>),
}

// Events store a name as well as a probability:
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event(String, uom::si::rational64::Ratio);

impl Tree {
    fn cut_set(&self, events: &BTreeSet<Event>) -> bool { // test if a certain set of events is a cut set
        match self {
            Tree::BasicEvent(event) => events.contains(event),
            // true khi event trong BasicEvent match với set of events đang được test
            Tree::IntermediateEvent(_, subtree) => subtree.cut_set(events),
            // đẩy xuống cho Gate và BasicEvent handle
            Tree::Gate(gate) => match gate {
                Gate::Or(subtrees) => subtrees.iter().any(|subtree| subtree.cut_set(&events)),
                Gate::And(subtrees) => subtrees.iter().all(|subtree| subtree.cut_set(&events)),
            },
            // set of events đang được test là cutset when any of subtrees true in "or" gate
            // set of events đang được test là cutset when all of subtrees true in "and" gate
        }
    }

    fn cut_sets(&self) -> BTreeSet<BTreeSet<Event>> {
        let basic_events: BTreeSet<Event> = self.basic_events();
        powerset(basic_events)
            .into_iter()
            .filter(|cutset| self.cut_set(cutset))
            .collect()
    }
    fn basic_events(&self) -> BTreeSet<Event> {
        match self {
            Tree::BasicEvent(x) => BTreeSet::from_iter(vec![x.clone()]),
            Tree::IntermediateEvent(_, tree) => tree.basic_events(),
            Tree::Gate(gate) => {
                let c = gate
                    .children()
                    .iter()
                    .flat_map(|child| child.basic_events())
                    .collect::<Vec<_>>();
                BTreeSet::from_iter::<Vec<Event>>(c)
            }
        }
    }
    pub(super) fn powerset<T: Ord + Clone>(mut set: BTreeSet<T>) -> BTreeSet<BTreeSet<T>> {
        if set.is_empty() {
            let mut powerset = BTreeSet::new();
            powerset.insert(set);
            return powerset;
        }
        let entry = set.iter().next().unwrap().clone();
        set.remove(&entry);
        let mut powerset = powerset(set);
        for mut set in powerset.clone().into_iter() {
            set.insert(entry.clone());
            powerset.insert(set);
        }
        powerset
    }

    fn naive_minimal_cut_sets(&self) -> BTreeSet<BTreeSet<Event>> { // computes the set of minimum cut sets in a naive fashion
        let mut last_set = self.cut_sets(); // [[Alarm not set, Slept too long, Train late], [Alarm not set, Slept too long], [Train late]]
        let mut current_set = self.cut_sets(); // [[Alarm not set, Slept too long, Train late], [Alarm not set, Slept too long], [Train late]]
        loop {
            let mut drop_set = BTreeSet::new();
            for subset in &current_set { // [Alarm not set, Slept too long, Train late]
                let s = BTreeSet::from_iter(vec![subset.clone()]); // [[Alarm not set], [Slept too long], [Train late]]
                let others = current_set.difference(&s).cloned().collect::<Vec<_>>(); 
                // [[Alarm not set, Slept too long, Train late], [Alarm not set, Slept too long], [Train late]] 
                // - [[Alarm not set], [Slept too long], [Train late]]
                // = [[Alarm not set, Slept too long, Train late], [Alarm not set, Slept too long], [Train late]]
                for other in others.into_iter() { 
                // 1st iter: [Alarm not set, Slept too long, Train late]
                // 2nd iter: [Alarm not set, Slept too long]
                    if subset.is_subset(&other) { 
                    // 1st iter: if [Alarm not set, Slept too long, Train late] is subset of [Alarm not set, Slept too long, Train late] <- true
                    // 2nd iter: if [Alarm not set, Slept too long, Train late] is subset of [Alarm not set, Slept too long] <- false
                        drop_set.insert(other); 
                        // 1st iter: add [Alarm not set, Slept too long, Train late] to drop_set <=> [Alarm not set, Slept too long, Train late] is not minimal cutset
                        // 2nd iter: not add [Alarm not set, Slept too long] to drop_set
                    }
                }
            }
            current_set = current_set.difference(&drop_set).cloned().collect(); // current set = current set - drop set
            if current_set.len() < last_set.len() { 
                last_set = current_set.clone();
                continue; // attempt to make this collection smaller
            } else {
                break; // as soon as we no longer succeed, we break
            }
        }
        current_set
    }
}

fn main() {
    println!("Hello, world!");
}
