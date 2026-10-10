mod command;
mod pages;
mod state;
mod ui;

use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use state::{Mode, Page, Store};
use ui::{Backdrop, Drawer, HelpOverlay, StatusLine, TopBar, Train};

#[component]
fn App() -> impl IntoView {
    let store = Store::new();
    provide_context(store);

    // ── modal keybindings ───────────────────────────────────────────────────
    window_event_listener(ev::keydown, move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        let is_input = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some();

        if key == "Escape" {
            if store.help.get_untracked() {
                store.help.set(false);
                return;
            }
            if store.mode.get_untracked() != Mode::Normal {
                store.mode.set(Mode::Normal);
                return;
            }
            if store.drawer.get_untracked() {
                store.drawer.set(false);
            }
            return;
        }
        if is_input {
            return;
        }

        if store.mode.get_untracked() == Mode::Normal && !store.help.get_untracked() {
            match key.as_str() {
                "i" => store.mode.set(Mode::Insert),
                ":" => store.mode.set(Mode::Command),
                "?" => store.help.update(|h| *h = !*h),
                "h" | "ArrowLeft" => store.page.update(|p| *p = p.shifted(-1)),
                "l" | "ArrowRight" => store.page.update(|p| *p = p.shifted(1)),
                "g" => store.page.set(Page::Home),
                "G" => store.page.set(Page::Contact),
                "1" => store.page.set(Page::ALL[0]),
                "2" => store.page.set(Page::ALL[1]),
                "3" => store.page.set(Page::ALL[2]),
                "4" => store.page.set(Page::ALL[3]),
                "5" => store.page.set(Page::ALL[4]),
                "6" => store.page.set(Page::ALL[5]),
                _ => {}
            }
        }
    });

    // ── browser back/forward ────────────────────────────────────────────────
    window_event_listener(ev::hashchange, move |_ev: web_sys::HashChangeEvent| {
        if let Some(hash) = location_hash() {
            if let Some(page) = Page::from_slug(&hash) {
                store.page.set(page);
            }
        }
    });

    view! {
        <div class="app" class:drawer-open=move || store.drawer.get()>
            <Backdrop />
            <TopBar store=store />
            <main class="stage">
                {move || {
                    let _ = store.data.get();
                    match store.page.get() {
                        Page::Home => view! { <pages::Home store=store /> }.into_any(),
                        Page::Skills => view! { <pages::Skills store=store /> }.into_any(),
                        Page::Projects => view! { <pages::Projects store=store /> }.into_any(),
                        Page::Experience => view! { <pages::Experience store=store /> }.into_any(),
                        Page::Research => view! { <pages::Research store=store /> }.into_any(),
                        Page::Contact => view! { <pages::Contact store=store /> }.into_any(),
                    }
                }}
                <footer class="brandfoot">
                    "© 2026 Tatsuya-PortfolioOS v4.0 — Rust → WASM (Leptos) && Hono && Bun && Cloudflare"
                </footer>
            </main>
            <Train store=store />
            <Drawer store=store />
            <StatusLine store=store />
            <HelpOverlay store=store />
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> });
}
