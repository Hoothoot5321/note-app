// let mut stdout = io::stdout();

// let mut lines = vec![Line::new("Hello boY".to_string())];

// let mut cursor_y: usize = 0;

// let mut cursor_x: usize = 0;

// let url = "https://ytuyawdxkrxnugvaptln.supabase.co/rest/v1";
// let api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Inl0dXlhd2R4a3J4bnVndmFwdGxuIiwicm9sZSI6ImFub24iLCJpYXQiOjE2Nzk3NTI0MTYsImV4cCI6MTk5NTMyODQxNn0.V9Q0LBUnxQ_9qaQtTRfOeD9ZM_dvGkK33Gc3N_D_6Uk";

// let sheik = supabase::Supabase::new(url.to_string(), api_key.to_string())?;

// terminal::enable_raw_mode()?;
// queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
// loop {
//     queue!(stdout, cursor::MoveTo(0, 0), cursor::Hide,)?;
//     lines.iter_mut().enumerate().for_each(|(pos, line)| {
//         if pos == cursor_y {
//             line.colour = Color::White
//         } else {
//             line.colour = Color::Black
//         }
//         let _ = &line.draw();
//     });

//     queue!(
//         stdout,
//         style::ResetColor,
//         terminal::Clear(terminal::ClearType::UntilNewLine),
//         cursor::MoveTo(cursor_x as u16, cursor_y as u16),
//         cursor::Show
//     )?;
//     stdout.flush()?;

//     if let Ok(Event::Key(event)) = event::read() {
//         match event {
//             KeyEvent {
//                 code: KeyCode::Char('q'),
//                 modifiers: KeyModifiers::ALT,
//                 ..
//             } => break,
//             KeyEvent {
//                 code: KeyCode::Enter,
//                 ..
//             } => {
//                 cursor_y += 1;
//                 cursor_x = 0;
//                 lines.push(Line::new("".to_string()))
//             }

//             KeyEvent {
//                 code: KeyCode::Delete,
//                 ..
//             } => {
//                 if cursor_x < lines[cursor_y].value.len() {
//                     lines[cursor_y].value.remove(cursor_x);
//                 }
//             }

//             KeyEvent {
//                 code: KeyCode::Backspace,
//                 ..
//             } => {
//                 if cursor_x > 0 {
//                     lines[cursor_y].value.remove(cursor_x - 1);
//                     cursor_x -= 1;
//                 }
//             }
//             KeyEvent {
//                 code: KeyCode::Up, ..
//             } => {
//                 if cursor_y > 0 {
//                     cursor_y = cursor_y - 1
//                 }
//             }
//             KeyEvent {
//                 code: KeyCode::Down,
//                 ..
//             } => {
//                 if cursor_y + 1 < lines.len() {
//                     cursor_y = cursor_y + 1
//                 }
//             }

//             KeyEvent {
//                 code: KeyCode::Char('s'),
//                 modifiers: KeyModifiers::CONTROL,
//                 ..
//             } => {
//                 let mut content = String::new();
//                 for line in &lines {
//                     content.push_str(&line.value);
//                     content.push_str("\n");
//                 }

//                 let status = sheik
//                     .patch_from_header("notes", "Test1".to_string(), content)
//                     .await?;
//                 lines.push(Line::new(status));
//             }

//             KeyEvent {
//                 code: KeyCode::Char(s),
//                 ..
//             } => {
//                 lines[cursor_y].value.insert(cursor_x, s);
//                 cursor_x += 1;
//             }

//             KeyEvent {
//                 code: KeyCode::Left,
//                 ..
//             } => {
//                 if cursor_x > 0 {
//                     cursor_x = cursor_x - 1
//                 }
//             }
//             KeyEvent {
//                 code: KeyCode::Right,
//                 ..
//             } => {
//                 if cursor_x < lines[cursor_y].value.len() {
//                     cursor_x = cursor_x + 1
//                 }
//             }

//             _ => {}
//         }
//     }
// }
