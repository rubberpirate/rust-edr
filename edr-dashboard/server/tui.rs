// TUI Dashboard using ratatui
use crate::{EndpointState, EndpointInfo};
use edr_common::types::*;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Gauge},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

pub async fn run_tui(state: EndpointState) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal, state).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: EndpointState) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        let endpoints = state.read().await;
        let endpoints_vec: Vec<EndpointInfo> = endpoints.values().cloned().collect();
        drop(endpoints);

        terminal.draw(|f| ui(f, &endpoints_vec))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, endpoints: &[EndpointInfo]) {
    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Footer
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new("🛡️  EDR Central Monitoring Dashboard")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content - split into two sections
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),  // Left: Endpoint list
            Constraint::Percentage(65),  // Right: Details
        ])
        .split(chunks[1]);

    // Left panel: Endpoint list
    render_endpoint_list(f, main_chunks[0], endpoints);

    // Right panel: Details (split vertically)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),  // Stats
            Constraint::Percentage(60),  // Events
        ])
        .split(main_chunks[1]);

    render_stats_panel(f, right_chunks[0], endpoints);
    render_events_panel(f, right_chunks[1], endpoints);

    // Footer
    let footer = Paragraph::new("Press 'q' to quit | Real-time monitoring active")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_endpoint_list(f: &mut Frame, area: Rect, endpoints: &[EndpointInfo]) {
    let items: Vec<ListItem> = if endpoints.is_empty() {
        vec![ListItem::new("⏳ Waiting for agents...").style(Style::default().fg(Color::Yellow))]
    } else {
        endpoints
            .iter()
            .map(|ep| {
                let status = get_endpoint_status(ep);
                let color = if status == "🟢 Online" {
                    Color::Green
                } else {
                    Color::Red
                };

                let lines = vec![
                    Line::from(vec![
                        Span::styled("━━━━━━━━━━━━━━━━━━━━", Style::default().fg(Color::DarkGray)),
                    ]),
                    Line::from(vec![
                        Span::styled("ID: ", Style::default().fg(Color::Cyan)),
                        Span::raw(&ep.endpoint_id),
                    ]),
                    Line::from(vec![
                        Span::styled("Host: ", Style::default().fg(Color::Cyan)),
                        Span::raw(&ep.hostname),
                    ]),
                    Line::from(vec![
                        Span::styled("Status: ", Style::default().fg(Color::Cyan)),
                        Span::styled(status, Style::default().fg(color)),
                    ]),
                    Line::from(vec![
                        Span::styled("CPU: ", Style::default().fg(Color::Yellow)),
                        Span::raw(format!("{:.1}%", ep.stats.cpu_usage)),
                    ]),
                    Line::from(vec![
                        Span::styled("MEM: ", Style::default().fg(Color::Yellow)),
                        Span::raw(format!(
                            "{:.1}/{:.1} GB",
                            ep.stats.memory_used_gb, ep.stats.memory_total_gb
                        )),
                    ]),
                    Line::from(vec![
                        Span::styled("Proc: ", Style::default().fg(Color::Magenta)),
                        Span::raw(format!("{}", ep.stats.process_count)),
                    ]),
                ];

                ListItem::new(lines)
            })
            .collect()
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("📡 Endpoints ({})", endpoints.len()))
            .style(Style::default().fg(Color::White)),
    );

    f.render_widget(list, area);
}

fn render_stats_panel(f: &mut Frame, area: Rect, endpoints: &[EndpointInfo]) {
    let stats_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),   // Aggregated stats
            Constraint::Min(3),      // Gauges
        ])
        .split(area);

    // Aggregated stats
    let total_processes: usize = endpoints.iter().map(|e| e.stats.process_count).sum();
    let total_connections: usize = endpoints.iter().map(|e| e.stats.network_connections).sum();
    let avg_cpu = if !endpoints.is_empty() {
        endpoints.iter().map(|e| e.stats.cpu_usage).sum::<f64>() / endpoints.len() as f64
    } else {
        0.0
    };
    let total_threats: usize = endpoints.iter().map(|e| e.recent_threats.len()).sum();

    let stats_text = vec![
        Line::from(vec![
            Span::styled("Total Processes: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}", total_processes),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Network Connections: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}", total_connections),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Avg CPU Usage: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.1}%", avg_cpu),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Active Threats: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}", total_threats),
                Style::default()
                    .fg(if total_threats > 0 { Color::Red } else { Color::Green })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let stats_para = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("📊 System Overview")
            .style(Style::default().fg(Color::White)),
    );

    f.render_widget(stats_para, stats_chunks[0]);

    // Gauges for first endpoint (if available)
    if let Some(ep) = endpoints.first() {
        let gauge_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3)])
            .split(stats_chunks[1]);

        let cpu_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
            .gauge_style(Style::default().fg(Color::Yellow))
            .percent(ep.stats.cpu_usage as u16);
        f.render_widget(cpu_gauge, gauge_chunks[0]);

        let mem_percent = if ep.stats.memory_total_gb > 0.0 {
            ((ep.stats.memory_used_gb / ep.stats.memory_total_gb) * 100.0) as u16
        } else {
            0
        };
        let mem_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(mem_percent);
        f.render_widget(mem_gauge, gauge_chunks[1]);
    }
}

