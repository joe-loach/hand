use std::{io::Write, path::PathBuf, sync::Arc};

use anyhow::Context as _;
use clap::Parser;
use codespan_reporting::{files::SimpleFile, term};

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

    let source_text =
        std::fs::read_to_string(&cli.file_path).context("reading input file failed")?;

    let source_text: Arc<str> = Arc::from(source_text);

    let file = SimpleFile::new(cli.file_path.display().to_string(), source_text.clone());

    let matcher = asm::build_matcher();

    let binary = asm::assemble(source_text.clone(), &matcher);

    let binary = match binary {
        Ok(bin) => bin,
        Err(asm_errors) => {
            let diagnostics = asm_errors.report().collect::<Vec<_>>();

            let mut writer =
                term::termcolor::BufferedStandardStream::stderr(term::termcolor::ColorChoice::Auto);
            let config = term::Config::default();

            for diag in diagnostics {
                term::emit(&mut writer, &config, &file, &diag)?;
            }

            return Ok(());
        }
    };

    let output = cli
        .output
        .unwrap_or_else(|| cli.file_path.with_extension("o"));

    let file = std::fs::File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output)
        .context("opening output file failed")?;

    let mut file = std::io::BufWriter::new(file);

    file.write_all(&binary)
        .context("writing binary file failed")?;

    Ok(())
}
