use std::{cell::RefCell, rc::Rc, time::Instant};

use ratatui::{layout::Rect, style::Color, Frame};
use tachyonfx::{
    fx, CellFilter, Duration as FxDuration, Effect, EffectRenderer, EffectTimer, Interpolation,
    RefRect,
};

use super::Theme;

const DIGIT_SYMBOLS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const FIB_DIGITS: &str = "011235813213455891442333776109871597258441816765109461";
const PRIME_DIGITS: &str = "2357111317192329313741434753596167717379838997101103";
const LOOK_SAY_DIGITS: &str = "11231221113112221131112211313211321322113312221113";
const CATALAN_DIGITS: &str = "11251442132142914304862167961679658786";
const SEQUENCE_POOL: [&str; 4] = [FIB_DIGITS, PRIME_DIGITS, LOOK_SAY_DIGITS, CATALAN_DIGITS];

fn sequence_digit(seq: &str, idx: usize) -> u8 {
    if seq.is_empty() {
        return b'0';
    }
    let bytes = seq.as_bytes();
    bytes[idx % bytes.len()]
}

fn digit_symbol(digit: u8) -> &'static str {
    let idx = digit.saturating_sub(b'0') % 10;
    DIGIT_SYMBOLS[idx as usize]
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let (ar, ag, ab) = color_to_rgb(a);
    let (br, bg, bb) = color_to_rgb(b);
    let mix = |x: u8, y: u8| -> u8 {
        let xi = x as f32;
        let yi = y as f32;
        ((xi + (yi - xi) * t).round().clamp(0.0, 255.0)) as u8
    };
    Color::Rgb(mix(ar, br), mix(ag, bg), mix(ab, bb))
}

fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Black => (0, 0, 0),
        Color::Red => (205, 0, 0),
        Color::Green => (0, 205, 0),
        Color::Yellow => (205, 205, 0),
        Color::Blue => (0, 0, 205),
        Color::Magenta => (205, 0, 205),
        Color::Cyan => (0, 205, 205),
        Color::Gray => (190, 190, 190),
        Color::DarkGray => (100, 100, 100),
        Color::LightRed => (255, 85, 85),
        Color::LightGreen => (85, 255, 85),
        Color::LightYellow => (255, 255, 85),
        Color::LightBlue => (85, 85, 255),
        Color::LightMagenta => (255, 85, 255),
        Color::LightCyan => (85, 255, 255),
        Color::White => (255, 255, 255),
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Indexed(i) => (i, i, i),
        Color::Reset => (0, 0, 0),
    }
}

#[derive(Clone)]
struct SnakePalette {
    background_dim: Color,
    trace: Color,
    head: Color,
}

impl SnakePalette {
    fn from_theme(theme: &Theme) -> Self {
        // Keep the animation on a pitch black canvas.
        let background_dim = Color::Black;
        let trace = lerp_color(theme.muted_color(), theme.accent_color(), 0.45);
        let head = theme.accent_color();

        Self {
            background_dim,
            trace,
            head,
        }
    }
}

#[derive(Clone)]
struct SnakeConfig {
    speed_multiplier: f32,
}

impl SnakeConfig {
    fn new(multiplier: f32) -> Self {
        Self {
            speed_multiplier: multiplier,
        }
    }
}

#[derive(Clone)]
struct SequenceFieldState {
    palette: Rc<RefCell<SnakePalette>>,
    config: Rc<RefCell<SnakeConfig>>,
    phase: f32,
    scroll: u32,
}

impl SequenceFieldState {
    fn new(palette: Rc<RefCell<SnakePalette>>, config: Rc<RefCell<SnakeConfig>>) -> Self {
        Self {
            palette,
            config,
            phase: 0.0,
            scroll: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WelcomeAnimationMode {
    Normal,
    Slow,
    Off,
}

impl Default for WelcomeAnimationMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl WelcomeAnimationMode {
    const ALL: [Self; 3] = [Self::Normal, Self::Slow, Self::Off];

    pub fn modes() -> &'static [Self; 3] {
        &Self::ALL
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Slow => "Slow",
            Self::Off => "Off",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Normal => "Sequence snake with full motion",
            Self::Slow => "Sequence snake at a calmer pace",
            Self::Off => "Disable background animation",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Slow => "slow",
            Self::Off => "off",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "normal" => Some(Self::Normal),
            "slow" => Some(Self::Slow),
            "off" => Some(Self::Off),
            _ => None,
        }
    }

    fn speed_multiplier(&self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Slow => 0.4,
            Self::Off => 0.0,
        }
    }
}

/// Animated background that renders a coiling numeric "sequence snake" behind the welcome modal.
pub struct WelcomeAnimation {
    effect: Effect,
    palette: Rc<RefCell<SnakePalette>>,
    config: Rc<RefCell<SnakeConfig>>,
    exclusion_area: RefRect,
    last_tick: Instant,
    mode: WelcomeAnimationMode,
}

