use prettytable::{Table, Row, Cell};

Commands::ListBots { all, page, limit } => {
    let response = rest_client.get_bots(all, page, limit).await?;

    // Create a table for the output
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Bot ID"),
        Cell::new("Name"),
        Cell::new("Exchange"),
    ]));

    for bot in response {
        table.add_row(Row::new(vec![
            Cell::new(&bot.bot_id),
            Cell::new(&bot.name.unwrap_or_else(|| "N/A".to_string())),
            Cell::new(&bot.exchange.unwrap_or_else(|| "N/A".to_string())),
        ]));
    }

    table.printstd();
    Ok(())
}
