use color_eyre::Result;
use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal;
use crossterm::{cursor::MoveTo, ExecutableCommand};
use mal::api::model::RankingType;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use std::sync::Arc;
use std::{
    io::{self}, //Write
    panic,
};
use tokio::sync::Mutex;

use mal::app::*;
use mal::auth::OAuth;
// use mal::cli::{Opt, StructOpt};
use mal::config::{app_config::AppConfig, oauth_config::AuthConfig};
use mal::event;
use mal::event::key::Key;
use mal::handlers;
use mal::network::{IoEvent, Network};
use mal::ui;

fn setup_terminal() -> Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    execute!(stdout, crossterm::event::EnableMouseCapture)?;

    terminal::enable_raw_mode()?;
    Ok(())
}

fn cleanup_terminal() -> Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, crossterm::event::DisableMouseCapture)?;

    execute!(stdout, cursor::MoveTo(0, 0))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    execute!(stdout, cursor::Show)?;

    terminal::disable_raw_mode()?;
    Ok(())
}

/// Makes sure that the terminal cleans up even when there's a panic
fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        cleanup_terminal().unwrap();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));
}

#[tokio::main]
async fn main() -> Result<()> {
    better_panic::install();
    setup_panic_hook();

    // let opt: Opt = Opt::from_args();

    // Get config
    let app_config = AppConfig::load()?;

    let auth_config = AuthConfig::load()?;
    let oauth = OAuth::get_auth_async(auth_config).await?;

    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

    // initialize app state
    let app = Arc::new(Mutex::new(App::new(sync_io_tx, app_config.clone())));

    let cloned_app = Arc::clone(&app);
    std::thread::spawn(move || {
        let mut network = Network::new(oauth, &app);
        start_network(sync_io_rx, &mut network);
    });

    // run ui
    start_ui(app_config, &cloned_app).await?;

    Ok(())
}

#[tokio::main]
async fn start_network<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}

async fn start_ui(app_config: AppConfig, app: &Arc<Mutex<App>>) -> Result<()> {
    // set up terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    setup_terminal()?;

    let events = event::Events::new(app_config.behavior.tick_rate_milliseconds);
    {
        let mut app = app.lock().await;
        app.active_top_three = TopThreeBlock::Loading(RankingType::AnimeRankingType(
            app_config.top_three_anime_types[0].clone(),
        ));
        app.active_top_three_anime = Some(app_config.top_three_anime_types[0].clone());

        app.active_top_three_manga = Some(app_config.top_three_manga_types[0].clone());

        app.dispatch(IoEvent::GetTopThree(TopThreeBlock::Anime(
            app_config.top_three_anime_types[0].clone(),
        )));
    }
    // let mut is_first_render = true;

    loop {
        let mut app = app.lock().await;

        let current_block = app.active_block;
        terminal.draw(|mut f| match current_block {
            // todo: handle help inside the main_layout
            // ActiveBlock::Help => {
            //     ui::draw_help_menu(&mut f, &app);
            // }
            //todo: handle error inside the display block
            // ActiveBlock::Error => {
            //     ui::draw_error(&mut f, &app);
            // }
            _ => {
                ui::draw_main_layout(&mut f, &app);
            }
        })?;

        if current_block == ActiveBlock::Input {
            terminal.show_cursor()?;
        } else {
            terminal.hide_cursor()?;
        }

        let cursor_offset = if app.size.height > ui::util::SMALL_TERMINAL_HEIGHT {
            2
        } else {
            1
        };

        terminal.backend_mut().execute(MoveTo(
            cursor_offset + app.input_cursor_position,
            cursor_offset,
        ))?;

        /*
        there are five blocks:
            1.Input
            2.AnimeMenu
            3.MangaMenu
            4.UserMenu
            5.DisplayBlock

        and there are different display blocks :
            1.SearchResultBlock
            2.Help
            3.UserInfo
            4.UserAnimeList,
            5.UserMangaList
            6.Suggestions
            7.Seasonal
            8.AnimeRanking
            9.MangaRanking
            10.Loading
            11.Error
            12.Empty

        we switch between blocks by pressing Tab and between display by input and navigation
        we will implement a stack for display block to allow going back and forth
                */
        match events.next()? {
            event::Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    //todo: display confirmation to  quit
                    break;
                }

                let active_block = app.active_block;
                //# change the default of menu selecting to None when leaving the block
                if key == Key::Tab {
                    // handle navigation between block
                    handlers::handle_tab(&mut app);
                } else if active_block == ActiveBlock::Input {
                    handlers::input_handler(key, &mut app);
                } else if key == app.app_config.keys.back {
                    if app.active_block != ActiveBlock::Input {
                        app.load_previous_route();
                        break;
                    }
                } else {
                    handlers::handle_app(key, &mut app);
                }
            }
            _ => {}
        }
    }

    // clean up terminal
    cleanup_terminal()?;
    Ok(())
}
