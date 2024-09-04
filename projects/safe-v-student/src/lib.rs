#[macro_use]
extern crate prettytable;

use std::collections::BTreeSet;
use std::iter::FromIterator;

mod fault_tree;
pub mod trees;
pub mod util;

use uom::si::ratio::percent;
use uom::si::rational64::Ratio;
use util::powerset;

pub enum Style {
    Pretty,
    CSV,
}

pub fn parse_style_from_args() -> Style {
    std::env::args()
        .into_iter()
        .find(|arg| arg == "--csv")
        .map(|_| Style::CSV)
        .unwrap_or(Style::Pretty)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gate {
    Or(Vec<Tree>),
    And(Vec<Tree>),
}

impl Gate {
    fn children(&self) -> &Vec<Tree> {
        match self {
            Gate::Or(children) => children,
            Gate::And(children) => children,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event(String, uom::si::rational64::Ratio);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tree {
    BasicEvent(Event),
    IntermediateEvent(String, Box<Tree>),
    Gate(Gate),
}

impl Tree {
    pub fn cut_sets(&self) -> BTreeSet<BTreeSet<Event>> {
        // all cut sets (including those that are not minimal)
        let basic_events: BTreeSet<Event> = self.basic_events();
        powerset(basic_events)
            .into_iter()
            .filter(|cutset| self.cut_set(cutset)) // filter from power set to achieve cutset
            .collect()
    }

    pub fn cut_set(&self, events: &BTreeSet<Event>) -> bool {
        // test if a certain set of events is a cut set
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

    pub fn naive_minimal_cut_sets(&self) -> BTreeSet<BTreeSet<Event>> {
        // Computes the set of minimum cut sets in a naive fashion.
        // Removal itself works by comparing all sets with each other and if one is the subset of another set, we drop the other set (as it is not minimal).
        let mut last_set = self.cut_sets(); // [[Alarm not set, Slept too long, Train late], [Alarm not set, Slept too long], [Train late]]
        let mut current_set = self.cut_sets(); // [[Alarm not set, Slept too long, Train late], [Alarm not set, Slept too long], [Train late]]
        loop {
            let mut drop_set = BTreeSet::new();
            for subset in &current_set {
                // [Alarm not set, Slept too long, Train late]
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

    fn basic_events(&self) -> BTreeSet<Event> {
        match self {
            Tree::BasicEvent(x) => BTreeSet::from_iter(vec![x.clone()]),
            Tree::IntermediateEvent(_, tree) => tree.basic_events(), // bỏ qua intermediate event, tiến thẳng tới basic event
            Tree::Gate(gate) => {
                // collect basic event của từng children node
                let c = gate
                    .children()
                    .iter()
                    .flat_map(|child| child.basic_events())
                    .collect::<Vec<_>>();
                BTreeSet::from_iter::<Vec<Event>>(c)
            }
        }
    }

    pub fn minimal_cut_sets(&self) -> BTreeSet<BTreeSet<Event>> {
        fault_tree::minimal_cut_sets(self)
    }

    pub fn path_set(&self, events: &BTreeSet<Event>) -> bool {
        fault_tree::path_set(self, events)
    }

    pub fn minimal_path_sets(&self) -> BTreeSet<BTreeSet<Event>> {
        fault_tree::minimal_path_sets(self)
    }

    pub fn failure_probability(&self) -> Ratio {
        fault_tree::failure_probability(self)
    }

    pub fn q0(&self, events: &BTreeSet<Event>) -> Ratio {
        fault_tree::q0(self, events)
    }

    pub fn birnbaum_importance(&self, event: &str) -> Ratio {
        fault_tree::birnbaum_importance(self, event)
    }

    pub fn improvement_potential_importance(&self, event: &str) -> Ratio {
        fault_tree::improvement_potential_importance(self, event)
    }

    pub fn report_analysis(&self, style: Style) {
        use prettytable::{Cell, Row, Table};

        let format = Ratio::format_args(
            uom::si::ratio::per_mille,
            uom::fmt::DisplayStyle::Description,
        );

        println!(
            "Top-Level Reliability: {}",
            format.with(self.failure_probability())
        );

        let mut table = Table::new();
        table.add_row(row![
            "Event",
            "Birnbaum Importance",
            "Improvement Potential Importance"
        ]);
        self.basic_events().iter().for_each(|event| {
            let event_name = &event.0;
            let line = Row::new(vec![
                Cell::new(event_name),
                Cell::new(&format!(
                    "{}",
                    format.with(self.birnbaum_importance(event_name))
                )),
                Cell::new(&format!(
                    "{}",
                    format.with(self.improvement_potential_importance(event_name))
                )),
            ]);
            table.add_row(line);
        });
        match style {
            Style::Pretty => table.printstd(),
            Style::CSV => {
                let out = std::io::stdout();
                table.to_csv(out).unwrap();
            }
        }
    }
}

impl From<Event> for Tree {
    fn from(event: Event) -> Self {
        Self::BasicEvent(event)
    }
}

impl From<Gate> for Tree {
    fn from(gate: Gate) -> Self {
        Self::Gate(gate)
    }
}

impl<T> From<T> for Event
where
    T: Into<String>,
{
    fn from(description: T) -> Self {
        Self(description.into(), Ratio::new::<percent>(0.into()))
    }
}

#[cfg(test)]
mod tests {
    use uom::si::ratio::per_mille;

    use super::*;

    #[test]
    fn test_single_cut_set() {
        let tree = trees::bus_tree();

        assert_eq!(
            tree.cut_set(&BTreeSet::from_iter(vec![Event(
                "Late breakfast".into(),
                Ratio::new::<percent>(0.into())
            )
            .into()])),
            true
        );
        assert_eq!(
            tree.cut_set(&BTreeSet::from_iter(vec![Event(
                "Did not reach bus stop on time".into(),
                Ratio::new::<percent>(0.into())
            )
            .into()])),
            true
        );
        assert_eq!(
            tree.cut_set(&BTreeSet::from_iter(vec![
                Event("Slept late".into(), Ratio::new::<percent>(0.into())).into(),
                Event("Faulty Alarm".into(), Ratio::new::<percent>(0.into())).into(),
            ])),
            true
        );
    }

    #[test]
    fn test_single_path_set() {
        let tree = trees::bus_tree();
        assert_eq!(
            tree.path_set(&BTreeSet::from_iter(vec![
                Event("Late breakfast".into(), Ratio::new::<percent>(0.into())).into(),
                Event(
                    "Did not reach bus stop on time".into(),
                    Ratio::new::<percent>(0.into())
                )
                .into(),
                Event("Slept late".into(), Ratio::new::<percent>(0.into())).into(),
            ])),
            true
        );
    }

    #[test]
    fn test_minimal_cut_sets() {
        let tree = trees::bus_tree();

        let expected = BTreeSet::from_iter(vec![
            BTreeSet::from_iter(vec!["Slept late".into(), "Faulty Alarm".into()]),
            BTreeSet::from_iter(vec!["Slept late".into(), "Forgot to set alarm".into()]),
            BTreeSet::from_iter(vec!["Late breakfast".into()]),
            BTreeSet::from_iter(vec!["Did not reach bus stop on time".into()]),
        ]);
        assert_eq!(tree.naive_minimal_cut_sets(), expected);
        assert_eq!(tree.minimal_cut_sets(), expected);
    }

    #[test]
    fn test_minimal_path_sets() {
        let tree = trees::bus_tree();

        let expected = BTreeSet::from_iter(vec![
            BTreeSet::from_iter(vec![
                Event("Late breakfast".into(), Ratio::new::<percent>(0.into())).into(),
                Event(
                    "Did not reach bus stop on time".into(),
                    Ratio::new::<percent>(0.into()),
                )
                .into(),
                Event("Slept late".into(), Ratio::new::<percent>(0.into())).into(),
            ]),
            BTreeSet::from_iter(vec![
                Event("Late breakfast".into(), Ratio::new::<percent>(0.into())).into(),
                Event(
                    "Did not reach bus stop on time".into(),
                    Ratio::new::<percent>(0.into()),
                )
                .into(),
                Event("Faulty Alarm".into(), Ratio::new::<percent>(0.into())).into(),
                Event(
                    "Forgot to set alarm".into(),
                    Ratio::new::<percent>(0.into()),
                )
                .into(),
            ]),
        ]);
        assert_eq!(tree.minimal_path_sets(), expected);
    }

    #[test]
    fn test_basic_events() {
        let tree = trees::bus_tree();

        let expected = BTreeSet::from_iter(vec![
            "Late breakfast".into(),
            "Slept late".into(),
            "Faulty Alarm".into(),
            "Forgot to set alarm".into(),
            "Did not reach bus stop on time".into(),
        ]);
        assert_eq!(tree.basic_events(), expected);
    }

    #[test]
    fn test_prob_basic_event() {
        let tree: Tree = Event("Slept late".into(), Ratio::new::<percent>(5.into())).into();
        assert_eq!(tree.failure_probability(), Ratio::new::<percent>(5.into()));
    }

    #[test]
    fn test_prob_and() {
        let e1 = Event("Slept late".into(), Ratio::new::<percent>(5.into()));
        let e2 = Event("Late breakfast".into(), Ratio::new::<percent>(10.into()));
        let tree = Tree::Gate(Gate::And(vec![e1.into(), e2.into()]));
        assert_eq!(
            tree.failure_probability(),
            Ratio::new::<per_mille>(5.into())
        );
    }

    #[test]
    fn test_prob_or() {
        let e1 = Event("Slept late".into(), Ratio::new::<percent>(5.into()));
        let e2 = Event("Late breakfast".into(), Ratio::new::<percent>(10.into()));
        let tree = Tree::Gate(Gate::Or(vec![e1.into(), e2.into()]));
        assert_eq!(
            tree.failure_probability(),
            Ratio::new::<per_mille>(145.into())
        );
    }

    #[test]
    fn test_q0_event() {
        let e1 = Event("Slept late".into(), Ratio::new::<percent>(5.into()));
        let e2 = Event("Late breakfast".into(), Ratio::new::<percent>(10.into()));
        let tree = Tree::Gate(Gate::Or(vec![e1.into(), e2.into()]));
        let events = &BTreeSet::from_iter(vec![]);
        assert_eq!(tree.q0(events), Ratio::new::<per_mille>(145.into()));
        let events = &BTreeSet::from_iter(vec![Event(
            "Late breakfast".into(),
            Ratio::new::<percent>(0.into()),
        )]);
        assert_eq!(tree.q0(events), Ratio::new::<percent>(5.into()));
    }

    #[test]
    fn test_q0_event_dependent() {
        let e1 = Event("Slept late".into(), Ratio::new::<percent>(5.into()));
        let e2 = Event("Late breakfast".into(), Ratio::new::<percent>(10.into()));
        let tree = Tree::Gate(Gate::And(vec![
            e1.clone().into(),
            Tree::Gate(Gate::Or(vec![e1.clone().into(), e2.into()])),
        ]));
        let events = &BTreeSet::from_iter(vec![]);
        assert_eq!(
            tree.q0(events),
            Ratio::new::<uom::si::ratio::part_per_million>(54750.into())
        );

        // q0 overapproximates in this case, failure probability should be exact
        assert_eq!(
            tree.failure_probability(),
            Ratio::new::<uom::si::ratio::part_per_million>(7250.into())
        );
    }

    #[test]
    fn test_birnbaum() {
        let tree = trees::train_tree();
        assert_eq!(
            tree.birnbaum_importance("Alarm not set"),
            Ratio::new::<percent>(27.into())
        );
        assert_eq!(
            tree.birnbaum_importance("Slept late"),
            Ratio::new::<percent>(18.into())
        );
        assert_eq!(
            tree.birnbaum_importance("Train late"),
            Ratio::new::<percent>(94.into())
        );
    }

    #[test]
    fn test_improvement_potential() {
        let tree = trees::train_tree();
        assert_eq!(
            tree.improvement_potential_importance("Slept late"),
            Ratio::new::<per_mille>(54.into())
        );
        assert_eq!(
            tree.improvement_potential_importance("Alarm not set"),
            Ratio::new::<per_mille>(54.into())
        );
        assert_eq!(
            tree.improvement_potential_importance("Train late"),
            Ratio::new::<per_mille>(94.into())
        );
    }
}
