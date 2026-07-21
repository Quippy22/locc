use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use ignore::WalkBuilder;
use tokei::{Config, LanguageType};

mod args;
mod display;
mod languages;

fn get_paths(root: &str) -> Vec<PathBuf> {
    WalkBuilder::new(root)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().map_or(false, |ft| ft.is_dir()))
        .filter(|e| e.depth() > 0)
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn count_lines(path: &PathBuf) -> io::Result<usize> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

fn main() -> io::Result<()> {
    let cli = args::Args::parse();

    let raw: Vec<(LanguageType, usize)> = get_paths(&cli.path)
        .into_iter()
        .filter_map(|path| {
            let lang = LanguageType::from_path(&path, &Config::default())?;
            if languages::SKIP_TYPES.contains(&lang) {
                return None;
            }
            let lines = count_lines(&path).ok()?;
            Some((lang, lines))
        })
        .collect();

    let mut totals: HashMap<LanguageType, usize> = HashMap::new();
    for (lang, lines) in raw {
        *totals.entry(lang).or_insert(0) += lines;
    }

    let total_lines: usize = totals.values().sum();
    if total_lines == 0 {
        return Ok(());
    }

    let mut entries: Vec<(LanguageType, usize)> = totals.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));

    let only_graph = cli.graph && !cli.short && !cli.lines && !cli.detailed;

    let mode = display::Mode {
        show_bar: cli.graph || cli.detailed,
        show_header: cli.detailed,
        show_lines: !only_graph && (!cli.short || cli.lines),
        show_pct: !only_graph && (!cli.lines || cli.short),
    };

    display::display(&entries, total_lines, &mode);

    Ok(())
}
