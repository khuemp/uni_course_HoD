use crate::{Event, Gate, Tree};
use std::collections::BTreeSet;
use uom::num_rational::Ratio;

pub(super) fn powerset<T: Ord + Clone>(mut set: BTreeSet<T>) -> BTreeSet<BTreeSet<T>> {
    // power set (P) = set contains all possible combination of basic events
    if set.is_empty() {
        let mut powerset = BTreeSet::new();
        powerset.insert(set);
        return powerset;
    }
    let entry = set.iter().next().unwrap().clone(); // move first element into "entry"...
    set.remove(&entry); // ... and delete it
    let mut powerset = powerset(set); // recursion removing second, third,... element from the set
    for mut set in powerset.clone().into_iter() {
        set.insert(entry.clone());
        powerset.insert(set);
    }
    powerset
}

pub fn parse_dft(text: &str) -> Tree {
    let mut lines = text.lines();
    let toplevel = lines.next().unwrap();
    let toplevel = toplevel
        .split(" ")
        .nth(1)
        .unwrap()
        .replace('"', "")
        .replace(';', "");
    let lines = lines.rev();
    let mut already_parsed = vec![];
    for line in lines {
        let tree = parse_line(line, &mut already_parsed);
        already_parsed.push(tree);
    }
    Tree::IntermediateEvent(toplevel, Box::new(already_parsed.pop().unwrap())).into()
}

fn parse_line(line: &str, already_parsed: &mut Vec<Tree>) -> Tree {
    let line = line.replace(';', "").to_string();
    let mut parts = line.split(" ");
    let name = parts.next().unwrap().replace('"', "");
    let t = parts.next().unwrap();
    match t {
        "or" | "and" => {
            let subtrees = parts
                .map(|part| {
                    let (index, _) = already_parsed
                        .iter()
                        .enumerate()
                        .find(|(_, already)| match already {
                            Tree::Gate(_) => false,
                            Tree::BasicEvent(Event(name, _)) | Tree::IntermediateEvent(name, _) => {
                                name == &part.replace('"', "")
                            }
                        })
                        .unwrap_or_else(|| panic!("Could not find {}", part));
                    already_parsed.remove(index)
                })
                .collect::<Vec<_>>();
            match t {
                "or" => {
                    Tree::IntermediateEvent(name, Box::new(Tree::Gate(Gate::Or(subtrees)))).into()
                }
                "and" => {
                    Tree::IntermediateEvent(name, Box::new(Tree::Gate(Gate::And(subtrees)))).into()
                }
                _ => unreachable!(),
            }
        }
        _ => {
            if t.starts_with("prob=") {
                let mut parts = t.split("=");
                parts.next().unwrap();
                let ratio = parts.next().unwrap().replace(';', "");
                let denom = 10usize.pow(if t.contains("e-") {
                    t.split("e-").nth(1).unwrap().parse::<u32>().unwrap()
                } else {
                    (ratio.len() - 2) as u32
                });
                let numer = (ratio.parse::<f64>().unwrap() * (denom as f64)) as usize;
                Event(name, Ratio::new(numer as i64, denom as i64).into()).into()
            } else {
                todo!("{:#?}", parts)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_event_and_gate() {
        let event1 = parse_line(r#""X5_3_1" prob=2.0e-4;"#, &mut vec![]);
        assert_eq!(
            event1,
            Tree::BasicEvent(Event("X5_3_1".to_string(), Ratio::new(2, 10000).into())),
        );

        let event2 = parse_line(r#""X5_3_2" prob=0.002;"#, &mut vec![]);
        assert_eq!(
            event2,
            Tree::BasicEvent(Event("X5_3_2".to_string(), Ratio::new(2, 1000).into())),
        );

        let mut events = vec![event1.clone(), event2.clone()];

        let or = parse_line(r#""G5_3" or "X5_3_1" "X5_3_2";"#, &mut events);
        let expected = Tree::IntermediateEvent(
            "G5_3".into(),
            Box::new(Tree::Gate(Gate::Or(vec![event1, event2]))),
        )
        .into();
        assert_eq!(or, expected);

        let expected = Tree::IntermediateEvent("G5_3".into(), Box::new(expected));
        let tree = parse_dft(&std::fs::read_to_string("ffort/simple.dft").unwrap());
        assert_eq!(tree, expected.into());
    }
}
