use crate::api::{models::BFileEntry, Cache, OEISClient, OEISResponse, SearchQuery, Sequence, UserSettings};
use crate::i18n::{I18n, Language};
use crate::ui::{
    self,
    animation::{WelcomeAnimation, WelcomeAnimationMode},
    Theme,
};
use crate::utils::{keybindings::KeyBindings, parse_search_terms};
use anyhow::Result;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use ratatui::{layout::Rect, widgets::Block, Frame};
use std::time::{Duration, Instant};
use tokio::task::{JoinError, JoinHandle};

const DOUBLE_CLICK_THRESHOLD_MS: u64 = 400;
const SCROLL_LINES: u16 = 3;
const PREVIEW_SCROLL_LINES: u16 = 3;
const DETAIL_TAB_COUNT: usize = 8;
const PREVIEW_TAB_COUNT: usize = 6;
pub const PLACEHOLDER_EXAMPLES: &[&str] = &[
    "1,2,3,4",
    "id:A000045",
    "keyword:prime",
    "fibonacci",
    "keyword:nice",
    "id:A001203",
];

/// Application screens/states
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    /// Main search interface
    Search,
    /// Sequence detail view
    Detail,
    /// Graph visualization
    Graph,
    /// Webcam mode (auto-refreshing sequences)
    Webcam,
    /// Settings screen
    Settings,
}

/// Input mode for text entry
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Focus state for search screen panels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchFocus {
    Input,
    Results,
    History,
    Bookmarks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebcamFocus {
    Categories,
    Intervals,
    Sequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsFocus {
    Language,
    Theme,
    Animation,
}

/// Main application state
pub struct App {
    /// Current screen
    pub current_screen: Screen,
    /// Should the application quit
    pub should_quit: bool,
    /// OEIS API client
    pub client: OEISClient,
    /// Local cache
    pub cache: Cache,
    /// Internationalization
    pub i18n: I18n,

    // Search state
    /// Current search query input
    pub search_input: String,
    /// Cursor position in search input
    pub search_cursor: usize,
    /// Input mode (normal or editing)
    pub input_mode: InputMode,
    /// Focus state for search screen panels
    pub search_focus: SearchFocus,
    /// Search results
    pub search_results: Vec<Sequence>,
    /// Total result count from OEIS
    pub result_count: i32,
    /// Current search query (for pagination)
    pub current_query: Option<SearchQuery>,
    /// Terms extracted from the current search (for highlighting)
    pub search_terms: Vec<String>,
    /// Selected result index
    pub selected_result: usize,
    /// Cached search input area for mouse interactions
    pub search_input_area: Option<Rect>,
    /// Cached results table area for mouse interactions
    pub search_results_area: Option<Rect>,
    /// Cached preview area for mouse interactions
    pub search_preview_area: Option<Rect>,
    /// Cached preview tabs area for mouse interactions
    pub search_preview_tabs_area: Option<Rect>,
    /// Search in progress
    pub searching: bool,
    /// Show welcome modal overlay
    pub show_welcome_modal: bool,
    /// Has the welcome modal been dismissed before
    pub welcome_dismissed: bool,
    /// Help modal visibility
    pub help_modal_visible: bool,
    /// About modal visibility
    pub about_modal_visible: bool,
    /// Last search time in seconds
    pub last_search_time: Option<f64>,
    /// Results per page (configurable)
    pub results_per_page: usize,
    /// Show preview pane on search screen
    pub show_preview: bool,
    /// Scroll position in preview pane
    pub preview_scroll: u16,
    /// Active tab in preview pane
    pub preview_tab: usize,
    /// Placeholder rotation timer
    pub placeholder_timer: Instant,
    /// Current placeholder index
    pub placeholder_index: usize,
    /// Recently viewed sequences for history panel
    pub recent_sequences: Vec<(i32, String, i32, String)>,
    /// Selected index in history panel
    pub history_selected: usize,
    /// Cached history panel area for mouse interactions
    pub history_area: Option<Rect>,

    // Bookmarks state
    /// Bookmarked sequences [(number, notes)]
    pub bookmarks: Vec<(i32, Option<String>)>,
    /// Selected index in bookmarks panel
    pub bookmarks_selected: usize,
    /// Cached bookmarks panel area for mouse interactions
    pub bookmarks_area: Option<Rect>,
    /// Show bookmarks panel (true) or history panel (false)
    pub show_bookmarks: bool,

    // Detail view state
    /// Currently viewed sequence
    pub current_sequence: Option<Sequence>,
    /// Active tab in detail view
    pub detail_tab: usize,
    /// Scroll position in detail view
    pub detail_scroll: u16,
    /// Known references (A-numbers) in the current detail pane
    pub detail_references: Vec<String>,
    /// Currently focused reference index
    pub detail_reference_index: Option<usize>,
    /// Cached detail tabs area for mouse interactions
    pub detail_tabs_area: Option<Rect>,
    /// Cached detail content area for mouse interactions
    pub detail_content_area: Option<Rect>,

    // Graph state
    /// Graph type for visualization
    pub graph_type: GraphType,

    // Webcam state
    /// Webcam refresh interval in seconds
    pub webcam_interval: Option<WebcamInterval>,
    /// Last webcam update time
    pub webcam_last_update: Option<std::time::Instant>,
    /// Webcam category
    pub webcam_category: usize,
    /// Focused region in webcam mode
    pub webcam_focus: WebcamFocus,
    /// Scroll offset for webcam category list
    pub webcam_category_scroll: u16,
    /// Scroll offset for webcam interval list
    pub webcam_interval_scroll: u16,

    // Export state
    /// Export format
    pub export_format: ExportFormat,

    // Help state
    /// Scroll position in help screen
    pub help_scroll: u16,
    /// Detail view help modal visibility
    pub detail_help_visible: bool,
    /// Scroll position in detail help modal
    pub detail_help_scroll: u16,

    // Settings state
    /// Selected language index in settings
    pub settings_selected_language: usize,
    /// Selected theme index in settings
    pub settings_selected_theme: usize,
    /// Selected animation mode index in settings
    pub settings_selected_animation: usize,
    /// Which settings section currently has focus
    pub settings_focus: SettingsFocus,
    /// Scroll offset for language settings list
    pub settings_language_scroll: u16,
    /// Scroll offset for theme settings list
    pub settings_theme_scroll: u16,
    /// Scroll offset for animation settings list
    pub settings_animation_scroll: u16,

    // Theme state
    pub themes: Vec<Theme>,
    pub active_theme: usize,
    /// Current welcome animation mode
    pub welcome_animation_mode: WelcomeAnimationMode,
    /// User-selected welcome animation mode (for persistence)
    welcome_animation_user_pref: WelcomeAnimationMode,
    /// Whether we temporarily enable animation at startup when the preference is Off
    welcome_animation_temp_active: bool,
    /// Whether the welcome animation has already been shown once (for suppressing future auto-play)
    welcome_animation_played: bool,
    /// Persisted settings on disk
    settings_store: UserSettings,
    /// Runtime keybindings (configurable via settings)
    keybindings: KeyBindings,
    /// Current keybindings preset (e.g., "vim")
    keybindings_preset: Option<String>,

    // Animation state
    /// Spinner animation frame counter
    pub spinner_frame: u8,
    /// Last time the spinner was advanced
    spinner_last_tick: Instant,
    /// Animated welcome background effect
    welcome_animation: WelcomeAnimation,
    /// Active background search task (if any)
    pending_search: Option<PendingSearch>,
    /// Active background random sequence task (if any)
    pending_random: Option<PendingRandom>,
    /// Active background B-file fetch task (if any)
    pub pending_bfile: Option<PendingBFile>,
    /// B-file data for current sequence
    pub bfile_data: Option<Vec<BFileEntry>>,
    /// B-file error message
    pub bfile_error: Option<String>,
    /// Timestamp/index of the last results click (for double-click detection)
    last_result_click: Option<(Instant, usize)>,

    // Error state
    /// Current error message (if any)
    pub error_message: Option<String>,
}

/// Graph visualization types
#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    Line,
    Scatter,
    LogScatter,
    PinPlot,
}

/// Webcam refresh intervals
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WebcamInterval {
    Manual,
    FiveSeconds,
    TenSeconds,
    TwentySeconds,
    ThirtySeconds,
    OneMinute,
}

impl WebcamInterval {
    pub fn as_duration(&self) -> Option<Duration> {
        match self {
            Self::Manual => None,
            Self::FiveSeconds => Some(Duration::from_secs(5)),
            Self::TenSeconds => Some(Duration::from_secs(10)),
            Self::TwentySeconds => Some(Duration::from_secs(20)),
            Self::ThirtySeconds => Some(Duration::from_secs(30)),
            Self::OneMinute => Some(Duration::from_secs(60)),
        }
    }
}

/// Export formats
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
    Txt,
    Markdown,
    BFile,
}

impl ExportFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Json => "JSON",
            Self::Csv => "CSV",
            Self::Txt => "TXT",
            Self::Markdown => "Markdown",
            Self::BFile => "B-file",
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Txt => "txt",
            Self::Markdown => "md",
            Self::BFile => "txt",
        }
    }
}

/// Different search flows that run in the background
enum PendingSearchKind {
    /// Fresh search from user input
    Initial,
    /// Advance to next result page
    NextPage,
    /// Return to previous result page
    PreviousPage,
}

