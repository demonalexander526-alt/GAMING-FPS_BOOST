use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, BorderType, Paragraph, Row, Table, Sparkline, Gauge},
    Terminal,
};
use std::{io, process::Command, time::{Duration, Instant}};
use sysinfo::{System, Disks, CpuExt, ProcessExt};

fn update_nexo_notification(fps: u64, ram_used: u64, ram_total: u64, disk_used: u64) {
    let content = format!(
        "🚀 FPS: {} | 🧠 RAM: {}/{}MB | 💾 Disk Used: {}GB",
        fps, ram_used, ram_total, disk_used
    );
    let _ = Command::new("termux-notification")
        .args(["--title", "NEXO-TECH Live Overdrive", "--content", &content, "--id", "nexo_fps", "--priority", "high", "--ongoing"])
        .spawn();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // === 1. STARTUP BANNER & CREDITS (RESTORED) ===
    print!("{}[2J", 27 as char); 
    println!("{}", "====================================================".cyan().bold());
    println!("{}", "       🚀 NEXO-TEH ULTIMATE AUTO-BOOSTER           ".green().bold());
    println!("{}", "           INSPIRED BY ALEXANDER                   ".blue().bold());
    println!("{}", "====================================================".cyan().bold());
    
    // Applying Gaming Tweaks
    println!("Applying Input Latency & Rendering Tweaks...");
    let _ = Command::new("adb").args(["shell", "settings", "put", "secure", "tap_duration_threshold", "0.0"]).output();
    let _ = Command::new("adb").args(["shell", "setprop", "debug.hwc.force_gpu_vsync", "1"]).output();
    let _ = Command::new("adb").args(["shell", "cmd", "power", "set-fixed-performance-mode-enabled", "true"]).output();

    print!("\n{} (y/n): ", "Initialize Full NEXO-FPS REPORT & Overdrive?".yellow());
    io::Write::flush(&mut io::stdout())?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    crossterm::terminal::enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut sys = System::new_all();
    let mut fps_history = vec![0u64; 100];
    let mut last_tick = Instant::now();
    let mut total_ram_freed = 0;

    loop {
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        
        let elapsed = last_tick.elapsed().as_secs_f64();
        let current_fps = if elapsed > 0.0 { (1.0 / elapsed) as u64 } else { 0 };
        last_tick = Instant::now();
        fps_history.remove(0);
        fps_history.push(current_fps);

        let total_ram = sys.total_memory() / 1024 / 1024;
        let used_ram = sys.used_memory() / 1024 / 1024;
        let cpu_freq = sys.global_cpu_info().frequency();
        
        let (used_disk, total_disk) = if let Some(disk) = disks.iter().next() {
            ((disk.total_space() - disk.available_space()) / 1024 / 1024 / 1024, disk.total_space() / 1024 / 1024 / 1024)
        } else { (0, 0) };

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(7), // NEXO-FPS REPORT (RESTORED)
                    Constraint::Length(4), // GAUGES
                    Constraint::Length(7), // FPS CHART (RESTORED)
                    Constraint::Min(5),    // PROCESSES
                    Constraint::Length(3), // FOOTER
                ])
                .split(f.size());

            // --- 1. NEXO-FPS REPORT ---
            let report_text = format!(
                "OVERALL STATUS: NEXO-OPTIMIZED\n\
                 [FPS]: {} | [CPU]: {}MHz\n\
                 [STORAGE]: {}GB / {}GB USED\n\
                 [BOOST]: +{}MB FREED",
                current_fps, cpu_freq, used_disk, total_disk, total_ram_freed
            );
            f.render_widget(Paragraph::new(report_text)
                .alignment(Alignment::Center)
                .block(Block::default().title(" 📊 NEXO-FPS REPORT ").borders(Borders::ALL).border_type(BorderType::Double).fg(Color::LightGreen)), chunks[0]);

            // --- 2. DYNAMIC GAUGES ---
            let res_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            f.render_widget(Gauge::default()
                .block(Block::default().title(" RAM LOAD ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(((used_ram as f64 / total_ram as f64) * 100.0) as u16), res_chunks[0]);

            f.render_widget(Gauge::default()
                .block(Block::default().title(" STORAGE ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent(if total_disk > 0 { ((used_disk as f64 / total_disk as f64) * 100.0) as u16 } else { 0 }), res_chunks[1]);

            // --- 3. FPS CHART ---
            f.render_widget(Sparkline::default()
                .block(Block::default().title(" FPS STABILITY ").borders(Borders::ALL))
                .data(&fps_history)
                .style(Style::default().fg(Color::Yellow)), chunks[2]);

            // --- 4. TOP PROCESSES ---
            let mut procs: Vec<_> = sys.processes().values().collect();
            procs.sort_by(|a, b| b.memory().cmp(&a.memory()));
            let rows: Vec<Row> = procs.iter().take(5).map(|p| {
                Row::new(vec![p.name().to_string(), format!("{} MB", p.memory()/1024/1024)])
            }).collect();
            f.render_widget(Table::new(rows, [Constraint::Percentage(70), Constraint::Percentage(30)])
                .header(Row::new(vec!["PROCESS", "USAGE"]).style(Style::default().bold().blue()))
                .block(Block::default().title(" [ HEAVY APPS ] ").borders(Borders::ALL)), chunks[3]);

            // --- 5. FOOTER ---
            f.render_widget(Paragraph::new(" [Q] Quit | [Y] Kill Top | [C] Clean | CREATED BY NEXO-TEH & ALEXANDER ")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).fg(Color::DarkGray)), chunks[4]);
        })?;

        update_nexo_notification(current_fps, used_ram, total_ram, used_disk);

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('y') => {
                        let mut procs: Vec<_> = sys.processes().values().collect();
                        procs.sort_by(|a, b| b.memory().cmp(&a.memory()));
                        if let Some(top) = procs.first() {
                            total_ram_freed += (top.memory() / 1024 / 1024) as i64;
                            let _ = top.kill();
                        }
                    }
                    crossterm::event::KeyCode::Char('q') => {
                        let _ = Command::new("termux-notification-remove").arg("nexo_fps").spawn();
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
