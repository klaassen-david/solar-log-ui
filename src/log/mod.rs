use leptos::IntoView;
use plotly::{Plot, Scatter};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Log {
    pub keys: Vec<String>,
    pub units: String,
    pub entries: Vec<Box<Vec<Option<f32>>>>,
    pub timestamps: Vec<i32>,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("could not read file")]
    IOError(#[from] std::io::Error),
    #[error("log files contain at least a header and the data scheme")]
    NotEnoughLines,
}

impl Log {
    pub fn parse_file(path: String) -> Result<Self, ParseError> {
        let binding = std::fs::read_to_string(path)?;
        let mut lines = binding.lines();
        (0..5).for_each(|_| {
            lines.next();
        }); // cut header
        let units = lines.next().unwrap().to_string();
        let keys: Vec<String> = match lines.next() {
            Some(lines) => lines.split('\t').map(|s| s.to_string()).collect(),
            None => {
                return Err(ParseError::NotEnoughLines);
            }
        };
        let len = keys.len();
        let entries: Vec<Box<Vec<Option<f32>>>> = (0..len)
            .map(|_| Box::new(Vec::<Option<f32>>::new()))
            .collect();
        let mut log = Log {
            keys,
            units,
            entries,
            timestamps: Vec::new(),
        };
        lines
            .into_iter()
            .map(|line| {
                line.split('\t')
                    .map(|v| match v.parse::<f32>() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .collect::<Vec<Option<f32>>>()
            })
            .for_each(|entry| {
                log.timestamps.push(entry[0].unwrap() as i32);
                (1..log.keys.len()).for_each(|i| {
                    log.entries
                        .get_mut(i)
                        .unwrap()
                        .push(*entry.get(i).unwrap_or(&None))
                });
            });
        Ok(log)
    }

    async fn draw(&self) -> impl IntoView {
        let id = "plotid";
        let mut plot = Plot::new();
        let trace = Scatter::new(self.timestamps.clone(), *self.entries[3].clone());
        plot.add_trace(trace);
        plotly::bindings::new_plot(id, &plot).await;
    }
}
