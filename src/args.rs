use clap::Parser;

#[derive(Parser)]
#[command(name = "locc", version, about = "Count lines of code per language")]
pub struct Args {
    #[arg(long, help = "Show language and percentage only")]
    pub short: bool,

    #[arg(long, help = "Show everything: graph, header, language, lines, percentage")]
    pub detailed: bool,

    #[arg(long, help = "Show language breakdown bar")]
    pub graph: bool,

    #[arg(long, help = "Show language and lines only")]
    pub lines: bool,

    #[arg(help = "Directory to scan", default_value = ".")]
    pub path: String,
}
