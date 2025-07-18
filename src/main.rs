use std::error::Error;

mod chart;

fn main() -> Result<(), Box<dyn Error>> {
    chart::chart_run()?;

    Ok(())
}
