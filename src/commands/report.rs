use anyhow::Result;

pub fn run(week: bool, month: bool, all: bool) -> Result<()> {
    // TODO: Implement reporting in Phase 4
    if week {
        println!("Weekly report - coming soon in Phase 4!");
    } else if month {
        println!("Monthly report - coming soon in Phase 4!");
    } else if all {
        println!("All-time report - coming soon in Phase 4!");
    } else {
        println!("Please specify --week, --month, or --all");
    }

    Ok(())
}
