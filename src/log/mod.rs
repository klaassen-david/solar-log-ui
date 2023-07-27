use leptos::log;
use plotly::{Scatter, Trace};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod file;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Log {
    pub keys: Vec<String>,
    pub units: String,
    pub entries: Vec<Box<Vec<Option<f64>>>>,
    pub timestamps: Vec<i64>,
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
        Self::parse(binding)
    }

    pub fn parse(content: String) -> Result<Self, ParseError> {
        let mut lines = content.lines();
        (0..5).for_each(|_| {
            lines.next();
        }); // cut header
        let units: String = lines.next().unwrap().to_string();
        let keys: Vec<String> = match lines.next() {
            Some(lines) => lines.split('\t').map(|s| s.to_string()).collect(),
            None => {
                return Err(ParseError::NotEnoughLines);
            }
        };
        let len = keys.len();
        let entries = (0..len)
            .map(|_| Box::new(Vec::<Option<f64>>::new()))
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
                    .map(|v| match v.parse::<f64>() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .collect::<Vec<Option<f64>>>()
            })
            .for_each(|entry| {
                log.timestamps.push(entry[0].unwrap() as i64);
                (1..log.keys.len()).for_each(|i| {
                    log.entries
                        .get_mut(i - 1)
                        .unwrap()
                        .push(*entry.get(i).unwrap_or(&None))
                });
            });
        Ok(log)
    }

    pub fn scatter() -> Box<dyn Trace> {
        todo!()
    }
}
