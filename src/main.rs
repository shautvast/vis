use anyhow::anyhow;
use std::env::args;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: vis vis-file"));
    } else {
        let vis_file = read_file(&args[1])?;
        let vis = vis::parser::parse_vis(vis_file.as_str())?;
        println!("{:?}", vis);
    }

    Ok(())
}

fn read_file(file_name: &str) -> anyhow::Result<String> {
    fs::read_to_string(file_name).map_err(|e| anyhow!("Cannot read file '{}': {}", file_name, e))
}
