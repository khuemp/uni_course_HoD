use super::*;

// "AND" gate increases the number of basic events in a cut set. "OR" gate increases the number of cut sets.
pub(super) fn minimal_cut_sets(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    sets_to_minimal_sets(micsup(tree))
}
fn micsup(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    match tree {
        Tree::BasicEvent(event) => {
            BTreeSet::from_iter(vec![BTreeSet::from_iter(vec![event.clone()])])
        }
        Tree::IntermediateEvent(_, subtree) => micsup(subtree),
        Tree::Gate(gate) => match gate {
            Gate::Or(subtrees) => {
                let mut new_cutsets = BTreeSet::new();
                for subtree in subtrees {
                    new_cutsets.append(&mut micsup(subtree));
                }
                new_cutsets
            }
            Gate::And(subtrees) => {
                // can use Struct std::iter::Map instead
                let mut buffer = BTreeSet::new();
                for subtree in subtrees {
                    let current_subtree = micsup(subtree);
                    let last_subtree = buffer.clone();
                    buffer.clear();
                    for curr_row in &current_subtree {
                        for last_row in &last_subtree {
                            buffer.insert(BTreeSet::from_iter(
                                curr_row.union(last_row).cloned().collect::<Vec<_>>(),
                            ));
                        }
                    }
                    if last_subtree.is_empty() {
                        buffer = current_subtree.clone();
                    }
                }
                buffer
            }
        },
    }
}

pub(super) fn path_set(tree: &Tree, events: &BTreeSet<Event>) -> bool {
    minimal_path_sets(tree).contains(events)
}

// "OR" gate increases the number of basic events in a path set. "AND" gate increases the number of path sets.
pub(super) fn minimal_path_sets(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    sets_to_minimal_sets(path_sets(tree))
}
fn path_sets(tree: &Tree) -> BTreeSet<BTreeSet<Event>> {
    match tree {
        Tree::BasicEvent(event) => {
            BTreeSet::from_iter(vec![BTreeSet::from_iter(vec![event.clone()])])
        }
        Tree::IntermediateEvent(_, subtree) => path_sets(subtree),
        Tree::Gate(gate) => match gate {
            Gate::Or(subtrees) => {
                let mut buffer = BTreeSet::new();
                for subtree in subtrees {
                    let current_subtree = path_sets(subtree);
                    let last_subtree = buffer.clone();
                    buffer.clear();
                    for curr_row in &current_subtree {
                        for last_row in &last_subtree {
                            buffer.insert(BTreeSet::from_iter(
                                curr_row.union(last_row).cloned().collect::<Vec<_>>(),
                            ));
                        }
                    }
                    if last_subtree.is_empty() {
                        buffer = current_subtree.clone();
                    }
                }
                buffer
            }
            Gate::And(subtrees) => {
                let mut new_pathsets = BTreeSet::new();
                for subtree in subtrees {
                    new_pathsets.append(&mut path_sets(subtree));
                }
                new_pathsets
            }
        },
    }
}

fn sets_to_minimal_sets(mut current_set: BTreeSet<BTreeSet<Event>>) -> BTreeSet<BTreeSet<Event>> {
    let mut last_set = current_set.clone();
    loop {
        let mut drop_set = BTreeSet::new();
        for subset in &current_set {
            let s = BTreeSet::from_iter(vec![subset.clone()]);
            let others = current_set.difference(&s).cloned().collect::<Vec<_>>();
            for other in others.into_iter() {
                if subset.is_subset(&other) {
                    drop_set.insert(other);
                }
            }
        }
        current_set = current_set.difference(&drop_set).cloned().collect();
        if current_set.len() < last_set.len() {
            last_set = current_set.clone();
            continue;
        } else {
            break;
        }
    }
    current_set
}

// "OR" phải trừ đi phần bị overlapped P (A or B) = P (A ∪ B) = P(A) + P(B) - P (A ∩ B) = 1 - (1 - P(A))*(1 - P(B))
// "AND" nhân như bình thường P (A and B) = P (A ∩ B) = P(A) P(B)
pub(super) fn failure_probability(tree: &Tree) -> Ratio {
    match tree {
        Tree::BasicEvent(event) => event.1.clone(),
        Tree::IntermediateEvent(_, subtree) => failure_probability(subtree),
        Tree::Gate(gate) => match gate {
            Gate::Or(subtrees) => {
                let mut res = Ratio::new::<percent>(100.into());
                for subtree in subtrees {
                    res = res * (Ratio::new::<percent>(100.into()) - failure_probability(subtree));
                }
                Ratio::new::<percent>(100.into()) - res
            }
            Gate::And(subtrees) => {
                let mut res = Ratio::new::<percent>(100.into());
                for subtree in subtrees {
                    res = res * failure_probability(subtree);
                }
                res
            }
        },
    }
}

pub(super) fn q0(tree: &Tree, override_events: &BTreeSet<Event>) -> Ratio {
    let mut res = Ratio::new::<percent>(100.into());
    for cutset in &minimal_cut_sets(tree) {
        res = res * (Ratio::new::<percent>(100.into()) - qi(cutset, override_events));
    }
    Ratio::new::<percent>(100.into()) - res
}
fn qi(cutset: &BTreeSet<Event>, override_events: &BTreeSet<Event>) -> Ratio {
    let mut res = Ratio::new::<percent>(100.into());
    for event in cutset {
        let mut prob = event.1.clone();
        for override_event in override_events {
            if override_event.0 == event.0 {
                prob = override_event.1.clone();
                break;
            }
        }
        res = res * prob;
    }
    res
}

pub(super) fn birnbaum_importance(tree: &Tree, event: &str) -> Ratio {
    let prob_100 =
        &BTreeSet::from_iter(vec![Event(event.into(), Ratio::new::<percent>(100.into()))]);
    let prob_0 = &BTreeSet::from_iter(vec![Event(event.into(), Ratio::new::<percent>(0.into()))]);
    q0(tree, prob_100) - q0(tree, prob_0)
}

pub(super) fn improvement_potential_importance(tree: &Tree, event: &str) -> Ratio {
    let prob_0 = &BTreeSet::from_iter(vec![Event(event.into(), Ratio::new::<percent>(0.into()))]);
    let prob_normal = &BTreeSet::from_iter(vec![]);
    q0(tree, prob_normal) - q0(tree, prob_0)
}
