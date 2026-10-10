use leptos::prelude::*;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

// ─────────────────────────────────────────────────────────────────────────────
//  Content model (mirrors the locale JSON CMS)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Contact {
    pub github: String,
    #[serde(rename = "LinkedIn")]
    pub linkedin: String,
    pub email: String,
    pub orcid: String,
    pub twitter: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Project {
    pub name: String,
    pub tech: Vec<String>,
    pub desc: String,
    pub link: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Experience {
    pub period: String,
    pub role: String,
    pub company: String,
    pub desc: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Research {
    pub title: String,
    pub desc: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Education {
    pub period: String,
    pub degree: String,
    pub institution: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Award {
    pub date: String,
    pub title: String,
    pub desc: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Publication {
    pub year: String,
    pub title: String,
    pub publisher: String,
    pub link: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Skills {
    pub languages: Vec<String>,
    pub frontend: Vec<String>,
    pub backend: Vec<String>,
    #[serde(rename = "Infrastructure")]
    pub infrastructure: Vec<String>,
}

impl Skills {
    pub fn categories(&self) -> [(&'static str, &[String]); 4] {
        [
            ("languages", &self.languages),
            ("frontend", &self.frontend),
            ("backend", &self.backend),
            ("Infrastructure", &self.infrastructure),
        ]
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct PortfolioData {
    pub name: String,
    pub role: String,
    pub bio: String,
    pub skills: Skills,
    pub projects: Vec<Project>,
    pub experiences: Vec<Experience>,
    pub research: Vec<Research>,
    pub education: Vec<Education>,
    pub awards: Vec<Award>,
    pub publications: Vec<Publication>,
    pub hobbies: Vec<String>,
    pub contact: Contact,
}

impl PortfolioData {
    /// English fallback used for the very first paint (before the locale JSON lands).
    pub fn fallback() -> Self {
        Self {
            name: "Tatsuya Miura".into(),
            role: "Full-Stack Engineer".into(),
            bio: "Architecting applications with Rust & WebAssembly and IoT systems with Arduino. Keen on memory-safe systems and distributed architectures.".into(),
            contact: Contact {
                github: "https://github.com/TatsuyaM2667".into(),
                linkedin: "https://www.linkedin.com/in/tatsuya-miura-1bb48339a/".into(),
                email: "tatsuyamiura2667@gmail.com".into(),
                orcid: "https://orcid.org/0009-0007-5480-2657".into(),
                twitter: String::new(),
            },
            ..Default::default()
        }
    }

    pub fn skills_json(&self) -> String {
        let v = serde_json::json!({
            "languages": self.skills.languages,
            "frontend": self.skills.frontend,
            "backend": self.skills.backend,
            "Infrastructure": self.skills.infrastructure,
        });
        serde_json::to_string_pretty(&v).unwrap_or_else(|_| "{}".into())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Enumerations
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

impl Mode {
    pub fn label(self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Command => "COMMAND",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Home,
    Skills,
    Projects,
    Experience,
    Research,
    Contact,
}

impl Page {
    pub const ALL: [Page; 6] = [
        Page::Home,
        Page::Skills,
        Page::Projects,
        Page::Experience,
        Page::Research,
        Page::Contact,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            Page::Home => "home",
            Page::Skills => "skills",
            Page::Projects => "projects",
            Page::Experience => "experience",
            Page::Research => "research",
            Page::Contact => "contact",
        }
    }

    pub fn from_slug(s: &str) -> Option<Page> {
        let s = s.trim().trim_start_matches("~/").trim_matches('/');
        match s {
            "" | "~" | "home" => Some(Page::Home),
            "skills" => Some(Page::Skills),
            "projects" => Some(Page::Projects),
            "experience" => Some(Page::Experience),
            "research" => Some(Page::Research),
            "contact" => Some(Page::Contact),
            _ => None,
        }
    }

    pub fn index(self) -> usize {
        Page::ALL.iter().position(|p| *p == self).unwrap_or(0)
    }

    pub fn shifted(self, delta: isize) -> Page {
        let n = Page::ALL.len() as isize;
        let i = (self.index() as isize + delta).rem_euclid(n);
        Page::ALL[i as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Tokyo,
    Matrix,
    Dracula,
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::Tokyo, Theme::Matrix, Theme::Dracula];

    pub fn slug(self) -> &'static str {
        match self {
            Theme::Tokyo => "tokyo",
            Theme::Matrix => "matrix",
            Theme::Dracula => "dracula",
        }
    }

    pub fn from_slug(s: &str) -> Option<Theme> {
        match s {
            "tokyo" => Some(Theme::Tokyo),
            "matrix" => Some(Theme::Matrix),
            "dracula" => Some(Theme::Dracula),
            _ => None,
        }
    }

    pub fn next(self) -> Theme {
        let i = Theme::ALL.iter().position(|t| *t == self).unwrap_or(0);
        Theme::ALL[(i + 1) % Theme::ALL.len()]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BgMode {
    Aurora,
    Stars,
    Grid,
    Mesh,
    None,
}

impl BgMode {
    pub const ALL: [BgMode; 5] = [
        BgMode::Aurora,
        BgMode::Stars,
        BgMode::Grid,
        BgMode::Mesh,
        BgMode::None,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            BgMode::Aurora => "aurora",
            BgMode::Stars => "stars",
            BgMode::Grid => "grid",
            BgMode::Mesh => "mesh",
            BgMode::None => "none",
        }
    }

    pub fn from_slug(s: &str) -> Option<BgMode> {
        match s {
            "aurora" => Some(BgMode::Aurora),
            "stars" => Some(BgMode::Stars),
            "grid" => Some(BgMode::Grid),
            "mesh" => Some(BgMode::Mesh),
            "none" => Some(BgMode::None),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    En,
    Ja,
    Fr,
    De,
    Zh,
    Ko,
    It,
}

impl Lang {
    pub const ALL: [Lang; 7] = [
        Lang::En,
        Lang::Ja,
        Lang::Fr,
        Lang::De,
        Lang::Zh,
        Lang::Ko,
        Lang::It,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Ja => "ja",
            Lang::Fr => "fr",
            Lang::De => "de",
            Lang::Zh => "zh",
            Lang::Ko => "ko",
            Lang::It => "it",
        }
    }

    pub fn locale(self) -> &'static str {
        match self {
            Lang::En => "en_US.UTF-8",
            Lang::Ja => "ja_JP.UTF-8",
            Lang::Fr => "fr_FR.UTF-8",
            Lang::De => "de_DE.UTF-8",
            Lang::Zh => "zh_CN.UTF-8",
            Lang::Ko => "ko_KR.UTF-8",
            Lang::It => "it_IT.UTF-8",
        }
    }

    pub fn from_code(s: &str) -> Option<Lang> {
        let s = s.trim_start_matches("file://").to_lowercase();
        let s = s.split(['.', '_', '-']).next().unwrap_or("");
        match s {
            "en" => Some(Lang::En),
            "ja" => Some(Lang::Ja),
            "fr" => Some(Lang::Fr),
            "de" => Some(Lang::De),
            "zh" => Some(Lang::Zh),
            "ko" => Some(Lang::Ko),
            "it" => Some(Lang::It),
            _ => None,
        }
    }

    fn detect() -> Lang {
        web_sys::window()
            .and_then(|w| w.navigator().language())
            .and_then(|l| Lang::from_code(&l))
            .unwrap_or(Lang::En)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Terminal scrollback
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogKind {
    Cmd,
    Out,
    Err,
    Info,
    Dim,
    Ok,
}

#[derive(Clone, Debug)]
pub struct LogLine {
    pub text: String,
    pub kind: LogKind,
}

impl LogLine {
    pub fn out(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: LogKind::Out,
        }
    }
    pub fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: LogKind::Info,
        }
    }
    pub fn err(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: LogKind::Err,
        }
    }
    pub fn ok(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: LogKind::Ok,
        }
    }
    pub fn dim(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: LogKind::Dim,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Global store
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Store {
    pub mode: RwSignal<Mode>,
    pub page: RwSignal<Page>,
    pub theme: RwSignal<Theme>,
    pub bg: RwSignal<BgMode>,
    pub lang: RwSignal<Lang>,
    pub data: RwSignal<PortfolioData>,
    pub log: RwSignal<Vec<LogLine>>,
    pub history: RwSignal<Vec<String>>,
    pub hist_idx: RwSignal<usize>,
    pub help: RwSignal<bool>,
    pub drawer: RwSignal<bool>,
    pub train: RwSignal<bool>,
    pub clock: RwSignal<String>,
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
}

fn load_pref(key: &str) -> Option<String> {
    local_storage().and_then(|s| s.get_item(key).ok().flatten())
}

pub fn save_pref(key: &str, value: &str) {
    if let Some(s) = local_storage() {
        let _ = s.set_item(key, value);
    }
}

pub async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(win) = web_sys::window() {
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                resolve.unchecked_ref::<js_sys::Function>(),
                ms,
            );
        }
    });
    let _ = JsFuture::from(promise).await;
}

pub async fn fetch_text(url: &str) -> Option<String> {
    let win = web_sys::window()?;
    let promise = win.fetch_with_str(url);
    let resp_val = JsFuture::from(promise).await.ok()?;
    let resp: web_sys::Response = resp_val.dyn_into().ok()?;
    if !resp.ok() {
        return None;
    }
    let text_val = JsFuture::from(resp.text().ok()?).await.ok()?;
    text_val.as_string()
}

fn load_locale(store: Store, lang: Lang) {
    spawn_local(async move {
        if let Some(body) = fetch_text(&format!("/locales/{}.json", lang.code())).await {
            if let Ok(parsed) = serde_json::from_str::<PortfolioData>(&body) {
                store.data.set(parsed);
            }
        }
    });
}

impl Store {
    pub fn new() -> Self {
        let theme = load_pref("portfolio.theme")
            .and_then(|s| Theme::from_slug(&s))
            .unwrap_or(Theme::Tokyo);
        let bg = load_pref("portfolio.bg")
            .and_then(|s| BgMode::from_slug(&s))
            .unwrap_or(BgMode::Aurora);
        let lang = load_pref("portfolio.lang")
            .and_then(|s| Lang::from_code(&s))
            .unwrap_or_else(Lang::detect);
        let page = location_hash()
            .as_deref()
            .and_then(Page::from_slug)
            .unwrap_or(Page::Home);

        let store = Store {
            mode: RwSignal::new(Mode::Normal),
            page: RwSignal::new(page),
            theme: RwSignal::new(theme),
            bg: RwSignal::new(bg),
            lang: RwSignal::new(lang),
            data: RwSignal::new(PortfolioData::fallback()),
            log: RwSignal::new(vec![
                LogLine::info("PortfolioOS v4.0 — WASM edition (Leptos + Rust)"),
                LogLine::dim("press i to type, : for commands, ? for help".to_string()),
            ]),
            history: RwSignal::new(Vec::new()),
            hist_idx: RwSignal::new(0),
            help: RwSignal::new(false),
            drawer: RwSignal::new(true),
            train: RwSignal::new(false),
            clock: RwSignal::new(String::new()),
        };

        // mirror theme / bg / lang onto <html> so CSS can hook into them
        Effect::new(move |_| {
            let t = store.theme.get();
            if let Some(el) = document().document_element() {
                let _ = el.set_attribute("data-theme", t.slug());
            }
            save_pref("portfolio.theme", t.slug());
        });
        Effect::new(move |_| {
            let b = store.bg.get();
            if let Some(el) = document().document_element() {
                let _ = el.set_attribute("data-bg", b.slug());
            }
            save_pref("portfolio.bg", b.slug());
        });
        Effect::new(move |_| {
            let l = store.lang.get();
            if let Some(el) = document().document_element() {
                let _ = el.set_attribute("lang", l.code());
            }
            save_pref("portfolio.lang", l.code());
            load_locale(store, l);
        });

        // hash routing: keep the URL in sync so deep links work
        Effect::new(move |_| {
            let slug = store.page.get().slug();
            if location_hash().as_deref() != Some(slug) {
                let _ = win_location().set_hash(&format!("#/{}", slug));
            }
        });

        // clock in the statusline
        set_interval(
            move || {
                let d = js_sys::Date::new_0();
                store.clock.set(format!(
                    "{:02}:{:02}:{:02}",
                    d.get_hours(),
                    d.get_minutes(),
                    d.get_seconds()
                ));
            },
            std::time::Duration::from_millis(1000),
        );

        store
    }
}

pub fn win_location() -> web_sys::Location {
    web_sys::window().unwrap().location()
}
