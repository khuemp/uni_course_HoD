use super::*;

pub fn bus_tree() -> Tree {
    Tree::Gate(Gate::Or(vec![
        Tree::IntermediateEvent(
            "Wake up late?".into(),
            Tree::Gate(Gate::And(vec![
                Event("Slept late".into(), Ratio::new::<percent>(0.into())).into(),
                Tree::from(Tree::IntermediateEvent(
                    "Alarm did not ring".into(),
                    Tree::Gate(Gate::Or(vec![
                        Event("Faulty Alarm".into(), Ratio::new::<percent>(0.into())).into(),
                        Event(
                            "Forgot to set alarm".into(),
                            Ratio::new::<percent>(0.into()),
                        )
                        .into(),
                    ]))
                    .into(),
                )),
            ]))
            .into(),
        )
        .into(),
        Event("Late breakfast".into(), Ratio::new::<percent>(0.into())).into(),
        Event(
            "Did not reach bus stop on time".into(),
            Ratio::new::<percent>(0.into()),
        )
        .into(),
    ]))
}

pub fn train_tree() -> Tree {
    let e1 = Event("Slept late".into(), Ratio::new::<percent>(30.into()));
    let e2 = Event("Alarm not set".into(), Ratio::new::<percent>(20.into()));
    let e3 = Event("Train late".into(), Ratio::new::<percent>(10.into()));

    Tree::Gate(Gate::Or(vec![
        Tree::Gate(Gate::And(vec![e1.into(), e2.into()])),
        e3.into(),
    ]))
}
