use colored::*;
use tokei::LanguageType;
use crate::languages::lang_color;

pub const BAR_CHAR: &str = "█";
pub const LIST_CHAR: &str = "█";
pub const SEP_CHAR: &str = "━";
pub const BAR_WIDTH: usize = 50;

pub struct Mode {
    pub show_bar: bool,
    pub show_header: bool,
    pub show_lines: bool,
    pub show_pct: bool,
}

pub fn display(entries: &[(LanguageType, usize)], total: usize, mode: &Mode) {
    if mode.show_bar {
        print_bar(entries, total);
    }

    if !mode.show_lines && !mode.show_pct {
        return;
    }

    let max_name = entries.iter().map(|(l, _)| format!("{:?}", l).len()).max().unwrap_or(0).max("Language".len());
    let line_width = if mode.show_lines {
        entries.iter().map(|(_, l)| l).max().unwrap_or(&0).to_string().len().max("Lines".len())
    } else {
        0
    };
    let pct_width = if mode.show_pct {
        entries.iter().map(|(_, l)| format!("{:.1}%", *l as f64 / total as f64).len()).max().unwrap_or(0).max("Percentage".len())
    } else {
        0
    };

    if mode.show_header {
        print_header(max_name, line_width, pct_width, mode);
    }

    for (lang, lines) in entries {
        let pct = (*lines as f64 / total as f64) * 100.0;
        let (r, g, b) = lang_color(lang);
        let name = format!("{:?}", lang);
        print_row(name, *lines, pct, max_name, line_width, pct_width, mode, r, g, b);
    }
}

fn print_header(name_w: usize, line_w: usize, pct_w: usize, mode: &Mode) {
    let mut cols = vec![format!("{:name_w$}", "Language", name_w = name_w)];
    if mode.show_lines {
        cols.push(format!("{:line_w$}", "Lines", line_w = line_w));
    }
    if mode.show_pct {
        cols.push(format!("{:pct_w$}", "Percentage", pct_w = pct_w));
    }
    if mode.show_bar {
        println!("   {}", cols.join("  "));
    } else {
        println!("{}", cols.join("  "));
    }
}

fn print_row(name: String, lines: usize, pct: f64, name_w: usize, line_w: usize, pct_w: usize, mode: &Mode, r: u8, g: u8, b: u8) {
    let mut cols = vec![format!("{:name_w$}", name, name_w = name_w)];
    if mode.show_lines {
        cols.push(format!("{:<line_w$}", lines, line_w = line_w));
    }
    if mode.show_pct {
        cols.push(format!("{:pct_w$}", format!("{:.1}%", pct), pct_w = pct_w));
    }
    if mode.show_bar {
        println!("{}  {}", LIST_CHAR.truecolor(r, g, b), cols.join("  "));
    } else {
        println!("{}", cols.join("  "));
    }
}

fn print_bar(entries: &[(LanguageType, usize)], total: usize) {
    for (lang, lines) in entries {
        let count = ((*lines as f64 / total as f64) * BAR_WIDTH as f64).round() as usize;
        let (r, g, b) = lang_color(lang);
        for _ in 0..count {
            print!("{}", BAR_CHAR.truecolor(r, g, b));
        }
    }
    println!();
    println!("{}", SEP_CHAR.repeat(BAR_WIDTH));
}