/// Metadata tracked for in-flight search jobs
struct PendingSearch {
    handle: JoinHandle<anyhow::Result<OEISResponse>>,
    query: SearchQuery,
    kind: PendingSearchKind,
    started_at: Instant,
}

struct PendingRandom {
    handle: JoinHandle<anyhow::Result<Option<Sequence>>>,
    started_at: Instant,
}

pub struct PendingBFile {
    handle: JoinHandle<anyhow::Result<Vec<BFileEntry>>>,
    started_at: Instant,
}

impl App {
    /// Create a new application instance
    pub async fn new() -> Result<Self> {
        let client = OEISClient::new()?;
        let cache = Cache::new()?;
        let mut i18n = I18n::new();
        let mut settings_store = UserSettings::load();

        // Load recent sequences before moving cache into struct
        let recent_sequences = cache
            .get_recently_viewed_with_details(8)
            .unwrap_or_default();

        // Load bookmarks before moving cache into struct
        let bookmarks = cache.get_bookmarks().unwrap_or_default();

        let themes = vec![
            Theme::dark(),
            Theme::light(),
            Theme::dracula(),
            Theme::nord(),
            Theme::gruvbox(),
            Theme::solarized_dark(),
            Theme::solarized_light(),
            Theme::monokai(),
            Theme::catppuccin_mocha(),
            Theme::one_dark(),
            Theme::night_owl(),
            Theme::phosphor_night(),
            Theme::punchcard_light(),
            Theme::terminal_trove(),
        ];
        let active_theme = settings_store
            .theme
            .unwrap_or(0)
            .min(themes.len().saturating_sub(1));
        let welcome_animation_user_pref = settings_store
            .welcome_animation_mode()
            .unwrap_or(WelcomeAnimationMode::Off);
        let mut welcome_animation_played = settings_store.welcome_animation_played.unwrap_or(false);
        // On first run with animation pref Off and not yet played, temporarily show it on the welcome screen only.
        let mut welcome_animation_mode = welcome_animation_user_pref;
        let welcome_animation_temp_active =
            matches!(welcome_animation_user_pref, WelcomeAnimationMode::Off)
                && !welcome_animation_played;
        if welcome_animation_temp_active {
            welcome_animation_mode = WelcomeAnimationMode::Normal;
            welcome_animation_played = true;
        }
        let welcome_animation =
            WelcomeAnimation::new(&themes[active_theme], welcome_animation_mode);
        let settings_selected_animation = Self::animation_mode_index(welcome_animation_user_pref);
        if welcome_animation_temp_active {
            // Persist that we've shown it once and that the preference is Off.
            settings_store.welcome_animation_played = Some(true);
            settings_store.welcome_animation = Some(WelcomeAnimationMode::Off.key().to_string());
        }
        let keybindings = settings_store.keybindings();
        let mut settings_selected_language = 0;
        if let Some(lang) = settings_store.language() {
            i18n.set_language(lang);
            settings_selected_language =
                Language::all().iter().position(|l| *l == lang).unwrap_or(0);
        }
        let settings_selected_theme = active_theme;
        let keybindings_preset = settings_store.keybindings_preset.clone();

        let mut app = Self {
            current_screen: Screen::Search,
            should_quit: false,
            client,
            cache,
            i18n,
            search_input: String::new(),
            search_cursor: 0,
            input_mode: InputMode::Editing,
            search_focus: SearchFocus::Input,
            search_results: Vec::new(),
            result_count: 0,
            current_query: None,
            search_terms: Vec::new(),
            selected_result: 0,
            search_input_area: None,
            search_results_area: None,
            search_preview_area: None,
            search_preview_tabs_area: None,
            searching: false,
            show_welcome_modal: true,
            welcome_dismissed: false,
            help_modal_visible: false,
            about_modal_visible: false,
            last_search_time: None,
            results_per_page: 15,
            show_preview: false,
            preview_scroll: 0,
            preview_tab: 0,
            placeholder_timer: Instant::now(),
            placeholder_index: 0,
            recent_sequences,
            history_selected: 0,
            history_area: None,
            bookmarks,
            bookmarks_selected: 0,
            bookmarks_area: None,
            show_bookmarks: false,
            current_sequence: None,
            detail_tab: 0,
            detail_scroll: 0,
            detail_references: Vec::new(),
            detail_reference_index: None,
            detail_tabs_area: None,
            detail_content_area: None,
            graph_type: GraphType::Line,
            webcam_interval: None,
            webcam_last_update: None,
            webcam_category: 0,
            webcam_focus: WebcamFocus::Categories,
            webcam_category_scroll: 0,
            webcam_interval_scroll: 0,
            export_format: ExportFormat::Json,
            help_scroll: 0,
            detail_help_visible: false,
            detail_help_scroll: 0,
            settings_selected_language,
            settings_selected_theme,
            settings_selected_animation,
            settings_focus: SettingsFocus::Language,
            settings_language_scroll: 0,
            settings_theme_scroll: 0,
            settings_animation_scroll: 0,
            themes,
            active_theme,
            welcome_animation_mode,
            welcome_animation_user_pref,
            welcome_animation_temp_active,
            welcome_animation_played,
            settings_store,
            keybindings,
            keybindings_preset,
            spinner_frame: 0,
            spinner_last_tick: Instant::now(),
            welcome_animation,
            pending_search: None,
            pending_random: None,
            pending_bfile: None,
            bfile_data: None,
            bfile_error: None,
            last_result_click: None,
            error_message: None,
        };

        // Persist initial settings on first landing so the config file is created immediately.
        app.persist_startup_settings();

        Ok(app)
    }

    /// Render the current screen
    pub fn render(&mut self, f: &mut Frame) {
        // Render background first
        let background = Block::new().style(self.theme().background());
        f.render_widget(background, f.area());

        if self.current_screen == Screen::Search
            && self.show_welcome_modal
            && !matches!(self.welcome_animation_mode, WelcomeAnimationMode::Off)
        {
            let modal_area = ui::search::welcome_modal_area(f.area());
            self.welcome_animation.render(f, f.area(), modal_area);
        }

        match self.current_screen {
            Screen::Search => ui::search::render(f, self),
            Screen::Detail => ui::detail::render(f, self),
            Screen::Graph => ui::graph::render(f, self),
            Screen::Webcam => ui::webcam::render(f, self),
            Screen::Settings => ui::settings::render(f, self),
        }

        if self.help_modal_visible {
            ui::help::render_modal(f, self);
        }

        if self.about_modal_visible {
            ui::about::render_modal(f, self);
        }
    }

