use leptos::html;
use leptos::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;

use crate::command::{self, RunResult};
use crate::state::{fetch_text, sleep, Lang, LogKind, LogLine, Mode, Page, Store};

pub const ARCH_LOGO: &str = r#"      /\
     /  \
    /\   \
   /      \
  /   ,,   \
 /   |  |  -\
/_-''    ''-_\ "#;

pub const TRAIN: &str = r#"
      ====        ________                ___________
  _D _|  |_ ______|_  ____|_  _________  |_  _______|_
 |   |____| |      |_|    |_| |      |_| |_|       |_|
 |___________|______|______|______|______|___________|
  oo          oo          oo          oo          oo
"#;

fn kind_class(kind: LogKind) -> &'static str {
    match kind {
        LogKind::Cmd => "ln ln-cmd",
        LogKind::Err => "ln ln-err",
        LogKind::Info => "ln ln-info",
        LogKind::Dim => "ln ln-dim",
        LogKind::Ok => "ln ln-ok",
        LogKind::Out => "ln ln-out",
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Small building blocks
// ─────────────────────────────────────────────────────────────────────────────

pub fn tile(class: &str, cmd: &str, path: &str, body: impl IntoView) -> impl IntoView {
    let cls = format!("tile {class}");
    let cmd = cmd.to_string();
    let path = path.to_string();
    view! {
        <article class=cls>
            <header class="tile-head">
                <span class="tile-cmd">{cmd}</span>
                <span class="tile-path">{path}</span>
            </header>
            <div class="tile-body">{body}</div>
        </article>
    }
}

pub fn chip(label: impl Into<String>) -> impl IntoView {
    view! { <span class="chip">{label.into()}</span> }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Background
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Backdrop() -> impl IntoView {
    view! {
        <div class="backdrop" aria-hidden="true">
            <div class="blob blob-1"></div>
            <div class="blob blob-2"></div>
            <div class="blob blob-3"></div>
            <div class="blob blob-4"></div>
            <div class="layer layer-stars"></div>
            <div class="layer layer-stars2"></div>
            <div class="layer layer-grid"></div>
            <div class="layer layer-mesh"></div>
            <div class="layer layer-noise"></div>
            <div class="vignette"></div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Top bar
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn TopBar(store: Store) -> impl IntoView {
    let lang_open = RwSignal::new(false);
    view! {
        <header class="topbar">
            <div class="brand">
                <span class="brand-mark">"▲"</span>
                <span class="brand-name">{move || store.data.get().name}</span>
                <span class="brand-path">"~/portfolio"</span>
            </div>
            <nav class="nav">
                {Page::ALL
                    .into_iter()
                    .map(|p| {
                        view! {
                            <button
                                class="nav-pill"
                                class:active=move || store.page.get() == p
                                on:click=move |_| store.page.set(p)
                            >
                                {format!("~/{}", p.slug())}
                            </button>
                        }
                    })
                    .collect_view()}
            </nav>
            <div class="tools">
                <div class="lang-wrap">
                    <button
                        class="tool-btn neu"
                        on:click=move |_| lang_open.update(|o| *o = !*o)
                    >
                        <span class="globe">"◍"</span>
                        <span class="lang-label">{move || store.lang.get().locale()}</span>
                    </button>
                    <Show when=move || lang_open.get()>
                        <div class="lang-menu">
                            {Lang::ALL
                                .into_iter()
                                .map(|l| {
                                    view! {
                                        <button
                                            class="lang-item"
                                            class:active=move || store.lang.get() == l
                                            on:click=move |_| {
                                                store.lang.set(l);
                                                lang_open.set(false);
                                            }
                                        >
                                            {l.locale()}
                                        </button>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </Show>
                </div>
                <button
                    class="tool-btn neu"
                    title="cycle theme"
                    on:click=move |_| store.theme.update(|t| *t = t.next())
                >
                    "◐"
                </button>
            </div>
        </header>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Status line
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn StatusLine(store: Store) -> impl IntoView {
    view! {
        <footer class="statusline">
            <span
                class="seg seg-mode"
                class:is-normal=move || store.mode.get() == Mode::Normal
                class:is-insert=move || store.mode.get() == Mode::Insert
                class:is-command=move || store.mode.get() == Mode::Command
            >
                {move || store.mode.get().label()}
            </span>
            <span class="seg">"~/" {move || store.page.get().slug()}</span>
            <span class="seg">
                "[" {move || store.page.get().index() + 1} "/6]"
            </span>
            <span class="seg seg-grow"></span>
            <span class="seg seg-opt">{move || format!("LANG {}", store.lang.get().code())}</span>
            <span class="seg seg-opt">{move || format!("THEME {}", store.theme.get().slug())}</span>
            <span class="seg seg-opt">{move || format!("BG {}", store.bg.get().slug())}</span>
            <span class="seg seg-clock">{move || store.clock.get()}</span>
            <span class="seg seg-hint">"? help"</span>
        </footer>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Terminal drawer
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Drawer(store: Store) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    let scroll_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {
        let mode = store.mode.get();
        if matches!(mode, Mode::Insert | Mode::Command) {
            store.drawer.set(true);
            let _ = set_timeout(
                move || {
                    if let Some(el) = input_ref.get() {
                        let _ = el.focus();
                        let len = el.value().len() as u32;
                        let _ = el.set_selection_range(len, len);
                    }
                },
                Duration::from_millis(40),
            );
        } else if let Some(el) = input_ref.get() {
            let _ = el.blur();
        }
    });

    Effect::new(move |_| {
        let _ = store.log.get().len();
        let _ = set_timeout(
            move || {
                if let Some(el) = scroll_ref.get() {
                    el.set_scroll_top(el.scroll_height());
                }
            },
            Duration::from_millis(30),
        );
    });

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let Some(el) = input_ref.get() else { return };
        let line = el.value().trim().to_string();
        if line.is_empty() {
            return;
        }
        el.set_value("");
        store.history.update(|h| h.push(line.clone()));
        store.hist_idx.set(0);
        let sigil = if store.mode.get_untracked() == Mode::Command {
            ":"
        } else {
            "$"
        };
        match command::run(store, &line) {
            RunResult::Output(out) => {
                store.log.update(|l| {
                    l.push(LogLine {
                        text: format!("{sigil} {line}"),
                        kind: LogKind::Cmd,
                    });
                    l.extend(out);
                    if l.len() > 400 {
                        let drain = l.len() - 400;
                        l.drain(0..drain);
                    }
                });
            }
            RunResult::Cleared => {}
        }
        if store.mode.get_untracked() == Mode::Command {
            store.mode.set(Mode::Normal);
        }
    };

    let on_key = move |ev: web_sys::KeyboardEvent| match ev.key().as_str() {
        "ArrowUp" => {
            ev.prevent_default();
            let h = store.history.get_untracked();
            if h.is_empty() {
                return;
            }
            let idx = store.hist_idx.get_untracked();
            if idx < h.len() {
                let n = idx + 1;
                store.hist_idx.set(n);
                if let Some(el) = input_ref.get() {
                    set_input(&el, &h[h.len() - n]);
                }
            }
        }
        "ArrowDown" => {
            ev.prevent_default();
            let h = store.history.get_untracked();
            let idx = store.hist_idx.get_untracked();
            if idx > 0 {
                let n = idx - 1;
                store.hist_idx.set(n);
                if let Some(el) = input_ref.get() {
                    if n == 0 {
                        el.set_value("");
                    } else {
                        set_input(&el, &h[h.len() - n]);
                    }
                }
            }
        }
        _ => {}
    };

    view! {
        <section class="drawer" class:open=move || store.drawer.get()>
            <header class="drawer-head">
                <div class="dots"><i></i><i></i><i></i></div>
                <span class="drawer-title">
                    {move || format!("tatsuya@portfolio: ~/{}", store.page.get().slug())}
                </span>
                <span
                    class="mode-badge"
                    class:is-normal=move || store.mode.get() == Mode::Normal
                    class:is-insert=move || store.mode.get() == Mode::Insert
                    class:is-command=move || store.mode.get() == Mode::Command
                >
                    {move || store.mode.get().label()}
                </span>
                <button
                    class="drawer-x"
                    title="close (esc)"
                    on:click=move |_| {
                        store.drawer.set(false);
                        store.mode.set(Mode::Normal);
                    }
                >
                    "×"
                </button>
            </header>
            <div class="scrollback" node_ref=scroll_ref>
                {move || {
                    store.log
                        .get()
                        .into_iter()
                        .map(|l| {
                            view! { <div class=kind_class(l.kind)>{l.text}</div> }
                        })
                        .collect_view()
                }}
            </div>
            <form class="prompt-row" on:submit=submit>
                <span class="sigil">
                    {move || if store.mode.get() == Mode::Command { ":" } else { "$" }}
                </span>
                <input
                    node_ref=input_ref
                    class="cmd-input"
                    autocomplete="off"
                    autocapitalize="off"
                    spellcheck="false"
                    placeholder=move || {
                        if store.mode.get() == Mode::Command {
                            "command — e.g. cat bio.txt | wc -w"
                        } else {
                            "type a command (esc to return to NORMAL)"
                        }
                    }
                    on:keydown=on_key
                    on:focus=move |_| {
                        if store.mode.get_untracked() == Mode::Normal {
                            store.mode.set(Mode::Insert);
                        }
                    }
                />
            </form>
        </section>
    }
}

fn set_input(el: &web_sys::HtmlInputElement, val: &str) {
    el.set_value(val);
    let len = val.len() as u32;
    let _ = el.set_selection_range(len, len);
}

// ─────────────────────────────────────────────────────────────────────────────
//  Overlays
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn HelpOverlay(store: Store) -> impl IntoView {
    let keys: [(&str, &str); 10] = [
        ("i", "enter INSERT — type into the shell"),
        (":", "enter COMMAND — run a one-liner"),
        ("esc", "back to NORMAL / close drawer"),
        ("h l  ← →", "previous / next page"),
        ("1 – 6", "jump to a page directly"),
        ("g G", "first / last page"),
        ("?", "toggle this help"),
        ("↑ ↓", "shell history"),
        ("cd <dir>", "navigate the virtual filesystem"),
        ("|", "pipe output: cat bio.txt | wc -w"),
    ];
    view! {
        <Show when=move || store.help.get()>
            <div class="overlay" on:click=move |_| store.help.set(false)>
                <div
                    class="overlay-card"
                    on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()
                >
                    <header class="overlay-head">
                        <span class="tile-cmd">"$ man portfolio"</span>
                        <span class="tile-path">"~/.config/normal-mode"</span>
                    </header>
                    <div class="help-grid">
                        {keys
                            .into_iter()
                            .map(|(k, d)| {
                                view! {
                                    <div class="help-row">
                                        <kbd>{k}</kbd>
                                        <span>{d}</span>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                    <p class="help-foot">
                        "PortfolioOS is modal: NORMAL reads keys, INSERT & COMMAND read text. Built with Rust → WASM (Leptos)."
                    </p>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn Train(store: Store) -> impl IntoView {
    view! {
        <Show when=move || store.train.get()>
            <div class="train-overlay">
                <pre class="train">{TRAIN}</pre>
            </div>
        </Show>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Typewriter
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Typewriter(text: Signal<String>, speed_ms: u64) -> impl IntoView {
    let shown = RwSignal::new(String::new());
    Effect::new(move |_| {
        let full = text.get();
        shown.set(String::new());
        let check = text;
        spawn_local(async move {
            let chars: Vec<char> = full.chars().collect();
            let mut acc = String::new();
            for c in chars {
                if check.get_untracked() != full {
                    return;
                }
                acc.push(c);
                shown.set(acc.clone());
                sleep(speed_ms as i32).await;
            }
        });
    });
    view! {
        <span class="typewriter">{move || shown.get()}</span>
        <span class="tw-cursor">"▍"</span>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Now playing (graceful: hides itself when the API is absent)
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn NowPlaying() -> impl IntoView {
    let track = RwSignal::new(None::<(String, String, String)>);
    let alive = Arc::new(AtomicBool::new(true));
    on_cleanup({
        let alive = Arc::clone(&alive);
        move || alive.store(false, Ordering::Relaxed)
    });
    Effect::new(move |_| {
        let alive = Arc::clone(&alive);
        spawn_local(async move {
            while alive.load(Ordering::Relaxed) {
                if let Some(body) = fetch_text("/api/now-playing").await {
                    let parsed = serde_json::from_str::<serde_json::Value>(&body).ok();
                    let found = parsed.and_then(|v| {
                        let item = v.get("item")?;
                        let title = item.get("name")?.as_str()?.to_string();
                        let artist = item
                            .get("artists")
                            .and_then(|a| a.get(0))
                            .and_then(|a| a.get("name"))
                            .and_then(|n| n.as_str())
                            .unwrap_or("")
                            .to_string();
                        let url = item
                            .get("external_urls")
                            .and_then(|u| u.get("spotify"))
                            .and_then(|u| u.as_str())
                            .unwrap_or("")
                            .to_string();
                        Some((title, artist, url))
                    });
                    track.set(found);
                }
                sleep(30_000).await;
            }
        });
    });
    view! {
        <Show when=move || track.get().is_some()>
            {move || {
                track
                    .get()
                    .map(|(title, artist, url)| {
                        view! {
                            <a class="now-playing" href=url target="_blank" rel="noopener">
                                <span class="np-dot"></span>
                                <span class="np-meta">
                                    <span class="np-title">{title}</span>
                                    <span class="np-artist">{artist}</span>
                                </span>
                                <span class="np-eq"><i></i><i></i><i></i><i></i></span>
                            </a>
                        }
                    })
            }}
        </Show>
    }
}
