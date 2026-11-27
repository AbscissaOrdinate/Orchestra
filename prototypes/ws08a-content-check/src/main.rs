// Usage: ws08a-content-check <content_dir> <scenario_id>
// Prints every validation error (ARC-051) and exits non-zero if there are any.
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: ws08a-content-check <content_dir> <scenario_id>");
        return ExitCode::from(2);
    }
    match ws08a_content_check::load(Path::new(&args[1]), &args[2]) {
        Ok(c) => {
            println!(
                "ok: scenario {} — {} regions, {} edges, {} polities",
                c.scenario.id,
                c.regions.len(),
                c.edges.len(),
                c.polities.len()
            );
            ExitCode::SUCCESS
        }
        Err(errors) => {
            for e in &errors {
                eprintln!("error: {e}");
            }
            eprintln!("{} error(s)", errors.len());
            ExitCode::FAILURE
        }
    }
}
