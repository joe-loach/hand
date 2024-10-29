use std::{io::Write, path::PathBuf, sync::Arc};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file to assemble
    #[arg(value_name = "INPUT_FILE")]
    file_path: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let source_text = std::fs::read_to_string(&cli.file_path)?;
    let source_text = Arc::from(source_text);

    let binary = asm::assemble(source_text);

    let output = cli
        .output
        .unwrap_or_else(|| cli.file_path.with_extension("o"));

    let file = std::fs::File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output)?;
    let mut file = std::io::BufWriter::new(file);

    file.write_all(&binary)?;

    Ok(())
}
