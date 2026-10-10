use leptos::prelude::*;

use crate::state::{Page, PortfolioData, Store};
use crate::ui::{chip, tile, NowPlaying, Typewriter, ARCH_LOGO};

// ─────────────────────────────────────────────────────────────────────────────
//  Home — the dashboard
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Home(store: Store) -> impl IntoView {
    let d = store.data.get();
    let name = d.name.clone();
    let role = d.role.clone();
    let bio = d.bio.clone();
    let github = d.contact.github.clone();
    let email = d.contact.email.clone();
    let education = d.education.clone();
    let awards = d.awards.clone();
    let hobbies = d.hobbies.clone();
    let publications = d.publications.clone();
    let show_pubs = publications.len() > 0;

    let ff = fastfetch_view(&d);

    view! {
        <section class="page page-home">
            <div class="bento">
                {tile(
                    "span-2 row-2 hero",
                    "$ whoami",
                    "~/home",
                    view! {
                        <div class="hero">
                            <div class="avatar-wrap">
                                <img class="avatar" src="/favicon.png" alt="Tatsuya Miura" />
                            </div>
                            <div class="hero-text">
                                <h1 class="hero-name">
                                    <Typewriter
                                        text=Signal::derive(move || store.data.get().name)
                                        speed_ms=60
                                    />
                                </h1>
                                <p class="hero-role">{role.clone()}</p>
                                <p class="hero-status">
                                    <span class="pulse"></span>
                                    "available for interesting projects"
                                </p>
                                <div class="hero-actions">
                                    <a class="btn neu" href=github target="_blank" rel="noopener">
                                        "github"
                                    </a>
                                    <a class="btn neu" href=format!("mailto:{email}")>"email"</a>
                                </div>
                                <p class="hero-keys">"i insert · : command · ? help · h/l pages"</p>
                            </div>
                        </div>
                    },
                )}
                {tile(
                    "span-2 ff-tile",
                    "$ fastfetch",
                    &format!("{name}@dev"),
                    ff,
                )}
                {tile(
                    "span-2 bio-tile",
                    "$ cat bio.txt",
                    "~/bio.txt",
                    view! { <p class="prose">{bio}</p> },
                )}
                {tile(
                    "span-1",
                    "$ cat education.md",
                    "~/education",
                    view! {
                        <ul class="stack">
                            {education
                                .into_iter()
                                .map(|e| {
                                    view! {
                                        <li class="stack-item">
                                            <span class="stack-when">{e.period}</span>
                                            <span class="stack-what">{e.degree}</span>
                                            <span class="stack-where">{e.institution}</span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    },
                )}
                {tile(
                    "span-1",
                    "$ cat awards.md",
                    "~/awards",
                    view! {
                        <ul class="stack">
                            {awards
                                .into_iter()
                                .map(|a| {
                                    view! {
                                        <li class="stack-item">
                                            <span class="stack-when">{a.date}</span>
                                            <span class="stack-what">{a.title}</span>
                                            <span class="stack-where">{a.desc}</span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    },
                )}
                {tile(
                    "span-1",
                    "$ ls hobbies/",
                    "~/hobbies",
                    view! {
                        <div class="chips">
                            {hobbies.into_iter().map(chip).collect_view()}
                        </div>
                    },
                )}
                {tile(
                    "span-1 now-tile",
                    "$ nowplaying",
                    "~/status",
                    view! { <NowPlaying /> },
                )}
                <Show when=move || show_pubs>
                    {tile(
                        "span-2",
                        "$ cat publications.md",
                        "~/publications",
                        view! {
                            <ul class="stack">
                                {publications
                                    .clone()
                                    .into_iter()
                                    .map(|p| {
                                        view! {
                                            <li class="stack-item">
                                                <span class="stack-when">{p.year}</span>
                                                <span class="stack-what">{p.title}</span>
                                                <span class="stack-where">{p.publisher}</span>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                        },
                    )}
                </Show>
                {tile(
                    "span-1 nav-tile",
                    "$ cd ~/",
                    "shortcuts",
                    view! {
                        <div class="shortcuts">
                            <button class="btn ghost" on:click=move |_| store.page.set(Page::Skills)>
                                "~/skills"
                            </button>
                            <button class="btn ghost" on:click=move |_| store.page.set(Page::Projects)>
                                "~/projects"
                            </button>
                            <button class="btn ghost" on:click=move |_| store.page.set(Page::Research)>
                                "~/research"
                            </button>
                            <button class="btn ghost" on:click=move |_| store.page.set(Page::Contact)>
                                "~/contact"
                            </button>
                        </div>
                    },
                )}
            </div>
        </section>
    }
}

fn fastfetch_view(d: &PortfolioData) -> impl IntoView {
    let rows = vec![
        ("OS", "Arch Linux x86_64".to_string()),
        ("Host", format!("{}-IdeaPad Slim 3", d.name)),
        ("Kernel", "6.18.33-1-lts".to_string()),
        ("Shell", "ghostty".to_string()),
        ("WM", "Hyprland".to_string()),
        ("Uptime", "since 2026".to_string()),
        ("CPU", "Ryzen 7 7735HS".to_string()),
        ("GPU", "Radeon 680M".to_string()),
        ("Role", d.role.clone()),
    ];
    view! {
        <div class="ff">
            <pre class="ascii">{ARCH_LOGO}</pre>
            <dl class="kv">
                {rows
                    .into_iter()
                    .map(|(k, v)| {
                        view! {
                            <div class="kv-row">
                                <dt>{k}</dt>
                                <dd>{v}</dd>
                            </div>
                        }
                    })
                    .collect_view()}
            </dl>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Skills
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Skills(store: Store) -> impl IntoView {
    let d = store.data.get();
    let tree = skill_tree(&d);
    let cats: Vec<(String, Vec<String>)> = d
        .skills
        .categories()
        .into_iter()
        .map(|(name, items)| (name.to_string(), items.to_vec()))
        .collect();

    view! {
        <section class="page page-skills">
            <div class="bento">
                {tile(
                    "span-2 tree-tile",
                    "$ tree ~/skills",
                    "~/skills",
                    view! { <pre class="tree-text">{tree}</pre> },
                )}
                {cats
                    .into_iter()
                    .map(|(name, items)| {
                        let heading = name.clone();
                        tile(
                            "span-1",
                            "$ cat skills.json",
                            &format!("~/{name}"),
                            view! {
                                <h3 class="cat-name">{heading}</h3>
                                <div class="chips">
                                    {items.into_iter().map(chip).collect_view()}
                                </div>
                            },
                        )
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

fn skill_tree(d: &PortfolioData) -> String {
    let cats = d.skills.categories();
    let mut s = String::from("~/skills\n");
    let mut files = 0usize;
    for (i, (cat, items)) in cats.iter().enumerate() {
        let last = i == cats.len() - 1;
        s.push_str(&format!("{}── {}/\n", if last { "└" } else { "├" }, cat));
        let indent = if last { "    " } else { "│   " };
        for (j, item) in items.iter().enumerate() {
            let last_item = j == items.len() - 1;
            s.push_str(&format!(
                "{}{}── {}\n",
                indent,
                if last_item { "└" } else { "├" },
                item
            ));
            files += 1;
        }
    }
    s.push_str(&format!("\n{} directories, {} files\n", cats.len(), files));
    s
}

// ─────────────────────────────────────────────────────────────────────────────
//  Projects
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Projects(store: Store) -> impl IntoView {
    let d = store.data.get();
    let projects = d.projects.clone();

    view! {
        <section class="page page-projects">
            <div class="bento">
                {tile(
                    "span-4 ls-tile",
                    "$ ls -la ~/projects",
                    "~/projects",
                    view! {
                        <div class="table">
                            <div class="tr tr-head">
                                <span class="td perms">"permissions"</span>
                                <span class="td name">"name"</span>
                                <span class="td tech">"tech"</span>
                            </div>
                            {projects
                                .iter()
                                .map(|p| {
                                    view! {
                                        <div class="tr">
                                            <span class="td perms">"drwxr-xr-x"</span>
                                            <span class="td name">{p.name.clone()}</span>
                                            <span class="td tech">{p.tech.join(" · ")}</span>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    },
                )}
                {projects
                    .iter()
                    .map(|p| {
                        let name = p.name.clone();
                        let desc = p.desc.clone();
                        let link = p.link.clone();
                        let tech = p.tech.clone();
                        tile(
                            "span-2 proj-tile",
                            "$ cat README.md",
                            &format!("~/{name}"),
                            view! {
                                <h3 class="proj-name">{name}</h3>
                                <p class="prose">{desc}</p>
                                <div class="chips">
                                    {tech.into_iter().map(chip).collect_view()}
                                </div>
                                <a class="btn neu" href=link target="_blank" rel="noopener">
                                    "view repository ↗"
                                </a>
                            },
                        )
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Experience
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Experience(store: Store) -> impl IntoView {
    let d = store.data.get();
    let experiences = d.experiences.clone();
    let skills_json = d.skills_json();

    view! {
        <section class="page page-experience">
            <div class="bento">
                {tile(
                    "span-2 log-tile",
                    "$ journalctl -u career.service",
                    "~/experience",
                    view! {
                        <div class="log">
                            {experiences
                                .into_iter()
                                .map(|e| {
                                    view! {
                                        <div class="log-entry">
                                            <span class="log-line">
                                                <span class="ts">{e.period}</span>
                                                <span class="lvl">"INFO"</span>
                                                <span class="msg">{e.role}</span>
                                            </span>
                                            <span class="log-sub">"Company: " {e.company}</span>
                                            <span class="log-desc">{e.desc}</span>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    },
                )}
                {tile(
                    "span-2 json-tile",
                    "$ cat skills.json",
                    "~/skills.json",
                    view! { <pre class="json">{skills_json}</pre> },
                )}
            </div>
        </section>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Research
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Research(store: Store) -> impl IntoView {
    let d = store.data.get();
    let research = d.research.clone();

    view! {
        <section class="page page-research">
            <div class="bento">
                {research
                    .into_iter()
                    .enumerate()
                    .map(|(i, r)| {
                        let title = r.title.clone();
                        let desc = r.desc.clone();
                        tile(
                            "span-2 research-tile",
                            "$ cat *.md",
                            &format!("~/research/{:02}", i + 1),
                            view! {
                                <span class="index">{format!("{:02}", i + 1)}</span>
                                <h3 class="research-name">{title}</h3>
                                <p class="prose">{desc}</p>
                                <span class="tag">"distributed · memory-safe · systems"</span>
                            },
                        )
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Contact
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn Contact(store: Store) -> impl IntoView {
    let d = store.data.get();
    let c = d.contact.clone();
    let name = d.name.clone();
    let role = d.role.clone();

    let links: Vec<(&str, &str, String)> = vec![
        ("github", "GitHub", c.github.clone()),
        ("linkedin", "LinkedIn", c.linkedin.clone()),
        ("email", "Email", format!("mailto:{}", c.email)),
        ("orcid", "ORCID", c.orcid.clone()),
    ];

    view! {
        <section class="page page-contact">
            <div class="bento">
                {tile(
                    "span-2 finger-tile",
                    "$ finger tatsuya",
                    "~/contact",
                    view! {
                        <div class="finger">
                            <div class="finger-row">
                                <span class="finger-k">"Login:"</span>
                                <span class="finger-v">"tatsuya"</span>
                            </div>
                            <div class="finger-row">
                                <span class="finger-k">"Name:"</span>
                                <span class="finger-v">{name}</span>
                            </div>
                            <div class="finger-row">
                                <span class="finger-k">"Role:"</span>
                                <span class="finger-v">{role}</span>
                            </div>
                            <div class="finger-row">
                                <span class="finger-k">"Directory:"</span>
                                <span class="finger-v">"~/contact"</span>
                            </div>
                            <div class="finger-row">
                                <span class="finger-k">"Shell:"</span>
                                <span class="finger-v">"/bin/portfolio-wasm"</span>
                            </div>
                            <div class="finger-row">
                                <span class="finger-k">"Location:"</span>
                                <span class="finger-v">"Colchester, UK / Tokyo, JP"</span>
                            </div>
                            <div class="finger-row">
                                <span class="finger-k">"Status:"</span>
                                <span class="finger-v ok">"available for interesting projects"</span>
                            </div>
                            <div class="finger-row">
                                <span class="finger-k">"Plan:"</span>
                                <span class="finger-v">"Graduate from Essex with first-class honours."</span>
                            </div>
                        </div>
                    },
                )}
                {links
                    .into_iter()
                    .map(|(_key, label, href)| {
                        view! {
                            <a
                                class="link-tile neu"
                                href=href
                                target="_blank"
                                rel="noopener"
                            >
                                <span class="link-label">{label}</span>
                                <span class="link-arrow">"↗"</span>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
