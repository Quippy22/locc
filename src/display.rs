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

fn ansi_fg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

const RESET: &str = "\x1b[0m";

fn colored(text: &str, r: u8, g: u8, b: u8) -> String {
    format!("{}{}{}", ansi_fg(r, g, b), text, RESET)
}

pub fn display(entries: &[(&str, usize, (u8, u8, u8))], total: usize, mode: &Mode) {
    if mode.show_bar {
        print_bar(entries, total);
    }

    if !mode.show_lines && !mode.show_pct {
        return;
    }

    let max_name = entries
        .iter()
        .map(|(n, _, _)| n.len())
        .max()
        .unwrap_or(0)
        .max("Language".len());
    let line_width = if mode.show_lines {
        entries
            .iter()
            .map(|(_, l, _)| l)
            .max()
            .unwrap_or(&0)
            .to_string()
            .len()
            .max("Lines".len())
    } else {
        0
    };
    let pct_width = if mode.show_pct {
        entries
            .iter()
            .map(|(_, l, _)| format!("{:.1}%", *l as f64 / total as f64).len())
            .max()
            .unwrap_or(0)
            .max("Percentage".len())
    } else {
        0
    };

    if mode.show_header {
        print_header(max_name, line_width, pct_width, mode);
    }

    for &(name, lines, (r, g, b)) in entries {
        let pct = lines as f64 / total as f64 * 100.0;
        print_row(
            name, lines, pct, max_name, line_width, pct_width, mode, r, g, b,
        );
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

fn print_row(
    name: &str,
    lines: usize,
    pct: f64,
    name_w: usize,
    line_w: usize,
    pct_w: usize,
    mode: &Mode,
    r: u8,
    g: u8,
    b: u8,
) {
    let mut cols = vec![format!("{:name_w$}", name, name_w = name_w)];
    if mode.show_lines {
        cols.push(format!("{:<line_w$}", lines, line_w = line_w));
    }
    if mode.show_pct {
        cols.push(format!("{:pct_w$}", format!("{:.1}%", pct), pct_w = pct_w));
    }
    if mode.show_bar {
        println!("{}  {}", colored(LIST_CHAR, r, g, b), cols.join("  "));
    } else {
        println!("{}", cols.join("  "));
    }
}

fn print_bar(entries: &[(&str, usize, (u8, u8, u8))], total: usize) {
    for &(_, lines, (r, g, b)) in entries {
        let count = (lines as f64 / total as f64 * BAR_WIDTH as f64).round() as usize;
        for _ in 0..count {
            print!("{}", colored(BAR_CHAR, r, g, b));
        }
    }
    println!();
    println!("{}", SEP_CHAR.repeat(BAR_WIDTH));
}
