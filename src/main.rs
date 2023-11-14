pub mod cursorcontroller;
pub mod cursorhandler;
mod supabase;

use std::io::{self, Stdout, Write};

use clap::{Parser, Subcommand};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Color},
    terminal,
};
use cursorhandler::CursorManager;
use supabase::{Headers, Supabase};
pub enum State {
    Setup,
    Editor,
    Loading,
}
pub struct Line {
    pub value: String,
    pub colour: Color,
}

impl Line {
    pub fn new(value: String) -> Line {
        Line {
            value,
            colour: Color::Black,
        }
    }
    fn draw(&self) {
        queue!(
            io::stdout(),
            style::SetBackgroundColor(self.colour),
            style::Print(&self.value),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveToNextLine(0)
        )
        .unwrap();
    }
}
pub struct LinesController {
    lines: Vec<Line>,
    pub header: String,
}
impl LinesController {
    pub fn new(header: String, content: Vec<String>) -> LinesController {
        let rows = terminal::size().expect("").0;
        let mut header_line = header;
        for _ in 0..((rows / 2 - header_line.len() as u16 * 2) as u16) {
            header_line.insert(0, ' ')
        }
        let mut lines = vec![
            Line::new((&header_line).to_string()),
            Line::new("".to_string()),
        ];
        for i in content {
            lines.push(Line::new((&i).to_string()))
        }
        LinesController {
            lines,
            header: header_line.replace(" ", ""),
        }
    }
    pub fn change_header(&mut self, header: String) {
        let rows = terminal::size().expect("").0;
        let mut header_line = header;
        for _ in 0..((rows / 2 - header_line.len() as u16 * 2) as u16) {
            header_line.insert(0, ' ')
        }
        self.lines[0] = Line::new((&header_line).to_string());
        self.header = header_line.replace(" ", "");
    }
    pub fn show_selected(&mut self, selected_line: usize) {
        self.lines.iter_mut().enumerate().for_each(|(pos, line)| {
            if pos == selected_line {
                line.colour = Color::White
            } else {
                line.colour = Color::Black
            }
        })
    }
    pub fn get_size(&self) -> usize {
        self.lines.len()
    }
    pub fn get_line_sizei_s(&self, line: usize) -> Result<usize, exitfailure::ExitFailure> {
        Ok(self.lines[line].value.len())
    }
    pub fn get_line_size(&self, line: usize) -> usize {
        self.lines[line].value.len()
    }
    pub fn reg_backspace(&mut self, cursor: &mut CursorManager) {
        if cursor.get_cursor().x > 0 {
            self.lines[cursor.get_cursor().y]
                .value
                .remove(cursor.get_cursor().x - 1);
        }
        cursor.cursor_controller.move_left()
    }

    pub fn reg_delete(&mut self, cursor: &mut CursorManager) {
        if cursor.get_cursor().x < self.get_line_size(cursor.get_cursor().y) {
            self.lines[cursor.get_cursor().y]
                .value
                .remove(cursor.get_cursor().x);
        }
    }
    pub fn type_text(&mut self, character: char, cursor: &mut CursorManager) {
        self.lines[cursor.get_cursor().y]
            .value
            .insert(cursor.get_cursor().x, character);
        cursor
            .cursor_controller
            .move_right(self.get_line_size(cursor.get_cursor().y) as i32)
    }
    pub fn add_line(&mut self, cursor: &mut CursorManager) {
        self.lines
            .insert(cursor.get_cursor().y + 1, Line::new("".to_string()));
        cursor.cursor_controller.move_down(self.get_size() as i32);
        cursor.cursor_controller.set_x(0);
    }
}
pub struct LinesManager {
    lines_controller: LinesController,
}
impl LinesManager {
    pub fn new(header: String, content: Vec<String>) -> LinesManager {
        LinesManager {
            lines_controller: LinesController::new(header, content),
        }
    }
    pub fn selected(&mut self, cursor: CursorManager) {
        self.lines_controller.show_selected(cursor.get_cursor().y)
    }