impl WelcomeAnimation {
    pub fn new(theme: &Theme, mode: WelcomeAnimationMode) -> Self {
        let palette = Rc::new(RefCell::new(SnakePalette::from_theme(theme)));
        let config = Rc::new(RefCell::new(SnakeConfig::new(mode.speed_multiplier())));
        let exclusion_area = RefRect::new(Rect::default());
        let effect = build_effect(palette.clone(), config.clone(), exclusion_area.clone());

        Self {
            effect,
            palette,
            config,
            exclusion_area,
            last_tick: Instant::now(),
            mode,
        }
    }

    pub fn update_palette(&self, theme: &Theme) {
        *self.palette.borrow_mut() = SnakePalette::from_theme(theme);
    }

    pub fn set_mode(&mut self, mode: WelcomeAnimationMode) {
        self.mode = mode;
        self.config.borrow_mut().speed_multiplier = mode.speed_multiplier();
        self.effect.reset();
        self.last_tick = Instant::now();
    }

    pub fn render(&mut self, frame: &mut Frame, screen_area: Rect, modal_area: Rect) {
        if matches!(self.mode, WelcomeAnimationMode::Off) {
            self.last_tick = Instant::now();
            return;
        }

        self.exclusion_area.set(modal_area);
        let now = Instant::now();
        let elapsed = now.saturating_duration_since(self.last_tick);
        self.last_tick = now;

        let millis = elapsed.as_millis().clamp(1, u32::MAX as u128) as u32;
        let tick = FxDuration::from_millis(millis);

        frame.render_effect(&mut self.effect, screen_area, tick);
    }
}

fn build_effect(
    palette: Rc<RefCell<SnakePalette>>,
    config: Rc<RefCell<SnakeConfig>>,
    exclusion_area: RefRect,
) -> Effect {
    let state = SequenceFieldState::new(palette.clone(), config.clone());

    // Draw a coiled "sequence snake" made of OEIS digits winding around the welcome modal.
    let snake_fx = fx::effect_fn_buf(
        state,
        EffectTimer::from_ms(1100, Interpolation::SineInOut),
        |state, ctx, buf| {
            let speed_multiplier = {
                let cfg = state.config.borrow();
                cfg.speed_multiplier
            };

            if speed_multiplier <= f32::EPSILON {
                return;
            }

            let area = ctx.area;
            if area.width == 0 || area.height == 0 {
                return;
            }

            let delta = ctx.last_tick.as_secs_f32() * (0.65 + speed_multiplier);
            state.phase = (state.phase + delta * 2.8) % (std::f32::consts::TAU * 4.0);
            state.scroll = state.scroll.wrapping_add(
                ((ctx.last_tick.as_millis() as f32 * (0.8 + speed_multiplier)) as u32).max(1),
            );

            let palette = state.palette.borrow();

            // Solid black backdrop before drawing the foreground digits.
            for y in area.y..area.bottom() {
                for x in area.x..area.right() {
                    let cell = &mut buf[(x, y)];
                    cell.set_symbol(" ");
                    cell.set_fg(Color::Black);
                    cell.set_bg(Color::Black);
                }
            }

            // Foreground: the coiling "snake" made from sequence digits.
            let center_x = area.x as f32 + area.width as f32 / 2.0;
            let center_y = area.y as f32 + area.height as f32 / 2.0;
            let length = ((area.width as usize + area.height as usize) * 2).clamp(16, 240);
            let radius_x = area.width.max(4) as f32 * 0.48;
            let radius_y = area.height.max(3) as f32 * 0.42;

            for i in 0..length {
                let t = i as f32 / length as f32;
                let angle = state.phase + t * std::f32::consts::TAU * 1.4;
                let slither = (state.phase * 0.6 + t * 14.0).sin();
                let wobble = (state.phase * 0.35 + t * 10.0).cos();

                let x = center_x
                    + angle.cos() * (radius_x * (0.65 + t * 0.5))
                    + slither * 1.4
                    + (t * 8.0).sin();
                let y =
                    center_y + angle.sin() * (radius_y * (0.75 + (1.0 - t) * 0.2)) + wobble * 0.9;

                let xi = x.round() as i32;
                let yi = y.round() as i32;
                if xi < area.x as i32
                    || xi >= area.right() as i32
                    || yi < area.y as i32
                    || yi >= area.bottom() as i32
                {
                    continue;
                }

                let seq_idx = (i + state.scroll as usize / 3) % SEQUENCE_POOL.len();
                let seq = SEQUENCE_POOL[seq_idx];
                let digit = sequence_digit(seq, i + state.scroll as usize);
                let head_mix = (1.0 - t).powf(0.35).clamp(0.12, 1.0);

                let cell = &mut buf[(xi as u16, yi as u16)];
                cell.set_symbol(digit_symbol(digit));
                cell.set_fg(lerp_color(palette.trace, palette.head, head_mix));
                cell.set_bg(palette.background_dim);
            }
        },
    )
    .with_filter(CellFilter::Not(Box::new(CellFilter::RefArea(
        exclusion_area.clone(),
    ))));

    fx::never_complete(snake_fx)
}
