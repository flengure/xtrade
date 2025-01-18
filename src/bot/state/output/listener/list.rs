//use crate::models::Listener;
use prettytable::{format, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::vec::IntoIter;

use super::view::ListenerView;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerListView(pub Vec<ListenerView>);

impl IntoIterator for ListenerListView {
    type Item = ListenerView;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ListenerListView {
    type Item = &'a ListenerView;
    type IntoIter = std::slice::Iter<'a, ListenerView>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ListenerListView {
    type Item = &'a mut ListenerView;
    type IntoIter = std::slice::IterMut<'a, ListenerView>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

// Implement Display for ListenerListView
impl fmt::Display for ListenerListView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        // Set the format to remove borders and gridlines
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        // Add the header row
        table.add_row(Row::new(vec![
            Cell::new("Bot ID"),
            Cell::new("Listener ID"),
            Cell::new("Service"),
            Cell::new("Message Preview"),
        ]));

        // Add a separator row (dashes under headers)
        table.add_row(Row::new(vec![
            Cell::new("------"),
            Cell::new("-----------"),
            Cell::new("-------"),
            Cell::new("---------------"),
        ]));

        // Add rows for each listener
        for listener in &self.0 {
            table.add_row(Row::new(vec![
                Cell::new(&listener.bot_id),
                Cell::new(&listener.listener_id),
                Cell::new(
                    &listener
                        .service
                        .clone()
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
                Cell::new(
                    &listener
                        .msg
                        .clone()
                        .unwrap_or_else(|| "N/A".to_string())
                        .chars()
                        .take(10)
                        .collect::<String>(),
                ),
            ]));
        }

        // Write the table to the formatter
        write!(f, "{}", table)
    }
}
