use crate::bot::state::BotView;
use prettytable::{format, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotListView(pub Vec<BotView>);

impl fmt::Display for BotListView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        // Set table format (optional)
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        // Add header row
        table.add_row(Row::new(vec![
            Cell::new("Bot ID"),
            Cell::new("Name"),
            Cell::new("Exchange"),
            Cell::new("Trading Fee"),
        ]));

        // Add rows for each bot
        for bot in &self.0 {
            table.add_row(Row::new(vec![
                Cell::new(&bot.bot_id),
                Cell::new(&bot.name),
                Cell::new(&bot.exchange),
                Cell::new(
                    &bot.trading_fee
                        .map_or("N/A".to_string(), |fee| format!("{:.4}", fee)),
                ),
            ]));
        }

        // Write the table to the formatter
        write!(f, "{}", table)
    }
}