fn render_events_panel(f: &mut Frame, area: Rect, endpoints: &[EndpointInfo]) {
    let event_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Recent threats
    let mut threat_items: Vec<ListItem> = Vec::new();
    for ep in endpoints {
        for threat in ep.recent_threats.iter().rev().take(5) {
            let severity_color = match threat.severity {
                edr_common::types::Severity::Critical => Color::Red,
                edr_common::types::Severity::High => Color::LightRed,
                edr_common::types::Severity::Medium => Color::Yellow,
                edr_common::types::Severity::Low => Color::Green,
                edr_common::types::Severity::Info => Color::Blue,
            };

            threat_items.push(ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        format!("{:?}", threat.severity),
                        Style::default().fg(severity_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" - "),
                    Span::raw(&threat.threat_type),
                ]),
                Line::from(vec![
                    Span::styled("  └─ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&ep.hostname, Style::default().fg(Color::Cyan)),
                    Span::raw(": "),
                    Span::raw(&threat.description),
                ]),
            ]));
        }
    }

    if threat_items.is_empty() {
        threat_items.push(ListItem::new("✅ No threats detected").style(Style::default().fg(Color::Green)));
    }

    let threats_list = List::new(threat_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("⚠️  Recent Threats")
            .style(Style::default().fg(Color::Red)),
    );

    f.render_widget(threats_list, event_chunks[0]);

    // Recent events (processes + network)
    let mut event_items: Vec<ListItem> = Vec::new();
    for ep in endpoints {
        for proc in ep.recent_processes.iter().rev().take(3) {
            let icon = match proc.event_type {
                edr_common::types::ProcessEventType::Created => "🟢",
                edr_common::types::ProcessEventType::Terminated => "🔴",
            };
            event_items.push(ListItem::new(vec![
                Line::from(vec![
                    Span::raw(icon),
                    Span::raw(" Process: "),
                    Span::styled(&proc.name, Style::default().fg(Color::Yellow)),
                ]),
                Line::from(vec![
                    Span::styled("  └─ ", Style::default().fg(Color::DarkGray)),
                    Span::raw(format!("PID {} on {}", proc.pid, ep.hostname)),
                ]),
            ]));
        }

        for net in ep.recent_network.iter().rev().take(3) {
            let (icon, _bytes_opt) = match &net.event_type {
                edr_common::types::NetworkEventType::ConnectionOpened => ("🔵", None),
                edr_common::types::NetworkEventType::ConnectionClosed => ("⚫", None),
                edr_common::types::NetworkEventType::DataTransferred { bytes } => ("📡", Some(*bytes)),
            };
            event_items.push(ListItem::new(vec![
                Line::from(vec![
                    Span::raw(icon),
                    Span::raw(" Network: "),
                    Span::styled(
                        format!("{}:{} → {}:{}", net.local_addr, net.local_port, net.remote_addr, net.remote_port),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("  └─ ", Style::default().fg(Color::DarkGray)),
                    Span::raw(&ep.hostname),
                ]),
            ]));
        }
    }

    if event_items.is_empty() {
        event_items.push(ListItem::new("⏳ Waiting for events...").style(Style::default().fg(Color::Gray)));
    }

    let events_list = List::new(event_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("📋 Recent Events")
            .style(Style::default().fg(Color::White)),
    );

    f.render_widget(events_list, event_chunks[1]);
}

fn get_endpoint_status(ep: &EndpointInfo) -> &'static str {
    let now = chrono::Utc::now();
    let elapsed = now.signed_duration_since(ep.last_seen);

    if elapsed.num_seconds() < 10 {
        "🟢 Online"
    } else {
        "🔴 Offline"
    }
}
