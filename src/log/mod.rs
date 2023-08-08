use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use thiserror::Error;

pub mod file;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Log {
    pub keys: Vec<String>,
    pub units: Vec<Option<String>>,
    pub entries: Vec<Vec<Option<f64>>>,
    pub timestamps: Vec<DateTime<Local>>,
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
        let mut units = HashMap::<String, String>::new();
        lines.next().unwrap().split(',').for_each(|s| {
            let mut iter = s.rsplit(' ');
            let s = iter.next().unwrap();
            let mut sub_iter = s.split('[');
            let key = sub_iter.next().unwrap().to_string();
            let val = sub_iter.next().unwrap().trim_matches(']').to_string();
            while let Some(alt_key) = iter.next().filter(|s| *s != "Logdaten" && !s.is_empty()) {
                units.insert(alt_key.to_string(), val.clone());
            }
            units.insert(key, val);
        });
        units.insert("S".to_owned(), "Status".to_owned());
        units.insert("Err".to_owned(), "Error".to_owned());
        // leptos::log!("{:?}", units);
        let keys: Vec<String> = match lines.next() {
            Some(lines) => lines.split('\t').skip(1).map(|s| s.to_string()).collect(),
            None => {
                return Err(ParseError::NotEnoughLines);
            }
        };
        let units: Vec<_> = keys
            .iter()
            .map(|k| units.get(k.rsplit(' ').next().unwrap()).map(|s| s.clone()))
            .collect();
        // leptos::log!("{:?}", keys.iter().zip(units.iter()).collect::<Vec<_>>());
        let len = keys.len();
        let mut entries: Vec<_> = (0..len).map(|_| (Vec::<Option<f64>>::new())).collect();
        let mut timestamps = Vec::new();
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
                timestamps.push(DateTime::<Local>::from_utc(
                    NaiveDateTime::from_timestamp_opt(entry[0].unwrap() as i64, 0).unwrap(),
                    FixedOffset::east_opt(2 * 60 * 60).unwrap(),
                ));
                (1..keys.len()).for_each(|i| {
                    entries
                        .get_mut(i - 1)
                        .unwrap()
                        .push(*entry.get(i).unwrap_or(&None))
                });
            });
        let log = Log {
            keys,
            units,
            entries,
            timestamps,
        };
        Ok(log)
    }

    pub fn get_legend(&self, idx: usize) -> String {
        format!(
            "{}{}",
            self.keys[idx].clone(),
            self.units[idx]
                .clone()
                .map(|u| " in ".to_string() + &u)
                .unwrap_or("".to_string())
        )
    }
}
