use clap::Parser;
use figlet_rs::FIGfont;
#[derive(Debug, Parser)]
#[command(name = "mal", version, about = "A TUI client for myanimelist.net", long_about = None)]
struct Cli {
    /// Show extra information about the tool
    #[arg(short = 'i', long = "info", action = clap::ArgAction::SetTrue)]
    info: bool,
    /// Show configuration file structure and all available options
    #[arg(short = 'c', long = "config", action = clap::ArgAction::SetTrue)]
    config: bool,
}

pub fn handle_args() -> bool {
    let cli = Cli::parse();

    if cli.info {
        print_info();
        return true;
    } else if cli.config {
        print_config_structure();
        return true;
    }
    false
}

fn print_info() {
    let standard_font = FIGfont::standard().unwrap();
    let figlet = standard_font.convert("mal-cli");
    let fig_string = figlet.unwrap().to_string();
    println!(
        r#"
{fig_string}

FILES:
    - OAuth2 tokens:       $HOME/.config/mal-cli/oauth2.yml
    - Cache data:          $HOME/.cache/mal-cli/
    - Configuration file:  $HOME/.config/mal-cli/config.yml

NOTE:
    - Use GPU-enhanced terminals, otherwise the images won't be rendered
    - The configuration file is optional. If it does not exist, the application will create a default one.
    - The cache directory is used to store images
"#
    );
}

fn print_config_structure() {
    println!(
        r#"

CONFIGURATION KEYS:
===================

KEYBINDINGS:
  keys:
    help: '?'                    # Show help menu
    back: 'q'                    # Go back/quit current view
    search: '/'                  # Open search
    toggle: 's'                  # Toggle between anime/manga or switch states
    next_state: Ctrl+p           # Navigate to next state/page
    open_popup: 'r'              # Open rating/status popup

THEME COLORS:
  theme:
    mal_color: '#2E51A2'         # MyAnimeList brand color (blue)
    active: Cyan                 # Color for active/focused elements
    banner: LightCyan            # Color for banners and titles
    hovered: Magenta             # Color for hovered elements
    text: White                  # Default text color
    selected: LightCyan          # Color for selected items
    error_border: Red            # Border color for error dialogs
    error_text: LightRed         # Text color for error messages
    inactive: Gray               # Color for inactive/unfocused elements
    status_completed: Green      # Color for completed anime/manga
    status_dropped: Gray         # Color for dropped items
    status_on_hold: Yellow       # Color for on-hold items
    status_watching: Blue        # Color for currently watching/reading
    status_plan_to_watch: Cyan   # Color for plan-to-watch/read items
    status_other: White          # Color for other status types

APPLICATION BEHAVIOR:
  behavior:
    tick_rate_milliseconds: 500  # UI refresh rate (lower = more responsive)
    show_logger: false           # Show debug logger window

CONTENT SETTINGS:
  nsfw: false                    # Show NSFW (18+) content
  title_language: English       # Preferred title language (English/Japanese)
  manga_display_type: Both      # Show volumes, chapters, or both (Vol/Ch/Both)

RANKING TYPES:
  top_three_anime_types:         # Anime ranking types to show in top 3
    - airing                     # Currently airing anime
    - all                        # All-time rankings
    - upcoming                   # Upcoming anime
    - movie                      # Movie rankings
    - special                    # Special episodes
    - ova                        # Original Video Animation
    - tv                         # TV series
    - popularity                 # Most popular
    - favorite                   # Most favorited

  top_three_manga_types:         # Manga ranking types to show in top 3
    - all                        # All-time rankings
    - manga                      # Manga only
    - novels                     # Light novels
    - oneshots                   # One-shot manga
    - doujinshi                  # Self-published works
    - manhwa                     # Korean comics
    - manhua                     # Chinese comics
    - bypopularity               # Most popular
    - favorite                   # Most favorited

PERFORMANCE SETTINGS:
  navigation_stack_limit: 15     # Max number of pages to keep in history
  search_limit: 30               # Max search results per page
  max_cached_images: 15          # Max images to cache for faster loading

EXAMPLE CONFIG FILE:
====================
Copy the example configuration from: config.example.yml
Location: $HOME/.config/mal-cli/config.yml
"#
    );
}
