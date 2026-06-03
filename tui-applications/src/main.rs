use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    DefaultTerminal, Frame,
};

// ─── 1. Clap：定義命令列參數 ─────────────────────────────────────────────────

/// 一個簡單的 Clap + Ratatui TUI 計數器應用
#[derive(Parser, Debug)]
#[command(author, version, about = "Simple Ratatui + Clap TUI Counter", long_about = None)]
struct Cli {
    /// 顯示在 TUI 標題的名稱
    #[arg(short, long, default_value = "World")]
    name: String,

    /// 計數器起始值 (0–100)
    #[arg(short, long, default_value_t = 0)]
    start: u16,
}

// ─── 2. 應用狀態 ──────────────────────────────────────────────────────────────

struct App {
    name: String,
    counter: u16, // 0 ~ 100
    should_quit: bool,
}

impl App {
    fn new(name: String, start: u16) -> Self {
        Self {
            name,
            counter: start.clamp(0, 100),
            should_quit: false,
        }
    }

    fn increment(&mut self) {
        if self.counter < 100 {
            self.counter += 1;
        }
    }

    fn decrement(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}

// ─── 3. 主程式：初始化 Terminal ───────────────────────────────────────────────

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut app = App::new(cli.name, cli.start);
    let mut terminal = ratatui::init(); // 進入 raw mode + alternate screen
    let result = run(&mut terminal, &mut app);
    ratatui::restore(); // 離開前還原終端
    result
}

// ─── 4. 事件迴圈 ──────────────────────────────────────────────────────────────

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Up | KeyCode::Char('k') => app.increment(),
                    KeyCode::Down | KeyCode::Char('j') => app.decrement(),
                    KeyCode::Char('r') => app.reset(),
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

// ─── 5. UI 渲染 ───────────────────────────────────────────────────────────────

fn render(frame: &mut Frame, app: &App) {
    // 切成三個垂直區塊
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // 標題
            Constraint::Length(5), // 計數器
            Constraint::Length(5), // 進度條
            Constraint::Min(0),    // 操作提示
        ])
        .split(frame.area());

    // ── 標題區塊 ──
    let title = Paragraph::new(Line::from(vec![
        Span::styled("Hello, ", Style::default().fg(Color::White)),
        Span::styled(
            &app.name,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("! 🐭", Style::default().fg(Color::White)),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Ratatui + Clap "),
    );

    frame.render_widget(title, chunks[0]);

    // ── 計數器數字 ──
    let counter_color = match app.counter {
        0..=33 => Color::Green,
        34..=66 => Color::Yellow,
        _ => Color::Red,
    };

    let counter_text = Paragraph::new(Line::from(vec![Span::styled(
        format!("  計數器：{:>3} / 100", app.counter),
        Style::default()
            .fg(counter_color)
            .add_modifier(Modifier::BOLD),
    )]))
    .block(Block::default().borders(Borders::ALL).title(" Counter "));

    frame.render_widget(counter_text, chunks[1]);

    // ── 進度條 ──
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(counter_color).bg(Color::Black))
        .percent(app.counter);

    frame.render_widget(gauge, chunks[2]);

    // ── 操作提示 ──
    let help = Paragraph::new(vec![
        Line::from("  ↑ / k   增加計數"),
        Line::from("  ↓ / j   減少計數"),
        Line::from("  r       重置"),
        Line::from("  q / Esc 離開"),
    ])
    .block(Block::default().borders(Borders::ALL).title(" 操作說明 "))
    .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(help, chunks[3]);
}
