use crate::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct FilterInterval<T> {
    start: Option<T>,
    end: Option<T>,
}

impl<T: std::fmt::Display + Copy> FilterInterval<T> {
    pub fn query(&self, field: &str) -> String {
        format!(
            "{}{}",
            match self.start {
                Some(x) => format!(",{}>={}", field, x),
                None => "".to_string(),
            },
            match self.end {
                Some(x) => format!(",{}<{}", field, x),
                None => "".to_string(),
            },
        )
    }

    pub fn desc(&self, field: &str) -> String {
        format!(
            "{}: [{}:{}]",
            field,
            match self.start {
                Some(x) => x.to_string(),
                None => "".to_string(),
            },
            match self.end {
                Some(x) => x.to_string(),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
/// `StoryNumericFilters` defines a list of options to filter stories
pub struct StoryNumericFilters {
    #[serde(default)]
    elapsed_days_interval: FilterInterval<u32>,
    #[serde(default)]
    points_interval: FilterInterval<u32>,
    #[serde(default)]
    num_comments_interval: FilterInterval<usize>,
}

impl StoryNumericFilters {
    fn from_elapsed_days_to_unix_time(elapsed_days: Option<u32>) -> Option<u64> {
        match elapsed_days {
            None => None,
            Some(day_offset) => {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                Some(current_time - utils::from_day_offset_to_time_offset_in_secs(day_offset))
            }
        }
    }

    pub fn desc(&self) -> String {
        format!(
            "{}, {}, {}",
            self.elapsed_days_interval.desc("elapsed_days"),
            self.points_interval.desc("points"),
            self.num_comments_interval.desc("num_comments")
        )
    }

    pub fn query(&self) -> String {
        // convert elapsed_days to unix time (in seconds)
        let time_interval = FilterInterval {
            end: Self::from_elapsed_days_to_unix_time(self.elapsed_days_interval.start),
            start: Self::from_elapsed_days_to_unix_time(self.elapsed_days_interval.end),
        };

        let mut query = format!(
            "{}{}{}",
            time_interval.query("created_at_i"),
            self.points_interval.query("points"),
            self.num_comments_interval.query("num_comments")
        );
        if !query.is_empty() {
            query.remove(0); // remove trailing ,
            format!("&numericFilters={}", query)
        } else {
            "".to_string()
        }
    }
}
