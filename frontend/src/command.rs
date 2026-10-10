use leptos::prelude::*;

use crate::state::{BgMode, Lang, LogLine, Mode, Page, PortfolioData, Store, Theme};

pub enum RunResult {
    Output(Vec<LogLine>),
    Cleared,
}

// ─────────────────────────────────────────────────────────────────────────────
//  Lexing
// ─────────────────────────────────────────────────────────────────────────────

pub fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut started = false;
    for ch in s.chars() {
        match ch {
            '\'' if !in_double => {
                in_single = !in_single;
                started = true;
            }
            '"' if !in_single => {
                in_double = !in_double;
                started = true;
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if started || !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            c => {
                cur.push(c);
                started = true;
            }
        }
    }
    if started || !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn split_pipes(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut in_double = false;
    for ch in s.chars() {
        match ch {
            '\'' if !in_double => {
                in_single = !in_single;
                cur.push(ch);
            }
            '"' if !in_single => {
                in_double = !in_double;
                cur.push(ch);
            }
            '|' if !in_single && !in_double => out.push(std::mem::take(&mut cur)),
            c => cur.push(c),
        }
    }
    out.push(cur);
    out
}

fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn lines<I, S>(v: I) -> Vec<LogLine>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    v.into_iter().map(LogLine::out).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
//  Public entry point
// ─────────────────────────────────────────────────────────────────────────────

pub fn run(store: Store, input: &str) -> RunResult {
    let input = input.trim();
    if input.is_empty() {
        return RunResult::Output(Vec::new());
    }

    let segments = split_pipes(input);
    let first = tokenize(segments.first().map(|s| s.as_str()).unwrap_or(""));
    if segments.len() == 1 && first.first().map(|s| s.as_str()) == Some("clear") {
        store.log.set(Vec::new());
        return RunResult::Cleared;
    }

    let mut stdin: Vec<String> = Vec::new();
    let mut result: Vec<LogLine> = Vec::new();
    for seg in segments.iter() {
        let toks = tokenize(seg);
        if toks.is_empty() {
            continue;
        }
        match execute(&toks, &stdin, store) {
            Ok(log_out) => {
                stdin = log_out.iter().map(|l| l.text.clone()).collect();
                result = log_out;
            }
            Err(e) => return RunResult::Output(vec![LogLine::err(e)]),
        }
    }
    RunResult::Output(result)
}

fn execute(tokens: &[String], stdin: &[String], store: Store) -> Result<Vec<LogLine>, String> {
    let cmd = tokens[0].as_str();
    let args = &tokens[1..];
    let data = store.data.get_untracked();

    match cmd {
        "help" => Ok(lines(help_text())),
        "man" => Ok(lines(man_page(args.first().map(|s| s.as_str()).unwrap_or("")))),
        "whoami" => Ok(vec![LogLine::out(format!("{} — {}", data.name, data.role))]),
        "pwd" => Ok(vec![LogLine::out(format!(
            "/home/tatsuya/{}",
            store.page.get_untracked().slug()
        ))]),
        "echo" => Ok(vec![LogLine::out(args.join(" "))]),
        "date" => Ok(vec![LogLine::out(now_string())]),
        "ls" => Ok(lines(ls(&args.join(" "), &data))),
        "tree" => Ok(lines(tree(&data))),
        "cat" => cat(args, stdin, &data),
        "grep" => Ok(grep(args, stdin)),
        "wc" => Ok(wc(args, stdin)),
        "head" => Ok(head_tail(args, stdin, false)),
        "tail" => Ok(head_tail(args, stdin, true)),
        "sort" => {
            let mut l = stdin.to_vec();
            l.sort();
            Ok(lines(l))
        }
        "uniq" => {
            let mut out: Vec<String> = Vec::new();
            for line in stdin {
                if out.last() != Some(line) {
                    out.push(line.clone());
                }
            }
            Ok(lines(out))
        }
        "cowsay" => Ok(lines(cowsay(&join_input(args, stdin)))),
        "cd" => cd(args.first().map(|s| s.as_str()).unwrap_or(""), store),
        "open" => open(args, &data),
        "theme" => theme(args, store),
        "bg" => bg(args, store),
        "lang" => lang(args, store),
        "mode" => mode_cmd(args, store),
        "history" => Ok(store
            .history
            .get_untracked()
            .iter()
            .enumerate()
            .map(|(i, c)| LogLine::out(format!("{:>4}  {}", i + 1, c)))
            .collect()),
        "uname" => {
            if args.iter().any(|a| a == "-a") {
                Ok(vec![LogLine::out(
                    "Linux portfolio 6.18.33-1-lts #1 SMP PREEMPT_DYNAMIC wasm64 GNU/Linux",
                )])
            } else {
                Ok(vec![LogLine::out("Linux")])
            }
        }
        "fastfetch" | "neofetch" => Ok(fastfetch(&data)),
        "ssh" => {
            if args.first().map(|s| s.as_str()) == Some("contact@tatsuya") {
                Ok(vec![
                    LogLine::out(format!("GitHub:   {}", data.contact.github)),
                    LogLine::out(format!("LinkedIn: {}", data.contact.linkedin)),
                    LogLine::out(format!("Email:    {}", data.contact.email)),
                    LogLine::out(format!("ORCID:    {}", data.contact.orcid)),
                ])
            } else {
                Err("ssh: connect to host port 22: Connection refused".into())
            }
        }
        "skills" => {
            store.page.set(Page::Skills);
            Ok(vec![LogLine::ok("cd ~/skills")])
        }
        "projects" => {
            store.page.set(Page::Projects);
            Ok(vec![LogLine::ok("cd ~/projects")])
        }
        "experience" => {
            store.page.set(Page::Experience);
            Ok(vec![LogLine::ok("cd ~/experience")])
        }
        "research" => {
            store.page.set(Page::Research);
            Ok(vec![LogLine::ok("cd ~/research")])
        }
        "contact" => {
            store.page.set(Page::Contact);
            Ok(vec![LogLine::ok("cd ~/contact")])
        }
        "sudo" => sudo(&tokens.join(" ")),
        "sl" => {
            store.train.set(true);
            let _ = leptos::prelude::set_timeout(
                move || store.train.set(false),
                std::time::Duration::from_millis(4200),
            );
            Ok(vec![LogLine::info("🚂  choo choo — you are in the wrong lane")])
        }
        "cmatrix" => {
            store.theme.set(Theme::Matrix);
            Ok(vec![
                LogLine::out("Wake up, Neo..."),
                LogLine::out("The Matrix has you..."),
                LogLine::out("Follow the white rabbit."),
            ])
        }
        "coffee" | "tea" => Ok(lines(vec![
            "    (  )   (  )",
            "     ) (    ) (",
            "   ___________",
            "  |           | )",
            "  |  COFFEE   | |",
            "  |           | )",
            "  |___________|/",
            "Freshly brewed British tea (or coffee) is served!",
        ])),
        "secret" => Ok(vec![LogLine::ok("🔓 Achievement unlocked: Shell Adept")]),
        "exit" => Ok(vec![LogLine::info("Session ended. Refresh to restart.")]),
        _ => Err(format!("{}: command not found", cmd)),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
//  Command implementations
// ─────────────────────────────────────────────────────────────────────────────

fn join_input(args: &[String], stdin: &[String]) -> String {
    if !args.is_empty() {
        args.join(" ")
    } else {
        stdin.join(" ")
    }
}

fn now_string() -> String {
    let d = js_sys::Date::new_0();
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        d.get_full_year(),
        d.get_month() + 1,
        d.get_date(),
        d.get_hours(),
        d.get_minutes(),
        d.get_seconds()
    )
}

fn help_text() -> Vec<&'static str> {
    vec![
        "PortfolioOS shell — a small, sharp, modal toolkit.",
        "",
        "MODES     i -> INSERT (shell)   : -> COMMAND   esc -> NORMAL   ? -> help",
        "NAV       h / l  or  <- / ->  switch page      1-6  jump page",
        "FILES     ls [-la] [dir]   cat <file>   tree   pwd   cd <dir>",
        "FILTERS   every command composes with pipes:  cat bio.txt | wc -w",
        "          grep <pat>   wc [-lwc]   head [-n N]   tail [-n N]   sort   uniq",
        "UTIL      whoami   uname -a   fastfetch   date   history   echo",
        "THEME     theme [tokyo|matrix|dracula]",
        "VIEW      bg [aurora|stars|grid|mesh|none]   lang [en|ja|fr|de|zh|ko|it]",
        "FUN       sl   cmatrix   coffee   cowsay   sudo   secret",
    ]
}

fn man_page(name: &str) -> Vec<String> {
    let body: Vec<&str> = match name {
        "cat" => vec![
            "NAME       cat - concatenate files and print on standard output",
            "SYNOPSIS   cat [file]",
            "DESCRIPTION",
            "       Reads virtual files: bio.txt, skills.json, education.md,",
            "       awards.md, publications.md, research/<slug>.md",
            "EXAMPLES   cat bio.txt | wc -w",
        ],
        "ls" => vec![
            "NAME       ls - list directory contents",
            "SYNOPSIS   ls [-a] [-l] [dir]",
            "OPTIONS    -a  include hidden entries (.secret_vault)",
            "           -l  long listing (permissions, owner, name)",
        ],
        "cd" => vec![
            "NAME       cd - change the visible page",
            "SYNOPSIS   cd [home|skills|projects|experience|research|contact]",
        ],
        "grep" => vec![
            "NAME       grep - print lines matching a pattern",
            "SYNOPSIS   ... | grep [-i] <pattern>",
        ],
        "wc" => vec![
            "NAME       wc - print newline, word and byte counts",
            "SYNOPSIS   ... | wc [-l] [-w] [-c]",
        ],
        "theme" => vec![
            "NAME       theme - set the colour palette",
            "SYNOPSIS   theme [tokyo|matrix|dracula]",
        ],
        "bg" => vec![
            "NAME       bg - set the aurora background",
            "SYNOPSIS   bg [aurora|stars|grid|mesh|none]",
        ],
        "pipes" | "pipe" => vec![
            "NAME       pipe - connect stdout of one command to stdin of the next",
            "EXAMPLES   cat bio.txt | grep Rust",
        ],
        "" => vec!["What manual page do you want?  man <command>"],
        _ => vec![
            "NAME       man - display the manual page for a command",
            "DESCRIPTION",
            "       No manual entry available.",
            "       Try: man cat | ls | cd | grep | wc | theme | bg | pipes",
        ],
    };
    let mut out = vec![format!("{}(1)  PortfolioOS Programmer's Manual  {}(1)", name.to_uppercase(), name.to_uppercase()), String::new()];
    out.extend(body.into_iter().map(|s| s.to_string()));
    out
}

fn home_entries(show_hidden: bool) -> Vec<String> {
    let mut v: Vec<String> = vec![
        "about/",
        "projects/",
        "research/",
        "skills/",
        "experience/",
        "contact/",
        "bio.txt",
        "skills.json",
        "education.md",
        "awards.md",
        "publications.md",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();
    if show_hidden {
        v.insert(0, ".secret_vault".to_string());
    }
    v
}

fn ls(argstr: &str, data: &PortfolioData) -> Vec<String> {
    let toks = tokenize(argstr);
    let show_all = toks.iter().any(|t| t == "-a" || t == "-la" || t == "-al");
    let long = toks.iter().any(|t| t.starts_with('-') && t.contains('l'));
    let path = toks
        .iter()
        .find(|t| !t.starts_with('-'))
        .cloned()
        .unwrap_or_default();
    let dir = path.trim().trim_end_matches('/');

    match dir {
        "projects" => {
            let mut v = vec![".".to_string(), "..".to_string()];
            v.extend(data.projects.iter().map(|p| p.name.clone()));
            v
        }
        "research" => {
            let mut v = vec![".".to_string(), "..".to_string()];
            v.extend(data.research.iter().map(|r| r.title.clone()));
            v
        }
        "about" => vec![".".into(), "..".into(), "education.md".into(), "awards.md".into(), "publications.md".into()],
        _ if long => {
            let mut v: Vec<String> = Vec::new();
            if show_all {
                v.push("drwxr-xr-x  tatsuya  .secret_vault".into());
            }
            v.extend(
                [
                    "drwxr-xr-x  tatsuya  about/",
                    "drwxr-xr-x  tatsuya  projects/",
                    "drwxr-xr-x  tatsuya  research/",
                    "drwxr-xr-x  tatsuya  skills/",
                    "-rw-r--r--  tatsuya  bio.txt",
                    "-rw-r--r--  tatsuya  skills.json",
                ]
                .into_iter()
                .map(|s| s.to_string()),
            );
            v
        }
        _ => home_entries(show_all),
    }
}

fn tree(data: &PortfolioData) -> Vec<String> {
    let mut v: Vec<String> = vec![
        "~".into(),
        "├── about/".into(),
        "│   ├── education.md".into(),
        "│   ├── awards.md".into(),
        "│   └── publications.md".into(),
        "├── projects/".into(),
    ];
    for p in &data.projects {
        v.push(format!("│   ├── {}", slugify(&p.name)));
    }
    v.push("├── research/".into());
    for r in &data.research {
        v.push(format!("│   ├── {}.md", slugify(&r.title)));
    }
    v.extend([
        "├── skills.json".to_string(),
        "├── bio.txt".to_string(),
        "└── .secret_vault".to_string(),
    ]);
    v
}

fn cat(args: &[String], stdin: &[String], data: &PortfolioData) -> Result<Vec<LogLine>, String> {
    let Some(name) = args.iter().find(|a| !a.starts_with('-')) else {
        if !stdin.is_empty() {
            return Ok(lines(stdin.iter().cloned()));
        }
        return Err("cat: missing operand".into());
    };
    match name.as_str() {
        "bio.txt" => Ok(vec![LogLine::out(&data.bio)]),
        "skills.json" => Ok(vec![LogLine::out(data.skills_json())]),
        "education.md" => {
            let mut v = vec!["# Education".to_string(), String::new()];
            for e in &data.education {
                v.push(format!("- {}  {} — {}", e.period, e.degree, e.institution));
            }
            Ok(lines(v))
        }
        "awards.md" => {
            let mut v = vec!["# Awards".to_string(), String::new()];
            for a in &data.awards {
                v.push(format!("## {} — {}", a.date, a.title));
                v.push(a.desc.clone());
                v.push(String::new());
            }
            Ok(lines(v))
        }
        "publications.md" => {
            if data.publications.is_empty() {
                return Ok(vec![LogLine::info("no publications found (yet)")]);
            }
            let mut v = vec!["# Publications".to_string(), String::new()];
            for p in &data.publications {
                v.push(format!("- ({}) {} — {}", p.year, p.title, p.publisher));
            }
            Ok(lines(v))
        }
        ".secret_vault" => Ok(vec![
            LogLine::ok("🔐 decrypting .secret_vault … ok"),
            LogLine::out("Hint: Essex is one of the top research universities in the UK."),
            LogLine::out("Hint: you found the vault. Try `secret`."),
        ]),
        name if name.starts_with("research/") => {
            let key = slugify(name.trim_start_matches("research/").trim_end_matches(".md"));
            data.research
                .iter()
                .find(|r| slugify(&r.title) == key)
                .map(|r| {
                    vec![
                        LogLine::out(format!("# {}", r.title)),
                        LogLine::out(""),
                        LogLine::out(&r.desc),
                    ]
                })
                .ok_or_else(|| format!("cat: {}: No such file or directory", name))
        }
        name => Err(format!("cat: {}: No such file or directory", name)),
    }
}

fn grep(args: &[String], stdin: &[String]) -> Vec<LogLine> {
    let ignore_case = args.iter().any(|a| a == "-i");
    let pattern = args.iter().find(|a| !a.starts_with('-')).cloned().unwrap_or_default();
    let pat = if ignore_case { pattern.to_lowercase() } else { pattern.clone() };
    stdin
        .iter()
        .filter(|line| {
            let hay = if ignore_case { line.to_lowercase() } else { (*line).clone() };
            hay.contains(&pat)
        })
        .cloned()
        .map(LogLine::out)
        .collect()
}

fn wc(args: &[String], stdin: &[String]) -> Vec<LogLine> {
    let text = stdin.join("\n");
    let lc = stdin.len();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    match args.iter().find(|a| a.starts_with('-')).map(|s| s.as_str()) {
        Some("-l") => vec![LogLine::out(format!("{}", lc))],
        Some("-w") => vec![LogLine::out(format!("{}", words))],
        Some("-c") => vec![LogLine::out(format!("{}", chars))],
        _ => vec![LogLine::out(format!("{} {} {}", lc, words, chars))],
    }
}

fn head_tail(args: &[String], stdin: &[String], tail: bool) -> Vec<LogLine> {
    let mut n = 10usize;
    if let Some(pos) = args.iter().position(|a| a == "-n") {
        if let Some(v) = args.get(pos + 1).and_then(|s| s.parse::<usize>().ok()) {
            n = v;
        }
    }
    let slice: Vec<String> = if tail {
        let start = stdin.len().saturating_sub(n);
        stdin[start..].to_vec()
    } else {
        stdin.iter().take(n).cloned().collect()
    };
    slice.into_iter().map(LogLine::out).collect()
}

fn cowsay(_msg: &str) -> Vec<&'static str> {
    vec![
        " _________________________________________",
        "< beep boop — portfolio generated by Rust >",
        " -----------------------------------------",
        "        \\   ^__^",
        "         \\  (oo)\\_______",
        "            (__)\\       )\\/\\",
        "                ||----w |",
        "                ||     ||",
    ]
}

fn cd(target: &str, store: Store) -> Result<Vec<LogLine>, String> {
    if target.is_empty() || target == "~" {
        store.page.set(Page::Home);
        return Ok(vec![LogLine::ok("cd ~/home")]);
    }
    match Page::from_slug(target) {
        Some(p) => {
            store.page.set(p);
            Ok(vec![LogLine::ok(format!("cd ~/{}", p.slug()))])
        }
        None => Err(format!("cd: no such file or directory: {}", target)),
    }
}

fn open(args: &[String], data: &PortfolioData) -> Result<Vec<LogLine>, String> {
    let Some(key) = args.first() else {
        return Err("open: usage: open [github|linkedin|email|orcid|<project>]".into());
    };
    let url = match key.as_str() {
        "github" => data.contact.github.clone(),
        "linkedin" => data.contact.linkedin.clone(),
        "email" => format!("mailto:{}", data.contact.email),
        "orcid" => data.contact.orcid.clone(),
        name => data
            .projects
            .iter()
            .find(|p| slugify(&p.name) == slugify(name))
            .map(|p| p.link.clone())
            .ok_or_else(|| format!("open: {}: not found", name))?,
    };
    open_url(&url);
    Ok(vec![LogLine::ok(format!("opened {}", url))])
}

pub fn open_url(url: &str) {
    if let Some(win) = web_sys::window() {
        let _ = win.open_with_url(url);
    }
}

fn theme(args: &[String], store: Store) -> Result<Vec<LogLine>, String> {
    match args.first().and_then(|s| Theme::from_slug(s)) {
        Some(t) => {
            store.theme.set(t);
            Ok(vec![LogLine::ok(format!("theme → {}", t.slug()))])
        }
        None => Err(format!(
            "theme: available: {}",
            Theme::ALL.iter().map(|t| t.slug()).collect::<Vec<_>>().join(", ")
        )),
    }
}

fn bg(args: &[String], store: Store) -> Result<Vec<LogLine>, String> {
    match args.first().and_then(|s| BgMode::from_slug(s)) {
        Some(b) => {
            store.bg.set(b);
            Ok(vec![LogLine::ok(format!("bg → {}", b.slug()))])
        }
        None => Err(format!(
            "bg: available: {}",
            BgMode::ALL.iter().map(|b| b.slug()).collect::<Vec<_>>().join(", ")
        )),
    }
}

fn lang(args: &[String], store: Store) -> Result<Vec<LogLine>, String> {
    match args.first().and_then(|s| Lang::from_code(s)) {
        Some(l) => {
            store.lang.set(l);
            Ok(vec![LogLine::ok(format!("lang → {}", l.locale()))])
        }
        None => Err("lang: available: en, ja, fr, de, zh, ko, it".into()),
    }
}

fn mode_cmd(args: &[String], store: Store) -> Result<Vec<LogLine>, String> {
    match args.first().map(|s| s.as_str()) {
        Some("normal") => {
            store.mode.set(Mode::Normal);
            Ok(vec![LogLine::ok("-- NORMAL --")])
        }
        Some("insert") => {
            store.mode.set(Mode::Insert);
            Ok(vec![LogLine::ok("-- INSERT --")])
        }
        Some("command") => {
            store.mode.set(Mode::Command);
            Ok(vec![LogLine::ok("-- COMMAND --")])
        }
        _ => Ok(vec![LogLine::info(format!(
            "mode: {} (try `mode insert`)",
            store.mode.get_untracked().label()
        ))]),
    }
}

fn sudo(cmd: &str) -> Result<Vec<LogLine>, String> {
    if cmd.contains("rm -rf /") {
        return Ok(vec![
            LogLine::err("Steady on! You can't just delete the entire mainframe, mate."),
            LogLine::info("That's not very British of you. How about a cup of tea? ☕"),
        ]);
    }
    if cmd.contains("pacman") {
        return Ok(vec![
            LogLine::out(":: Synchronizing package databases..."),
            LogLine::out(" core    160.2 KiB  [######################] 100%"),
            LogLine::out(" extra  1012.4 KiB  [######################] 100%"),
            LogLine::out(":: Starting full system upgrade..."),
            LogLine::ok("there is nothing to do — Arch is life."),
        ]);
    }
    Err("sudo: permission denied".into())
}

fn fastfetch(data: &PortfolioData) -> Vec<LogLine> {
    let host = format!("{}@dev", data.name);
    let line = "-".repeat(host.len());
    let art = [
        "                   -`",
        "                  .o+`",
        "                 `ooo/",
        "                `+oooo:",
        "               `+oooooo:",
        "               -+oooooo+:",
        "             `/:-:++oooo+:",
        "            `/++++/+++++++:",
        "           `/++++++++++++++:",
        "          `/+++ooooooooooooo/`",
        "         ./ooosssso++osssssso+`",
        "        .oossssso-````/ossssss+`",
        "       -osssssso.      :ssssssso.",
        "      :osssssss/        osssso+++.",
        "     /ossssssss/        +ssssooo/-",
        "   `/ossssso+/:-        -:/+osssso+-",
        "  `+sso+:-`                 `.-/+oso:",
        " `++:.                           `-/+/",
        " .`                                 `",
    ];
    let info = [
        host.clone(),
        line,
        "OS: Arch Linux x86_64".to_string(),
        format!("Host: {}-IdeaPad Slim 3", data.name),
        "Kernel: 6.18.33-1-lts (wasm64)".to_string(),
        "Shell: ghostty 1.3.1-arch2".to_string(),
        "WM: Hyprland".to_string(),
        "Theme: PortfolioOS / aurora".to_string(),
        "CPU: AMD Ryzen 7 7735HS with Radeon Graphics (16)".to_string(),
        "GPU: AMD ATI Radeon 680M".to_string(),
        format!("Role: {}", data.role),
    ];
    let rows = art.len().max(info.len());
    let mut out = Vec::with_capacity(rows);
    for i in 0..rows {
        let left = art.get(i).copied().unwrap_or("                                        ");
        let right = info.get(i).map(|s| s.as_str()).unwrap_or("");
        out.push(LogLine::out(format!("{:<40} {}", left, right)));
    }
    out
}
