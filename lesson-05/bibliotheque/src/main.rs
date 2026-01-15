mod app;
mod models;
mod services;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use services::Bibliotheque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, panic};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup panic hook pour restaurer le terminal en cas de panic
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Création de la bibliothèque partagée
    // Arc<Mutex<T>> permet le partage thread-safe:
    // - Arc (Atomic Reference Counted) = smart pointer avec compteur de références
    // - Mutex = garantit l'accès exclusif lors des modifications
    let biblio = Arc::new(Mutex::new(Bibliotheque::new()));

    // Créer l'application
    let mut app = App::new(biblio);

    // Event loop
    let res = run_app(&mut terminal, &mut app);

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| app.render(f))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_input(key)?;
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