    pub fn get_size(&self) -> usize {
        self.lines_controller.get_size()
    }
    pub fn type_text(&mut self, character: char, cursor: &mut CursorManager) {
        self.lines_controller.type_text(character, cursor);
    }
    pub fn misc_keys(&mut self, key: KeyCode, cursor: &mut CursorManager) {
        match key {
            KeyCode::Backspace => self.lines_controller.reg_backspace(cursor),
            KeyCode::Enter => self.lines_controller.add_line(cursor),
            KeyCode::Delete => self.lines_controller.reg_delete(cursor),
            _ => {}
        }
    }
}
pub struct Reader {}
impl Reader {
    pub fn new() -> Reader {
        Reader {}
    }
    pub async fn read(
        &mut self,
        cur_man: &mut CursorManager,
        line_man: &mut LinesManager,
        supabase: &mut Supabase,
        header_list: &Vec<Headers>,
        table: &str,
    ) -> Result<bool, exitfailure::ExitFailure> {
        if let Ok(Event::Key(event)) = event::read() {
            match event {
                KeyEvent {
                    code: direction @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    cur_man.take_input(direction, &line_man);
                    Ok(true)
                }
                KeyEvent {
                    code: misc @ (KeyCode::Enter | KeyCode::Backspace | KeyCode::Delete),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    line_man.misc_keys(misc, cur_man);
                    Ok(true)
                }
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::ALT,
                    ..
                } => Ok(false),
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let mut content = String::new();
                    let mut count = 0;
                    for line in &line_man.lines_controller.lines {
                        if count < 2 {
                            count += 1;
                        } else {
                            content.push_str(&line.value);
                            content.push_str("\n");
                        }
                    }

                    if header_list.contains(&Headers {
                        header: (&line_man.lines_controller.header).to_string(),
                    }) {
                        supabase
                            .patch_from_header(
                                table,
                                (&line_man.lines_controller.header).to_string(),
                                content,
                            )
                            .await?;
                    } else {
                        supabase
                            .post_text(
                                table,
                                (&line_man.lines_controller.header).to_string(),
                                content,
                            )
                            .await?;
                    }
                    Ok(true)
                }
                KeyEvent {
                    code: KeyCode::Char('b'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    line_man.lines_controller.change_header("Banan".to_string());
                    Ok(true)
                }
                KeyEvent {
                    code: KeyCode::Char(character),
                    ..
                } => {
                    line_man.type_text(character, cur_man);
                    Ok(true)
                }
                _ => Ok(true),
            }
        } else {
            Ok(true)
        }
    }
}
pub struct Renderer {}
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }
    pub fn render(
        &self,
        stdout: &mut Stdout,
        lines: &LinesManager,
        cursor: &CursorManager,
    ) -> Result<(), exitfailure::ExitFailure> {
        queue!(stdout, cursor::MoveTo(0, 0), cursor::Hide,)?;
        lines
            .lines_controller
            .lines
            .iter()
            .for_each(|line| line.draw());
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveTo(cursor.get_cursor().x as u16, cursor.get_cursor().y as u16),
            cursor::Show
        )?;
        stdout.flush()?;
        Ok(())
    }
}

pub struct App {
    renderer: Renderer,
    lines_manager: LinesManager,
    cursor_manager: CursorManager,
    reder: Reader,
    stdout: Stdout,
    supabase: Supabase,
    header_list: Vec<Headers>,
    tables: String,
    state: State,
}
impl App {
    pub async fn new(
        header: String,
        url: String,
        api_key: String,
        tables: String,
        state: State,
    ) -> Result<App, exitfailure::ExitFailure> {
        let supabase = Supabase::new(url, api_key).unwrap();
        let header_list = supabase.get_all_headers(&tables).await?;
        let content: Vec<String> = if header_list.contains(&Headers {
            header: (&header).to_string(),
        }) {
            let resp = supabase.get_from_header(&tables, &header).await?;
            resp[0].content.split("\n").map(|x| x.to_string()).collect()
        } else {
            String::from("")
                .split("\n")
                .map(|x| x.to_string())
                .collect()
        };
        Ok(App {
            renderer: Renderer::new(),
            lines_manager: LinesManager::new(header, content),
            cursor_manager: CursorManager::new(),
            reder: Reader::new(),
            stdout: io::stdout(),
            supabase,
            header_list,
            tables,
            state,
        })
    }
    fn render(self: &mut Self) -> Result<(), exitfailure::ExitFailure> {
        self.renderer
            .render(&mut self.stdout, &self.lines_manager, &self.cursor_manager)?;
        Ok(())
    }
    async fn update(self: &mut Self) -> Result<bool, exitfailure::ExitFailure> {
        self.reder
            .read(
                &mut self.cursor_manager,
                &mut self.lines_manager,
                &mut self.supabase,
                &self.header_list,
                &self.tables,
            )
            .await
    }
    pub async fn run(&mut self) -> Result<bool, exitfailure::ExitFailure> {
        self.render()?;
        self.update().await
    }
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    // #[command(subcommand)]
    // command: Option<Commands>,
    /// Title of the work enviroment.
    #[arg(short, long)]
    f_header: Option<Option<String>>,
}

// #[derive(Subcommand)]
// enum Commands {
//     /// Adds files to myapp
//     Lol,
// }

#[tokio::main]
async fn main() -> Result<(), exitfailure::ExitFailure> {
    let mut title;
    let mut state = State::Setup;
    let args = Cli::parse();

    match args.f_header {
        Some(ttl) => match ttl {
            Some(ttl) => title = ttl,
            None => title = "Fisk".to_string(),
        },
        None => title = "Fisk".to_string(),
    }

    let url = "https://ytuyawdxkrxnugvaptln.supabase.co/rest/v1";
    let api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Inl0dXlhd2R4a3J4bnVndmFwdGxuIiwicm9sZSI6ImFub24iLCJpYXQiOjE2Nzk3NTI0MTYsImV4cCI6MTk5NTMyODQxNn0.V9Q0LBUnxQ_9qaQtTRfOeD9ZM_dvGkK33Gc3N_D_6Uk";
    let mut app = App::new(
        title,
        url.to_string(),
        api_key.to_string(),
        "notes".to_string(),
        state,
    )
    .await?;
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    while app.run().await? {}

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::Show,
        cursor::MoveTo(0, 0)
    )?;

    terminal::disable_raw_mode()?;
    Ok(())
}
