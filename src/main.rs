use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use ignore::WalkBuilder;

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

    let raw = get_paths(&cli.path)
        .into_iter()
        .filter_map(|path| {
            let (name, color) = languages::detect(&path)?;
            let lines = count_lines(&path).ok()?;
            Some((name.to_string(), color, lines))
        })
        .collect::<Vec<(String, (u8, u8, u8), usize)>>();

    let mut totals: HashMap<String, (u8, u8, u8, usize)> = HashMap::new();
    for (name, color, lines) in raw {
        let entry = totals.entry(name).or_insert((color.0, color.1, color.2, 0));
        entry.3 += lines;
    }

    let total_lines: usize = totals.values().map(|(_, _, _, l)| l).sum();
    if total_lines == 0 {
        return Ok(());
    }

    let mut entries: Vec<(&str, usize, (u8, u8, u8))> = totals
        .iter()
        .map(|(n, (r, g, b, l))| (n.as_str(), *l, (*r, *g, *b)))
        .collect();
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
