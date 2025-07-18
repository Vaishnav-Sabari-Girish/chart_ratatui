use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::*,
    prelude::*,
    symbols,
    Terminal
};
use rand::Rng;
use std::{
    error::Error,
    io,
    time::Duration
};

pub fn chart_run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut data: Vec<(f64, f64)> = Vec::with_capacity(100);
    let mut x = 0.0;
    
    loop {
        let y = rand::rng().random_range(-1.0..1.0);
        data.push((x, y));
        
        if data.len() > 100 {
            data.remove(0);
        }
        
        x += 0.1;
        
        // Calculate dynamic bounds for x-axis
        let x_min = if data.is_empty() { 0.0 } else { data[0].0 };
        let x_max = if data.is_empty() { 10.0 } else { data[data.len() - 1].0 };
        
        // Create labels outside the closure to avoid borrowing issues
        let x_min_label = format!("{:.1}", x_min);
        let x_mid_label = format!("{:.1}", (x_min + x_max) / 2.0);
        let x_max_label = format!("{:.1}", x_max);
        
        terminal.draw(|f| {
            let size = f.area();
            
            let chart = Chart::new(vec![
                Dataset::default()
                    .name("Random Data")
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&data)
            ])
            .block(
                Block::default()
                    .title("Live Serial Data")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([x_min, x_max])
                    .labels(vec![
                        x_min_label.as_str(),
                        x_mid_label.as_str(),
                        x_max_label.as_str(),
                    ])
            )
            .y_axis(
                Axis::default()
                    .title("Value")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([-1.2, 1.2])
                    .labels(vec!["-1.0", "0.0", "1.0"])
            );
            
            f.render_widget(chart, size);
        })?;
        
        // Check for exit condition
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