    /// Handle terminal events
    pub async fn handle_events(&mut self) -> Result<()> {
        while event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key) => self.handle_key_event(key).await?,
                Event::Mouse(mouse) => self.handle_mouse_event(mouse).await?,
                Event::Resize(_, _) => {}
                _ => {}
            }
        }

        self.poll_pending_search().await?;
        self.poll_pending_random().await?;
        self.poll_pending_bfile().await?;
        self.enforce_welcome_animation_timeout();

        // Update spinner / placeholder timers
        if self.should_spin_spinner() {
            if self.spinner_last_tick.elapsed() >= Duration::from_millis(150) {
                self.spinner_frame = self.spinner_frame.wrapping_add(1);
                self.spinner_last_tick = Instant::now();
            }
        }

        if !self.searching
            && self.search_input.is_empty()
            && self.placeholder_timer.elapsed() > Duration::from_secs(4)
        {
            self.placeholder_index = (self.placeholder_index + 1) % PLACEHOLDER_EXAMPLES.len();
            self.placeholder_timer = Instant::now();
        }

        // Handle webcam auto-refresh
        if self.current_screen == Screen::Webcam {
            if let Some(interval) = self.webcam_interval {
                if let Some(duration) = interval.as_duration() {
                    if let Some(last_update) = self.webcam_last_update {
                        if last_update.elapsed() >= duration {
                            self.webcam_next_sequence().await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn theme(&self) -> &Theme {
        &self.themes[self.active_theme]
    }

    /// Whether we should draw at a fast cadence (animations/spinners active).
    pub fn wants_fast_tick(&self) -> bool {
        let animating_welcome = self.current_screen == Screen::Search
            && self.show_welcome_modal
            && !matches!(self.welcome_animation_mode, WelcomeAnimationMode::Off);

        animating_welcome || self.searching
    }

    fn set_active_theme(&mut self, index: usize) {
        if index >= self.themes.len() {
            return;
        }
        self.active_theme = index;
        self.settings_selected_theme = index;
        self.settings_store.theme = Some(index);
        self.persist_settings();
        self.welcome_animation.update_palette(self.theme());
    }

    fn set_language(&mut self, lang: Language) {
        self.i18n.set_language(lang);
        self.settings_selected_language =
            Language::all().iter().position(|l| *l == lang).unwrap_or(0);
        self.settings_store.language = Some(lang.code().to_string());
        self.persist_settings();
    }

    fn animation_mode_index(mode: WelcomeAnimationMode) -> usize {
        WelcomeAnimationMode::modes()
            .iter()
            .position(|m| *m == mode)
            .unwrap_or(0)
    }

    fn animation_mode_from_index(index: usize) -> WelcomeAnimationMode {
        WelcomeAnimationMode::modes()
            .get(index)
            .copied()
            .unwrap_or(WelcomeAnimationMode::default())
    }

    fn set_welcome_animation_mode(&mut self, mode: WelcomeAnimationMode) {
        self.welcome_animation_user_pref = mode;
        self.welcome_animation_mode = mode;
        self.welcome_animation.set_mode(mode);
        self.welcome_animation_temp_active = false;
        self.welcome_animation_played = true;
        self.settings_store.welcome_animation_played = Some(true);
        self.settings_selected_animation = Self::animation_mode_index(mode);
        self.settings_store.welcome_animation = Some(mode.key().to_string());
        self.persist_settings();
    }

    fn cycle_theme(&mut self) {
        if self.themes.is_empty() {
            return;
        }
        let next = (self.active_theme + 1) % self.themes.len();
        self.set_active_theme(next);
    }

    fn reload_keybindings(&mut self) {
        self.keybindings = KeyBindings::from_config(
            self.settings_store.keybindings.as_ref(),
            self.keybindings_preset.as_deref(),
        );
    }

    fn set_keybindings_preset(&mut self, preset: Option<&str>) {
        self.keybindings_preset = preset.map(|s| s.to_string());
        self.settings_store.keybindings_preset = self.keybindings_preset.clone();
        self.reload_keybindings();
        self.persist_settings();
    }

    fn toggle_keybindings_preset(&mut self) {
        match self.keybindings_preset.as_deref() {
            Some("vim") => self.set_keybindings_preset(None),
            _ => self.set_keybindings_preset(Some("vim")),
        }
    }

    fn close_welcome_modal(&mut self) {
        self.show_welcome_modal = false;
        self.welcome_dismissed = true;
        self.input_mode = InputMode::Normal;
    }

    fn enforce_welcome_animation_timeout(&mut self) {
        if self.welcome_animation_temp_active && self.current_screen != Screen::Search {
            self.welcome_animation_temp_active = false;
            self.welcome_animation_played = true;
            self.settings_store.welcome_animation_played = Some(true);
            self.welcome_animation_mode = self.welcome_animation_user_pref;
            self.welcome_animation
                .set_mode(self.welcome_animation_user_pref);
            self.persist_settings();
        }
    }

    fn persist_settings(&mut self) {
        let _ = self.settings_store.save();
    }

    fn persist_startup_settings(&mut self) {
        self.settings_store.theme = Some(self.active_theme);
        self.settings_store.welcome_animation =
            Some(self.welcome_animation_user_pref.key().to_string());
        self.settings_store.welcome_animation_played = Some(self.welcome_animation_played);
        self.settings_store.language = Some(self.i18n.get_current_language().code().to_string());
        self.settings_store.keybindings_preset = self.keybindings_preset.clone();
        let _ = self.settings_store.save();
    }

    async fn poll_pending_search(&mut self) -> Result<()> {
        if let Some(pending) = self.pending_search.as_ref() {
            if pending.handle.is_finished() {
                let PendingSearch {
                    handle,
                    query,
                    kind,
                    started_at,
                } = self
                    .pending_search
                    .take()
                    .expect("pending search should still be present");

                match handle.await {
                    Ok(Ok(response)) => {
                        self.complete_search_success(query, kind, started_at, response)
                    }
                    Ok(Err(error)) => self.complete_search_failure(kind, error),
                    Err(join_error) => self.complete_search_join_error(kind, join_error),
                }
            }
        }

        Ok(())
    }

    fn cancel_pending_search(&mut self) {
        if let Some(pending) = self.pending_search.take() {
            pending.handle.abort();
        }
        self.searching = false;
    }

    fn spawn_search(&mut self, query: SearchQuery, kind: PendingSearchKind) {
        self.cancel_pending_search();

        if matches!(
            kind,
            PendingSearchKind::NextPage | PendingSearchKind::PreviousPage
        ) {
            self.current_query = Some(query.clone());
        }

        self.error_message = None;

        let client = self.client.clone();
        let page_size = self.results_per_page;
        let query_for_task = query.clone();

        self.searching = true;
        self.spinner_frame = 0;

        let handle = tokio::spawn(async move { client.search(&query_for_task, page_size).await });

        self.pending_search = Some(PendingSearch {
            handle,
            query,
            kind,
            started_at: Instant::now(),
        });
    }

    fn complete_search_success(
        &mut self,
        query: SearchQuery,
        kind: PendingSearchKind,
        started_at: Instant,
        response: OEISResponse,
    ) {
        self.searching = false;

        let cache_response = response.clone();
        let count = response.count;
        let results = response.results.unwrap_or_default();

        self.search_results = results;
        self.selected_result = 0;
        self.last_search_time = Some(started_at.elapsed().as_secs_f64());
        self.error_message = None;
        self.last_result_click = None;

        match kind {
            PendingSearchKind::Initial => {
                self.result_count = count;
                self.current_query = Some(query.clone());
                let _ = self.cache.cache_search(&query.query, &cache_response);
                let _ = self.cache.add_search_history(&query.query);
            }
            PendingSearchKind::NextPage => {
                self.current_query = Some(query.clone());
            }
            PendingSearchKind::PreviousPage => {
                self.current_query = Some(query.clone());
            }
        }

        if self.show_preview {
            self.update_preview_if_enabled();
        }
    }

    fn complete_search_failure(&mut self, kind: PendingSearchKind, error: anyhow::Error) {
        self.searching = false;

        let message = match kind {
            PendingSearchKind::Initial => format!("Search failed: {}", error),
            PendingSearchKind::NextPage => format!("Failed to load next page: {}", error),
            PendingSearchKind::PreviousPage => format!("Failed to load previous page: {}", error),
        };

        self.error_message = Some(message);
    }

    fn complete_search_join_error(&mut self, kind: PendingSearchKind, error: JoinError) {
        self.searching = false;

        if error.is_panic() {
            let message = match kind {
                PendingSearchKind::Initial => "Search task panicked".to_string(),
                PendingSearchKind::NextPage => "Next page task panicked".to_string(),
                PendingSearchKind::PreviousPage => "Previous page task panicked".to_string(),
            };
            self.error_message = Some(message);
        }
    }

    async fn poll_pending_random(&mut self) -> Result<()> {
        if let Some(pending) = self.pending_random.as_ref() {
            if pending.handle.is_finished() {
                let PendingRandom { handle, started_at } = self
                    .pending_random
                    .take()
                    .expect("pending random should still be present");

                match handle.await {
                    Ok(Ok(Some(sequence))) => {
                        self.complete_random_success(started_at, sequence);
                    }
                    Ok(Ok(None)) => {
                        self.complete_random_none();
                    }
                    Ok(Err(error)) => {
                        self.complete_random_failure(error);
                    }
                    Err(join_error) => {
                        self.complete_random_join_error(join_error);
                    }
                }
            }
        }

        Ok(())
    }

    fn start_random_sequence(&mut self) -> Result<()> {
        self.cancel_pending_random();
        self.searching = true;
        self.error_message = None;
        self.spinner_frame = 0;
        self.last_result_click = None;

        let client = self.client.clone();
        let handle = tokio::spawn(async move { client.random_sequence().await });
        self.pending_random = Some(PendingRandom {
            handle,
            started_at: Instant::now(),
        });

        Ok(())
    }

    fn cancel_pending_random(&mut self) {
        if let Some(pending) = self.pending_random.take() {
            pending.handle.abort();
        }
        self.searching = false;
    }

    fn complete_random_success(&mut self, started_at: Instant, sequence: Sequence) {
        self.searching = false;
        self.error_message = None;
        self.last_search_time = Some(started_at.elapsed().as_secs_f64());

        let cached = sequence.clone();
        self.current_sequence = Some(sequence);
        self.current_screen = Screen::Detail;
        self.detail_tab = 0;
        self.detail_scroll = 0;
        self.reset_detail_reference_state();
        self.last_result_click = None;

        let _ = self.cache.record_view(cached.number);
        let _ = self.cache.cache_sequence(&cached);
    }

    fn complete_random_none(&mut self) {
        self.searching = false;
        self.error_message = Some("No random sequence found".to_string());
    }

    fn complete_random_failure(&mut self, error: anyhow::Error) {
        self.searching = false;
        self.error_message = Some(format!("Failed to load random sequence: {}", error));
    }

    fn complete_random_join_error(&mut self, error: JoinError) {
        self.searching = false;
        if error.is_panic() {
            self.error_message = Some("Random sequence task panicked".to_string());
        }
    }

    pub fn set_detail_references(&mut self, references: Vec<String>) {
        self.detail_references = references;
        self.ensure_detail_reference_index();
    }

    fn reset_detail_reference_state(&mut self) {
        self.detail_references.clear();
        self.detail_reference_index = None;
    }

    fn ensure_detail_reference_index(&mut self) {
        if self.detail_references.is_empty() {
            self.detail_reference_index = None;
            return;
        }

        match self.detail_reference_index {
            Some(idx) if idx < self.detail_references.len() => {}
            _ => self.detail_reference_index = Some(0),
        }
    }

    fn select_next_detail_reference(&mut self) {
        if self.detail_references.is_empty() {
            return;
        }

        let next = match self.detail_reference_index {
            Some(idx) => (idx + 1) % self.detail_references.len(),
            None => 0,
        };
        self.detail_reference_index = Some(next);
    }

    fn select_previous_detail_reference(&mut self) {
        if self.detail_references.is_empty() {
            return;
        }

        let len = self.detail_references.len();
        let prev = match self.detail_reference_index {
            Some(0) | None => len - 1,
            Some(idx) => (idx + len - 1) % len,
        };
        self.detail_reference_index = Some(prev);
    }

    fn should_spin_spinner(&self) -> bool {
        if self.searching {
            return true;
        }

        if self.current_screen == Screen::Webcam {
            if let (Some(interval), Some(last_update)) =
                (self.webcam_interval, self.webcam_last_update)
            {
                if let Some(duration) = interval.as_duration() {
                    return last_update.elapsed() < duration;
                }
            }
        }

        false
    }

    fn current_detail_reference(&self) -> Option<&str> {
        self.detail_reference_index
            .and_then(|idx| self.detail_references.get(idx))
            .map(|s| s.as_str())
    }

    async fn open_selected_reference(&mut self) -> Result<()> {
        let Some(anumber) = self.current_detail_reference().map(|s| s.to_string()) else {
            return Ok(());
        };

        self.error_message = None;
        self.searching = true;
        self.spinner_frame = 0;

        match self.client.get_sequence(&anumber).await? {
            Some(sequence) => {
                let cached = sequence.clone();
                self.current_sequence = Some(sequence);
                self.detail_tab = 0;
                self.detail_scroll = 0;
                self.reset_detail_reference_state();
                let _ = self.cache.cache_sequence(&cached);
                let _ = self.cache.record_view(cached.number);
            }
            None => {
                self.error_message = Some(format!("Sequence {} not found", anumber));
            }
        }

        self.searching = false;
        Ok(())
    }

    /// Handle keyboard input
    async fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        // Ignore key repeat/release events; we only care about initial presses
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        // Global keybindings (configurable via settings.json)
        if self.keybindings.is_toggle_keybindings_preset(&key) {
            self.toggle_keybindings_preset();
            return Ok(());
        }
        if self.keybindings.is_quit(&key) {
            self.should_quit = true;
            return Ok(());
        }
        if self.keybindings.is_help(&key) {
            self.help_modal_visible = !self.help_modal_visible;
            return Ok(());
        }
        if self.keybindings.is_about(&key) {
            self.about_modal_visible = !self.about_modal_visible;
            return Ok(());
        }
        if self.keybindings.is_cycle_theme(&key) {
            self.cycle_theme();
            return Ok(());
        }

        if self.help_modal_visible {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.help_modal_visible = false;
                    self.help_scroll = 0; // Reset scroll when closing
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.help_scroll = self.help_scroll.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.help_scroll = self.help_scroll.saturating_add(1);
                }
                KeyCode::PageUp => {
                    self.help_scroll = self.help_scroll.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    self.help_scroll = self.help_scroll.saturating_add(10);
                }
                KeyCode::Home => {
                    self.help_scroll = 0;
                }
                KeyCode::End => {
                    self.help_scroll = 100; // Arbitrary large number to scroll to bottom
                }
                _ => {}
            }
            return Ok(());
        }

        if self.about_modal_visible {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.about_modal_visible = false;
                }
                _ => {}
            }
            return Ok(());
        }

        // Screen-specific handling
        match self.current_screen {
            Screen::Search => self.handle_search_input(key).await?,
            Screen::Detail => self.handle_detail_input(key).await?,
            Screen::Graph => self.handle_graph_input(key).await?,
            Screen::Webcam => self.handle_webcam_input(key).await?,
            Screen::Settings => self.handle_settings_input(key).await?,
        }

        Ok(())
    }

    /// Handle mouse input
    async fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<()> {
        if self.help_modal_visible {
            if matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                self.help_modal_visible = false;
            }
            return Ok(());
        }

        if self.about_modal_visible {
            if matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                self.about_modal_visible = false;
            }
            return Ok(());
        }

        if self.show_welcome_modal {
            if matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
                if let Some(area) = self.search_input_area {
                    if Self::point_in_rect(area, event.column, event.row) {
                        let inner_x = area.x.saturating_add(1);
                        let relative = event.column.saturating_sub(inner_x) as usize;
                        self.search_cursor = relative.min(self.search_input.len());
                        self.input_mode = InputMode::Editing;
                        return Ok(());
                    }
                }
                self.close_welcome_modal();
            }
            return Ok(());
        }

        match self.current_screen {
            Screen::Search => {
                self.handle_search_mouse(event).await?;
            }
            Screen::Detail => {
                self.handle_detail_mouse(event).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_search_mouse(&mut self, event: MouseEvent) -> Result<()> {
        let column = event.column;
        let row = event.row;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(area) = self.search_input_area {
                    if Self::point_in_rect(area, column, row) {
                        let inner_x = area.x.saturating_add(1);
                        let relative = column.saturating_sub(inner_x) as usize;
                        self.search_cursor = relative.min(self.search_input.len());
                        self.input_mode = InputMode::Editing;
                        self.last_result_click = None;
                        return Ok(());
                    }
                }

                if let Some(area) = self.search_results_area {
                    if Self::point_in_rect(area, column, row) && !self.search_results.is_empty() {
                        if let Some(index) = self.result_index_from_position(area, row) {
                            let now = Instant::now();
                            let double_click = self
                                .last_result_click
                                .filter(|(_, last_index)| *last_index == index)
                                .is_some_and(|(last_time, _)| {
                                    last_time.elapsed()
                                        <= Duration::from_millis(DOUBLE_CLICK_THRESHOLD_MS)
                                });

                            self.last_result_click = Some((now, index));
                            self.input_mode = InputMode::Normal;

                            if double_click {
                                self.view_selected_sequence().await?;
                                self.last_result_click = None;
                            } else {
                                self.selected_result = index;
                                self.update_preview_if_enabled();
                            }
                        }
                        return Ok(());
                    }
                }

                if let Some(area) = self.search_preview_tabs_area {
                    if Self::point_in_rect(area, column, row) {
                        if let Some(index) =
                            Self::tab_index_from_point(area, column, PREVIEW_TAB_COUNT)
                        {
                            self.preview_tab = index;
                        }
                        return Ok(());
                    }
                }

                if let Some(area) = self.search_preview_area {
                    if Self::point_in_rect(area, column, row) && self.current_sequence.is_some() {
                        self.current_screen = Screen::Detail;
                        self.detail_tab = 0;
                        self.detail_scroll = 0;
                        self.reset_detail_reference_state();
                        self.last_result_click = None;
                        return Ok(());
                    }
                }

                // Handle history panel clicks
                if let Some(area) = self.history_area {
                    if Self::point_in_rect(area, column, row) && !self.recent_sequences.is_empty() {
                        if let Some(index) = Self::history_index_from_position(area, row) {
                            if index < self.recent_sequences.len() {
                                let (number, _, _, _) = self.recent_sequences[index];
                                self.load_sequence_by_number(number).await?;
                                return Ok(());
                            }
                        }
                    }
                }

                self.input_mode = InputMode::Normal;
                self.last_result_click = None;
            }
            MouseEventKind::ScrollUp => {
                if let Some(area) = self.search_results_area {
                    if Self::point_in_rect(area, column, row) {
                        self.select_previous_result();
                        return Ok(());
                    }
                }
                if let Some(area) = self.search_preview_area {
                    if Self::point_in_rect(area, column, row) && self.preview_scroll > 0 {
                        self.preview_scroll =
                            self.preview_scroll.saturating_sub(PREVIEW_SCROLL_LINES);
                        return Ok(());
                    }
                }
            }
            MouseEventKind::ScrollDown => {
                if let Some(area) = self.search_results_area {
                    if Self::point_in_rect(area, column, row) {
                        self.select_next_result();
                        return Ok(());
                    }
                }
                if let Some(area) = self.search_preview_area {
                    if Self::point_in_rect(area, column, row) {
                        self.preview_scroll =
                            self.preview_scroll.saturating_add(PREVIEW_SCROLL_LINES);
                        return Ok(());
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    async fn handle_detail_mouse(&mut self, event: MouseEvent) -> Result<()> {
        let column = event.column;
        let row = event.row;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(area) = self.detail_tabs_area {
                    if Self::point_in_rect(area, column, row) {
                        if let Some(index) =
                            Self::tab_index_from_point(area, column, DETAIL_TAB_COUNT)
                        {
                            if self.detail_tab != index {
                                self.detail_tab = index;
                                self.detail_scroll = 0;
                                self.reset_detail_reference_state();
                            }
                        }
                        return Ok(());
                    }
                }

                if let Some(area) = self.detail_content_area {
                    if Self::point_in_rect(area, column, row)
                        && event.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        self.open_selected_reference().await?;
                        return Ok(());
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                if let Some(area) = self.detail_content_area {
                    if Self::point_in_rect(area, column, row) && self.detail_scroll > 0 {
                        self.detail_scroll = self.detail_scroll.saturating_sub(SCROLL_LINES);
                        return Ok(());
                    }
                }
            }
            MouseEventKind::ScrollDown => {
                if let Some(area) = self.detail_content_area {
                    if Self::point_in_rect(area, column, row) {
                        self.detail_scroll = self.detail_scroll.saturating_add(SCROLL_LINES);
                        return Ok(());
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn point_in_rect(rect: Rect, column: u16, row: u16) -> bool {
        let max_x = rect.x.saturating_add(rect.width);
        let max_y = rect.y.saturating_add(rect.height);
        column >= rect.x && column < max_x && row >= rect.y && row < max_y
    }

    fn tab_index_from_point(rect: Rect, column: u16, count: usize) -> Option<usize> {
        if count == 0 || rect.width <= 2 {
            return None;
        }

        let inner_x = rect.x.saturating_add(1);
        let inner_width = rect.width.saturating_sub(2);
        if column < inner_x || column >= inner_x + inner_width {
            return None;
        }

        let relative = column - inner_x;
        let denominator = inner_width.max(1) as usize;
        let mut index = (relative as usize * count) / denominator;
        if index >= count {
            index = count - 1;
        }
        Some(index)
    }

    fn result_index_from_position(&self, rect: Rect, row: u16) -> Option<usize> {
        if self.search_results.is_empty() || rect.height <= 3 {
            return None;
        }

        let inner_top = rect.y.saturating_add(1);
        let inner_bottom = rect.y.saturating_add(rect.height).saturating_sub(1);

        if row < inner_top || row >= inner_bottom {
            return None;
        }

        let first_row = inner_top.saturating_add(2); // header + spacer
        if row < first_row {
            return None;
        }

        let mut index = (row - first_row) as usize;
        if index >= self.search_results.len() {
            index = self.search_results.len() - 1;
        }
        Some(index)
    }

    fn history_index_from_position(rect: Rect, row: u16) -> Option<usize> {
        if rect.height <= 3 {
            return None;
        }

        let inner_top = rect.y.saturating_add(1);
        let inner_bottom = rect.y.saturating_add(rect.height).saturating_sub(1);

        if row < inner_top || row >= inner_bottom {
            return None;
        }

        let first_row = inner_top.saturating_add(2); // header + spacer
        if row < first_row {
            return None;
        }

        Some((row - first_row) as usize)
    }

    /// Handle input on search screen
    async fn handle_search_input(&mut self, key: KeyEvent) -> Result<()> {
        if self.show_welcome_modal {
            match key.code {
                KeyCode::Enter => {
                    self.close_welcome_modal();
                    if !self.search_input.trim().is_empty() {
                        self.perform_search()?;
                    }
                    return Ok(());
                }
                KeyCode::Esc => {
                    self.close_welcome_modal();
                    return Ok(());
                }
                KeyCode::Char('w') => {
                    self.close_welcome_modal();
                    self.current_screen = Screen::Webcam;
                    if self.webcam_interval.is_none() {
                        self.webcam_interval = Some(WebcamInterval::Manual);
                    }
                    self.webcam_focus = WebcamFocus::Categories;
                    return Ok(());
                }
                _ => {}
            }
        }

        match self.input_mode {
            InputMode::Normal => {
                match key.code {
                    KeyCode::Char('i') | KeyCode::Char('/') => {
                        self.input_mode = InputMode::Editing;
                        self.search_focus = SearchFocus::Input;
                    }
                    KeyCode::Enter => {
                        if self.search_focus == SearchFocus::Results
                            && !self.search_results.is_empty()
                        {
                            self.view_selected_sequence().await?;
                        } else if self.search_focus == SearchFocus::History
                            && !self.recent_sequences.is_empty()
                            && self.history_selected < self.recent_sequences.len()
                        {
                            let (number, _, _, _) = self.recent_sequences[self.history_selected];
                            self.load_sequence_by_number(number).await?;
                        } else if self.search_focus == SearchFocus::Bookmarks
                            && !self.bookmarks.is_empty()
                            && self.bookmarks_selected < self.bookmarks.len()
                        {
                            self.load_bookmark(self.bookmarks_selected).await?;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if self.search_focus == SearchFocus::Results {
                            self.select_previous_result();
                        } else if self.search_focus == SearchFocus::History
                            && !self.recent_sequences.is_empty()
                        {
                            self.history_selected = self.history_selected.saturating_sub(1);
                        } else if self.search_focus == SearchFocus::Bookmarks
                            && !self.bookmarks.is_empty()
                        {
                            self.bookmarks_selected = self.bookmarks_selected.saturating_sub(1);
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.search_focus == SearchFocus::Results {
                            self.select_next_result();
                        } else if self.search_focus == SearchFocus::History
                            && !self.recent_sequences.is_empty()
                        {
                            let max_idx = self.recent_sequences.len().saturating_sub(1);
                            self.history_selected = (self.history_selected + 1).min(max_idx);
                        } else if self.search_focus == SearchFocus::Bookmarks
                            && !self.bookmarks.is_empty()
                        {
                            let max_idx = self.bookmarks.len().saturating_sub(1);
                            self.bookmarks_selected = (self.bookmarks_selected + 1).min(max_idx);
                        }
                    }
                    KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Set focus to history panel
                        if !self.show_preview && !self.recent_sequences.is_empty() {
                            self.search_focus = SearchFocus::History;
                            self.input_mode = InputMode::Normal;
                            self.history_selected = 0;
                        }
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.previous_page()?;
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.next_page()?;
                    }
                    KeyCode::Char('r') => {
                        self.start_random_sequence()?;
                    }
                    KeyCode::Char('p') => {
                        self.toggle_preview().await?;
                    }
                    KeyCode::Char('b') => {
                        self.toggle_bookmarks_panel();
                    }
                    KeyCode::Tab => {
                        if self.show_preview {
                            // Cycle through preview tabs when preview is enabled
                            self.preview_tab = (self.preview_tab + 1) % 6;
                        } else {
                            // Cycle through panels: Results -> History/Bookmarks -> Input
                            self.search_focus = match self.search_focus {
                                SearchFocus::Results => {
                                    if self.show_bookmarks && !self.bookmarks.is_empty() {
                                        SearchFocus::Bookmarks
                                    } else if !self.recent_sequences.is_empty() {
                                        SearchFocus::History
                                    } else {
                                        SearchFocus::Input
                                    }
                                }
                                SearchFocus::History | SearchFocus::Bookmarks => SearchFocus::Input,
                                SearchFocus::Input => SearchFocus::Results,
                            };

                            // Update input mode when switching to/from Input focus
                            self.input_mode = if self.search_focus == SearchFocus::Input {
                                InputMode::Editing
                            } else {
                                InputMode::Normal
                            };
                        }
                    }
                    KeyCode::Char('1'..='6') if self.show_preview => {
                        let tab_num = key
                            .code
                            .to_string()
                            .chars()
                            .next()
                            .unwrap()
                            .to_digit(10)
                            .unwrap() as usize;
                        self.preview_tab = (tab_num - 1).min(5);
                    }
                    KeyCode::PageUp if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Scroll preview up
                        if self.show_preview && self.preview_scroll > 0 {
                            self.preview_scroll = self.preview_scroll.saturating_sub(5);
                        }
                    }
                    KeyCode::PageDown if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Scroll preview down
                        if self.show_preview {
                            self.preview_scroll = self.preview_scroll.saturating_add(5);
                        }
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Scroll preview up (Vim-style)
                        if self.show_preview && self.preview_scroll > 0 {
                            self.preview_scroll = self.preview_scroll.saturating_sub(10);
                        }
                    }
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Scroll preview down (Vim-style)
                        if self.show_preview {
                            self.preview_scroll = self.preview_scroll.saturating_add(10);
                        }
                    }
                    KeyCode::Char('w') => {
                        self.current_screen = Screen::Webcam;
                        // Initialize webcam mode if not set
                        if self.webcam_interval.is_none() {
                            self.webcam_interval = Some(WebcamInterval::Manual);
                        }
                        // Load first sequence if none loaded
                        if self.current_sequence.is_none() {
                            // Will be loaded when user presses space/enter
                        }
                        self.webcam_focus = WebcamFocus::Categories;
                    }
                    KeyCode::Char('s') => {
                        self.current_screen = Screen::Settings;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    self.input_mode = InputMode::Normal;
                    self.perform_search()?;
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Char(c) => {
                    self.search_input.insert(self.search_cursor, c);
                    self.search_cursor += 1;
                }
                KeyCode::Backspace => {
                    if self.search_cursor > 0 {
                        self.search_input.remove(self.search_cursor - 1);
                        self.search_cursor -= 1;
                    }
                }
                KeyCode::Delete => {
                    if self.search_cursor < self.search_input.len() {
                        self.search_input.remove(self.search_cursor);
                    }
                }
                KeyCode::Left => {
                    if self.search_cursor > 0 {
                        self.search_cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.search_cursor < self.search_input.len() {
                        self.search_cursor += 1;
                    }
                }
                KeyCode::Home => {
                    self.search_cursor = 0;
                }
                KeyCode::End => {
                    self.search_cursor = self.search_input.len();
                }
                KeyCode::Tab => {
                    // Switch to results table
                    self.input_mode = InputMode::Normal;
                }
                _ => {}
            },
        }
        Ok(())
    }

    /// Handle input on detail screen
    async fn handle_detail_input(&mut self, key: KeyEvent) -> Result<()> {
        // Handle detail help modal if visible
        if self.detail_help_visible {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.detail_help_visible = false;
                    self.detail_help_scroll = 0; // Reset scroll when closing
                }
                KeyCode::Char('?') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.detail_help_visible = false;
                    self.detail_help_scroll = 0; // Reset scroll when closing
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.detail_help_scroll = self.detail_help_scroll.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.detail_help_scroll = self.detail_help_scroll.saturating_add(1);
                }
                KeyCode::PageUp => {
                    self.detail_help_scroll = self.detail_help_scroll.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    self.detail_help_scroll = self.detail_help_scroll.saturating_add(10);
                }
                KeyCode::Home => {
                    self.detail_help_scroll = 0;
                }
                KeyCode::End => {
                    self.detail_help_scroll = 100; // Arbitrary large number to scroll to bottom
                }
                _ => {}
            }
            return Ok(());
        }

        // Handle Ctrl+? key to show help modal
        if let KeyCode::Char('?') = key.code {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                self.detail_help_visible = true;
                return Ok(());
            }
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_screen = Screen::Search;
                self.reset_detail_reference_state();
            }
            KeyCode::Char('g') => {
                // If already on Graph tab, open full-screen graph view
                if self.detail_tab == 6 {
                    self.current_screen = Screen::Graph;
                } else {
                    // Jump to Graph tab
                    self.detail_tab = 6;
                    self.detail_scroll = 0;
                }
            }
            KeyCode::Char('e') => {
                // Jump to Export tab
                self.detail_tab = 7;
                self.detail_scroll = 0;
            }
            KeyCode::Char('o') => {
                self.open_in_browser()?;
            }
            KeyCode::Char('b') => {
                self.toggle_bookmark().await?;
            }
            KeyCode::Char('f') => {
                self.start_bfile_fetch()?;
            }
            KeyCode::Tab => {
                self.detail_tab = (self.detail_tab + 1) % DETAIL_TAB_COUNT;
                self.detail_scroll = 0;
                self.reset_detail_reference_state();
            }
            KeyCode::BackTab => {
                self.detail_tab = if self.detail_tab == 0 {
                    DETAIL_TAB_COUNT - 1
                } else {
                    self.detail_tab - 1
                };
                self.detail_scroll = 0;
                self.reset_detail_reference_state();
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.select_previous_detail_reference();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.select_next_detail_reference();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                // On Export tab, navigate formats instead of scrolling
                if self.detail_tab == 7 {
                    self.export_format = match self.export_format {
                        ExportFormat::Json => ExportFormat::BFile,
                        ExportFormat::Csv => ExportFormat::Json,
                        ExportFormat::Txt => ExportFormat::Csv,
                        ExportFormat::Markdown => ExportFormat::Txt,
                        ExportFormat::BFile => ExportFormat::Markdown,
                    };
                } else {
                    self.detail_scroll = self.detail_scroll.saturating_sub(1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // On Export tab, navigate formats instead of scrolling
                if self.detail_tab == 7 {
                    self.export_format = match self.export_format {
                        ExportFormat::Json => ExportFormat::Csv,
                        ExportFormat::Csv => ExportFormat::Txt,
                        ExportFormat::Txt => ExportFormat::Markdown,
                        ExportFormat::Markdown => ExportFormat::BFile,
                        ExportFormat::BFile => ExportFormat::Json,
                    };
                } else {
                    self.detail_scroll = self.detail_scroll.saturating_add(1);
                }
            }
            KeyCode::PageUp => {
                self.detail_scroll = self.detail_scroll.saturating_sub(10);
            }
            KeyCode::PageDown => {
                self.detail_scroll = self.detail_scroll.saturating_add(10);
            }
            KeyCode::Enter => {
                // On Export tab, export to clipboard
                if self.detail_tab == 7 {
                    self.export_to_clipboard()?;
                } else {
                    self.open_selected_reference().await?;
                }
            }
            KeyCode::Char('1') if self.detail_tab == 7 => {
                self.export_format = ExportFormat::Json;
            }
            KeyCode::Char('2') if self.detail_tab == 7 => {
                self.export_format = ExportFormat::Csv;
            }
            KeyCode::Char('3') if self.detail_tab == 7 => {
                self.export_format = ExportFormat::Txt;
            }
            KeyCode::Char('4') if self.detail_tab == 7 => {
                self.export_format = ExportFormat::Markdown;
            }
            KeyCode::Char('5') if self.detail_tab == 7 => {
                self.export_format = ExportFormat::BFile;
            }
            KeyCode::Char('1') if self.detail_tab == 6 => {
                self.graph_type = GraphType::Line;
            }
            KeyCode::Char('2') if self.detail_tab == 6 => {
                self.graph_type = GraphType::Scatter;
            }
            KeyCode::Char('3') if self.detail_tab == 6 => {
                self.graph_type = GraphType::LogScatter;
            }
            KeyCode::Char('4') if self.detail_tab == 6 => {
                self.graph_type = GraphType::PinPlot;
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) && self.detail_tab == 7 => {
                self.export_to_file()?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle input on graph screen
    async fn handle_graph_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_screen = Screen::Detail;
            }
            KeyCode::Char('1') => self.graph_type = GraphType::Line,
            KeyCode::Char('2') => self.graph_type = GraphType::Scatter,
            KeyCode::Char('3') => self.graph_type = GraphType::LogScatter,
            KeyCode::Char('4') => self.graph_type = GraphType::PinPlot,
            _ => {}
        }
        Ok(())
    }

    /// Handle input on webcam screen
    async fn handle_webcam_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_screen = Screen::Search;
                self.webcam_interval = None;
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.webcam_focus = WebcamFocus::Sequence;
                self.webcam_next_sequence().await?;
            }
            KeyCode::Char('d') => {
                // Switch to detail view if sequence is loaded
                if self.current_sequence.is_some() {
                    self.current_screen = Screen::Detail;
                    self.detail_tab = 0;
                    self.detail_scroll = 0;
                    self.webcam_focus = WebcamFocus::Sequence;
                }
            }
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                // Tab or Right: Switch to next section
                self.webcam_focus = match self.webcam_focus {
                    WebcamFocus::Categories => WebcamFocus::Intervals,
                    WebcamFocus::Intervals | WebcamFocus::Sequence => WebcamFocus::Categories,
                };
            }
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
                // BackTab or Left: Switch to previous section
                self.webcam_focus = match self.webcam_focus {
                    WebcamFocus::Intervals => WebcamFocus::Categories,
                    WebcamFocus::Categories | WebcamFocus::Sequence => WebcamFocus::Intervals,
                };
            }
            // Up/Down navigation works based on current focus
            KeyCode::Up | KeyCode::Char('k') => match self.webcam_focus {
                WebcamFocus::Categories => {
                    // Navigate categories with wraparound
                    if self.webcam_category > 0 {
                        self.webcam_category -= 1;
                    } else {
                        self.webcam_category = 3; // Wrap to last category
                    }
                }
                WebcamFocus::Intervals => {
                    // Navigate intervals upward
                    self.webcam_interval = match self.webcam_interval {
                        None | Some(WebcamInterval::Manual) => Some(WebcamInterval::OneMinute),
                        Some(WebcamInterval::FiveSeconds) => Some(WebcamInterval::Manual),
                        Some(WebcamInterval::TenSeconds) => Some(WebcamInterval::FiveSeconds),
                        Some(WebcamInterval::TwentySeconds) => Some(WebcamInterval::TenSeconds),
                        Some(WebcamInterval::ThirtySeconds) => Some(WebcamInterval::TwentySeconds),
                        Some(WebcamInterval::OneMinute) => Some(WebcamInterval::ThirtySeconds),
                    };
                }
                WebcamFocus::Sequence => {}
            },
            KeyCode::Down | KeyCode::Char('j') => match self.webcam_focus {
                WebcamFocus::Categories => {
                    // Navigate categories with wraparound
                    if self.webcam_category < 3 {
                        self.webcam_category += 1;
                    } else {
                        self.webcam_category = 0; // Wrap to first category
                    }
                }
                WebcamFocus::Intervals => {
                    // Navigate intervals downward
                    self.webcam_interval = match self.webcam_interval {
                        None | Some(WebcamInterval::Manual) => Some(WebcamInterval::FiveSeconds),
                        Some(WebcamInterval::FiveSeconds) => Some(WebcamInterval::TenSeconds),
                        Some(WebcamInterval::TenSeconds) => Some(WebcamInterval::TwentySeconds),
                        Some(WebcamInterval::TwentySeconds) => Some(WebcamInterval::ThirtySeconds),
                        Some(WebcamInterval::ThirtySeconds) => Some(WebcamInterval::OneMinute),
                        Some(WebcamInterval::OneMinute) => Some(WebcamInterval::Manual),
                    };
                }
                WebcamFocus::Sequence => {}
            },
            // Quick interval selection
            KeyCode::Char('0') => {
                self.webcam_focus = WebcamFocus::Intervals;
                self.webcam_interval = Some(WebcamInterval::Manual);
            }
            KeyCode::Char('1') => {
                self.webcam_focus = WebcamFocus::Intervals;
                self.webcam_interval = Some(WebcamInterval::FiveSeconds);
            }
            KeyCode::Char('2') => {
                self.webcam_focus = WebcamFocus::Intervals;
                self.webcam_interval = Some(WebcamInterval::TenSeconds);
            }
            KeyCode::Char('3') => {
                self.webcam_focus = WebcamFocus::Intervals;
                self.webcam_interval = Some(WebcamInterval::TwentySeconds);
            }
            KeyCode::Char('4') => {
                self.webcam_focus = WebcamFocus::Intervals;
                self.webcam_interval = Some(WebcamInterval::ThirtySeconds);
            }
            KeyCode::Char('5') => {
                self.webcam_focus = WebcamFocus::Intervals;
                self.webcam_interval = Some(WebcamInterval::OneMinute);
            }
            _ => {}
        }
        // Update scroll offsets to keep selected items visible
        self.update_webcam_scroll();
        Ok(())
    }

    /// Handle input on settings screen
    async fn handle_settings_input(&mut self, key: KeyEvent) -> Result<()> {
        let languages = Language::all();
        let language_max = languages.len().saturating_sub(1);
        let theme_max = self.themes.len().saturating_sub(1);
        let animation_max = WelcomeAnimationMode::modes().len().saturating_sub(1);

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                self.current_screen = Screen::Search;
            }
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                self.settings_focus = match self.settings_focus {
                    SettingsFocus::Language => SettingsFocus::Theme,
                    SettingsFocus::Theme => SettingsFocus::Animation,
                    SettingsFocus::Animation => SettingsFocus::Language,
                };
            }
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
                self.settings_focus = match self.settings_focus {
                    SettingsFocus::Language => SettingsFocus::Animation,
                    SettingsFocus::Theme => SettingsFocus::Language,
                    SettingsFocus::Animation => SettingsFocus::Theme,
                };
            }
            KeyCode::Up | KeyCode::Char('k') => match self.settings_focus {
                SettingsFocus::Language => {
                    if self.settings_selected_language > 0 {
                        self.settings_selected_language -= 1;
                    } else {
                        self.settings_selected_language = language_max;
                    }
                }
                SettingsFocus::Theme => {
                    if self.settings_selected_theme > 0 {
                        self.settings_selected_theme -= 1;
                    } else if !self.themes.is_empty() {
                        self.settings_selected_theme = theme_max;
                    }
                }
                SettingsFocus::Animation => {
                    if self.settings_selected_animation > 0 {
                        self.settings_selected_animation -= 1;
                    } else {
                        self.settings_selected_animation = animation_max;
                    }
                }
            },
            KeyCode::Down | KeyCode::Char('j') => match self.settings_focus {
                SettingsFocus::Language => {
                    if self.settings_selected_language < language_max {
                        self.settings_selected_language += 1;
                    } else {
                        self.settings_selected_language = 0;
                    }
                }
                SettingsFocus::Theme => {
                    if self.settings_selected_theme < theme_max {
                        self.settings_selected_theme += 1;
                    } else if !self.themes.is_empty() {
                        self.settings_selected_theme = 0;
                    }
                }
                SettingsFocus::Animation => {
                    if self.settings_selected_animation < animation_max {
                        self.settings_selected_animation += 1;
                    } else {
                        self.settings_selected_animation = 0;
                    }
                }
            },
            KeyCode::Enter => match self.settings_focus {
                SettingsFocus::Language => {
                    if let Some(lang) = languages.get(self.settings_selected_language) {
                        self.set_language(*lang);
                    }
                }
                SettingsFocus::Theme => {
                    self.set_active_theme(self.settings_selected_theme);
                }
                SettingsFocus::Animation => {
                    let mode = Self::animation_mode_from_index(self.settings_selected_animation);
                    self.set_welcome_animation_mode(mode);
                }
            },
            KeyCode::Char('1') if matches!(self.settings_focus, SettingsFocus::Language) => {
                if let Some(lang) = languages.first() {
                    self.settings_selected_language = 0;
                    self.set_language(*lang);
                }
            }
            KeyCode::Char('2') if matches!(self.settings_focus, SettingsFocus::Language) => {
                if let Some(lang) = languages.get(1) {
                    self.settings_selected_language = 1;
                    self.set_language(*lang);
                }
            }
            KeyCode::Char('3') if matches!(self.settings_focus, SettingsFocus::Language) => {
                if let Some(lang) = languages.get(2) {
                    self.settings_selected_language = 2;
                    self.set_language(*lang);
                }
            }
            KeyCode::Char('4') if matches!(self.settings_focus, SettingsFocus::Language) => {
                if let Some(lang) = languages.get(3) {
                    self.settings_selected_language = 3;
                    self.set_language(*lang);
                }
            }
            KeyCode::Char('5') if matches!(self.settings_focus, SettingsFocus::Language) => {
                if let Some(lang) = languages.get(4) {
                    self.settings_selected_language = 4;
                    self.set_language(*lang);
                }
            }
            KeyCode::Char('6') if matches!(self.settings_focus, SettingsFocus::Language) => {
                if let Some(lang) = languages.get(5) {
                    self.settings_selected_language = 5;
                    self.set_language(*lang);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Perform a search with the current input
    fn perform_search(&mut self) -> Result<()> {
        if self.search_input.trim().is_empty() {
            return Ok(());
        }

        self.search_terms = parse_search_terms(&self.search_input);
        let query = SearchQuery::new(&self.search_input);
        self.spawn_search(query, PendingSearchKind::Initial);
        Ok(())
    }

    /// Navigate to the next page of results
    fn next_page(&mut self) -> Result<()> {
        if let Some(ref query) = self.current_query {
            let next = query.next_page(self.results_per_page);
            self.spawn_search(next, PendingSearchKind::NextPage);
        }
        Ok(())
    }

    /// Navigate to the previous page of results
    fn previous_page(&mut self) -> Result<()> {
        if let Some(ref query) = self.current_query {
            if query.start > 0 {
                let prev = query.prev_page(self.results_per_page);
                self.spawn_search(prev, PendingSearchKind::PreviousPage);
            }
        }
        Ok(())
    }

    /// Select the next result in the list
    fn select_next_result(&mut self) {
        if !self.search_results.is_empty() {
            self.selected_result = (self.selected_result + 1).min(self.search_results.len() - 1);
            self.update_preview_if_enabled();
        }
    }

    /// Select the previous result in the list
    fn select_previous_result(&mut self) {
        if self.selected_result > 0 {
            self.selected_result -= 1;
            self.update_preview_if_enabled();
        }
    }

    /// Update preview pane if it's enabled
    fn update_preview_if_enabled(&mut self) {
        if self.show_preview && self.selected_result < self.search_results.len() {
            let sequence = self.search_results[self.selected_result].clone();
            self.current_sequence = Some(sequence);
            self.preview_scroll = 0;
        }
    }

    /// View the currently selected sequence
    async fn view_selected_sequence(&mut self) -> Result<()> {
        if let Some(sequence) = self.search_results.get(self.selected_result).cloned() {
            self.clear_bfile_data();
            self.current_sequence = Some(sequence.clone());
            self.current_screen = Screen::Detail;
            self.detail_tab = 0;
            self.detail_scroll = 0;
            self.reset_detail_reference_state();
            self.last_result_click = None;

            // Record view in cache
            let _ = self.cache.record_view(sequence.number);
            // Refresh history
            self.recent_sequences = self
                .cache
                .get_recently_viewed_with_details(8)
                .unwrap_or_default();

            // Cache the sequence
            let _ = self.cache.cache_sequence(&sequence);
        }
        Ok(())
    }

    /// Load a sequence by number (for history panel)
    async fn load_sequence_by_number(&mut self, number: i32) -> Result<()> {
        self.clear_bfile_data();
        // Try to get from cache first
        if let Ok(Some(sequence)) = self.cache.get_cached_sequence(number, 30) {
            self.current_sequence = Some(sequence.clone());
            self.current_screen = Screen::Detail;
            self.detail_tab = 0;
            self.detail_scroll = 0;
            self.reset_detail_reference_state();

            // Record view
            let _ = self.cache.record_view(number);
            // Refresh history
            self.recent_sequences = self
                .cache
                .get_recently_viewed_with_details(8)
                .unwrap_or_default();
        } else {
            // Fetch from API if not in cache
            let a_number = format!("A{:06}", number);
            if let Ok(Some(sequence)) = self.client.get_sequence(&a_number).await {
                self.current_sequence = Some(sequence.clone());
                self.current_screen = Screen::Detail;
                self.detail_tab = 0;
                self.detail_scroll = 0;
                self.reset_detail_reference_state();

                // Record view and cache
                let _ = self.cache.record_view(number);
                let _ = self.cache.cache_sequence(&sequence);
                // Refresh history
                self.recent_sequences = self
                    .cache
                    .get_recently_viewed_with_details(8)
                    .unwrap_or_default();
            }
        }
        Ok(())
    }

    /// Toggle preview pane on search screen
    async fn toggle_preview(&mut self) -> Result<()> {
        self.show_preview = !self.show_preview;

        // If enabling preview and we have results, load the selected sequence
        if self.show_preview
            && !self.search_results.is_empty()
            && self.selected_result < self.search_results.len()
        {
            let sequence = self.search_results[self.selected_result].clone();
            self.current_sequence = Some(sequence);
            self.preview_scroll = 0;
            self.preview_tab = 0;
        }

        Ok(())
    }

    /// Toggle bookmark for current sequence
    async fn toggle_bookmark(&mut self) -> Result<()> {
        if let Some(ref sequence) = self.current_sequence {
            let number = sequence.number;
            let is_bookmarked = self.cache.is_bookmarked(number).unwrap_or(false);

            if is_bookmarked {
                self.cache.remove_bookmark(number)?;
            } else {
                self.cache.add_bookmark(number, None)?;
            }

            // Refresh bookmarks list
            self.refresh_bookmarks().await?;
        }
        Ok(())
    }

    /// Refresh bookmarks list from cache
    async fn refresh_bookmarks(&mut self) -> Result<()> {
        self.bookmarks = self.cache.get_bookmarks().unwrap_or_default();
        // Ensure selection is valid
        if !self.bookmarks.is_empty() && self.bookmarks_selected >= self.bookmarks.len() {
            self.bookmarks_selected = self.bookmarks.len() - 1;
        } else if self.bookmarks.is_empty() {
            self.bookmarks_selected = 0;
        }
        Ok(())
    }

    /// Toggle bookmarks panel visibility
    fn toggle_bookmarks_panel(&mut self) {
        self.show_bookmarks = !self.show_bookmarks;
        // Switch focus based on which panel is now shown
        if self.show_bookmarks {
            self.search_focus = SearchFocus::Bookmarks;
        } else {
            self.search_focus = SearchFocus::History;
        }
    }

    /// Load selected bookmark
    async fn load_bookmark(&mut self, index: usize) -> Result<()> {
        if index < self.bookmarks.len() {
            let number = self.bookmarks[index].0;
            self.load_sequence_by_number(number).await?;
        }
        Ok(())
    }

    /// Start B-file fetch for current sequence
    fn start_bfile_fetch(&mut self) -> Result<()> {
        if let Some(ref sequence) = self.current_sequence {
            let number = sequence.number;
            let client = self.client.clone();

            self.bfile_data = None;
            self.bfile_error = None;

            let handle = tokio::spawn(async move {
                client.fetch_b_file(number).await
            });

            self.pending_bfile = Some(PendingBFile {
                handle,
                started_at: Instant::now(),
            });
            self.searching = true;
        }
        Ok(())
    }

    /// Cancel pending B-file fetch
    fn cancel_pending_bfile(&mut self) {
        if let Some(pending) = self.pending_bfile.take() {
            pending.handle.abort();
        }
        self.searching = false;
    }

    /// Poll pending B-file fetch and handle completion
    async fn poll_pending_bfile(&mut self) -> Result<()> {
        if let Some(pending) = self.pending_bfile.as_ref() {
            if pending.handle.is_finished() {
                let pending = self.pending_bfile.take().unwrap();
                let started_at = pending.started_at;

                match pending.handle.await {
                    Ok(Ok(data)) => {
                        self.complete_bfile_success(data, started_at);
                    }
                    Ok(Err(e)) => {
                        self.complete_bfile_failure(e);
                    }
                    Err(e) => {
                        self.complete_bfile_join_error(e);
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle successful B-file fetch
    fn complete_bfile_success(&mut self, data: Vec<BFileEntry>, _started_at: Instant) {
        self.bfile_data = Some(data);
        self.bfile_error = None;
        self.searching = false;
    }

    /// Handle B-file fetch failure
    fn complete_bfile_failure(&mut self, error: anyhow::Error) {
        self.bfile_error = Some(format!("B-file not available: {}", error));
        self.bfile_data = None;
        self.searching = false;
    }

    /// Handle B-file fetch join error
    fn complete_bfile_join_error(&mut self, error: tokio::task::JoinError) {
        self.bfile_error = Some(format!("B-file fetch failed: {}", error));
        self.bfile_data = None;
        self.searching = false;
    }

    /// Clear B-file data
    fn clear_bfile_data(&mut self) {
        self.bfile_data = None;
        self.bfile_error = None;
        self.cancel_pending_bfile();
    }

    /// Load next sequence in webcam mode
    async fn webcam_next_sequence(&mut self) -> Result<()> {
        // Build query based on selected category
        let query = match self.webcam_category {
            0 => {
                // All Sequences - get a random one
                match self.client.random_sequence().await {
                    Ok(Some(sequence)) => {
                        self.current_sequence = Some(sequence);
                        self.webcam_last_update = Some(std::time::Instant::now());
                        self.reset_detail_reference_state();
                        self.webcam_focus = WebcamFocus::Sequence;
                        return Ok(());
                    }
                    Ok(None) => {
                        self.error_message = Some("No sequence found".to_string());
                        return Ok(());
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Error loading sequence: {}", e));
                        return Ok(());
                    }
                }
            }
            1 => "keyword:nice", // Best Sequences
            2 => "keyword:more", // Needing Terms
            3 => "keyword:new",  // Recent Additions
            _ => "keyword:nice",
        };

        // For category-based searches, fetch and pick a random one from results
        let search_query = SearchQuery::new(query);

        match self.client.search(&search_query, 10).await {
            Ok(response) => {
                if let Some(results) = response.results {
                    if !results.is_empty() {
                        // Pick a random sequence from the results
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let index = rng.gen_range(0..results.len());
                        self.current_sequence = Some(results[index].clone());
                        self.webcam_last_update = Some(std::time::Instant::now());
                        self.reset_detail_reference_state();
                        self.webcam_focus = WebcamFocus::Sequence;
                    } else {
                        self.error_message =
                            Some("No sequences found in this category".to_string());
                    }
                } else {
                    self.error_message = Some("Too many results, please try again".to_string());
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Error loading sequence: {}", e));
            }
        }

        Ok(())
    }

    /// Update webcam scroll offsets to keep selected items visible
    /// visible_height: approximate number of visible items in the list (accounting for multi-line items)
    fn update_webcam_scroll(&mut self) {
        // For categories: 4 items total, each takes ~2 lines (name + description)
        // With borders and title, we have about 6 lines visible
        let category_visible = 2; // visible items (accounting for 2-line items)
        self.webcam_category_scroll = calculate_scroll_offset(
            self.webcam_category,
            4, // total categories
            category_visible,
            self.webcam_category_scroll,
        );

        // For intervals: 6 items total, each takes ~2 lines
        let interval_visible = 2; // visible items
        let selected_interval_index = match self.webcam_interval {
            None | Some(WebcamInterval::Manual) => 0,
            Some(WebcamInterval::FiveSeconds) => 1,
            Some(WebcamInterval::TenSeconds) => 2,
            Some(WebcamInterval::TwentySeconds) => 3,
            Some(WebcamInterval::ThirtySeconds) => 4,
            Some(WebcamInterval::OneMinute) => 5,
        };
        self.webcam_interval_scroll = calculate_scroll_offset(
            selected_interval_index,
            6, // total intervals
            interval_visible,
            self.webcam_interval_scroll,
        );
    }

    /// Open the current sequence in browser
    fn open_in_browser(&self) -> Result<()> {
        if let Some(ref sequence) = self.current_sequence {
            let url = sequence.url();
            open::that(url)?;
        }
        Ok(())
    }

    /// Export current sequence to clipboard
    fn export_to_clipboard(&mut self) -> Result<()> {
        use fluent::FluentArgs;

        if let Some(ref seq) = self.current_sequence {
            let content = crate::ui::export::export_sequence(
                seq,
                &self.export_format,
                self.bfile_data.as_ref(),
            );

            match arboard::Clipboard::new().and_then(|mut clip| clip.set_text(content)) {
                Ok(_) => {
                    self.error_message = Some(self.i18n.t("export-success").to_string());
                }
                Err(e) => {
                    let mut args = FluentArgs::new();
                    args.set("message", e.to_string());
                    self.error_message = Some(self.i18n.t_with_args("error-clipboard", Some(&args)));
                }
            }
        }
        Ok(())
    }

    /// Export current sequence to file
    fn export_to_file(&mut self) -> Result<()> {
        use fluent::FluentArgs;

        if let Some(ref seq) = self.current_sequence {
            let content = crate::ui::export::export_sequence(
                seq,
                &self.export_format,
                self.bfile_data.as_ref(),
            );
            let filename = format!("{}.{}", seq.a_number(), self.export_format.extension());

            match std::fs::write(&filename, content) {
                Ok(_) => {
                    let mut args = FluentArgs::new();
                    args.set("path", filename.clone());
                    self.error_message = Some(self.i18n.t_with_args("export-file-success", Some(&args)));
                }
                Err(e) => {
                    let mut args = FluentArgs::new();
                    args.set("message", e.to_string());
                    self.error_message = Some(self.i18n.t_with_args("error-file", Some(&args)));
                }
            }
        }
        Ok(())
    }

    /// Get the current spinner character based on animation frame
    pub fn get_spinner_char(&self) -> char {
        // Divide by 2 to slow down animation (updates every ~200ms instead of ~100ms)
        match (self.spinner_frame / 2) % 4 {
            0 => '|',
            1 => '/',
            2 => '-',
            3 => '\\',
            _ => '|',
        }
    }
}


/// Calculate scroll offset to keep selected item visible
/// Returns the appropriate scroll offset to ensure selected_index is visible
fn calculate_scroll_offset(
    selected_index: usize,
    total_items: usize,
    visible_items: usize,
    current_offset: u16,
) -> u16 {
    if total_items <= visible_items {
        // All items fit in view, no need to scroll
        return 0;
    }

    let current_offset = current_offset as usize;

    // If selected item is above visible area, scroll up to it
    if selected_index < current_offset {
        return selected_index as u16;
    }

    // If selected item is below visible area, scroll down to show it at bottom
    if selected_index >= current_offset + visible_items {
        return (selected_index.saturating_sub(visible_items - 1)) as u16;
    }

    // Selected item is already visible, keep current offset
    current_offset as u16
}
