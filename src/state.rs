use std::cell::OnceCell;

use chrono::{DateTime, Local};
use leptos::{RwSignal, SignalGet};
use plotly::Scatter;

use crate::log::Log;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trace {
    pub idx: RwSignal<usize>,
}

#[derive(Clone, Default)]
pub struct GlobalState {
    pub traces: Vec<(usize, Trace)>,
}

impl Trace {
    pub fn new(&self, idx: RwSignal<usize>) -> Self {
        Self { idx }
    }

    pub fn render(&self, log: OnceCell<Log>) -> Box<Scatter<DateTime<Local>, f64>> {
        let Log {
            entries,
            timestamps,
            ..
        } = log.get().unwrap();
        let entries: Vec<Option<f64>> = entries.get(self.idx.get()).unwrap().clone();
        let (timestamps, entries): (Vec<DateTime<Local>>, Vec<f64>) = timestamps
            .into_iter()
            .zip(entries.into_iter())
            .filter_map(|(t, e)| match e {
                Some(v) => Some((t, v)),
                None => None,
            })
            .unzip();
        Scatter::new(timestamps, entries).name(log.get().unwrap().get_legend(self.idx.get()))
    }
}
