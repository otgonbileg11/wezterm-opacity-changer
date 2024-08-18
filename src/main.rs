use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Opacity of the terminal you want to set
    #[arg(short, long)]
    opacity: Option<f32>,
}

fn replace_opacity_in_file<P: AsRef<Path>>(file_path: P, opacity: Option<f32>) -> io::Result<()> {
    // Read the file line by line
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    // Create a temporary file to write the changes
    let mut temp_file = File::create("temp.md")?;

    for line in reader.lines() {
        let line = line?;
        let new_line = if let Some(opacity_value) = opacity {
            if let Some((before, after)) = line.split_once("config.window_background_opacity = ") {
                format!(
                    "{}config.window_background_opacity = {}{}",
                    before,
                    opacity_value,
                    after
                    .chars()
                    .skip_while(|c| c.is_digit(10) || *c == '.' || *c == ' ')
                    .collect::<String>()
                )
            } else if line.contains("config.window_background_opacity = 1") {
                line.replace("config.window_background_opacity = 1", "config.window_background_opacity = 0.9")
            } else if line.contains("config.window_background_opacity = ") {
                let parts: Vec<&str> = line.split("config.window_background_opacity = ").collect();
                format!(
                    "{}config.window_background_opacity = 1{}",
                    parts[0],
                    parts[1]
                    .chars()
                    .skip_while(|c| c.is_digit(10) || *c == '.' || *c == ' ')
                    .collect::<String>()
                )
            } else {
                line
            } 
        } else if line.contains("config.window_background_opacity = 1") {
                line.replace("config.window_background_opacity = 1", "config.window_background_opacity = 0.9")
            } else if line.contains("config.window_background_opacity = ") {
                let parts: Vec<&str> = line.split("config.window_background_opacity = ").collect();
                format!(
                    "{}config.window_background_opacity = 1{}",
                    parts[0],
                    parts[1]
                    .chars()
                    .skip_while(|c| c.is_digit(10) || *c == '.' || *c == ' ')
                    .collect::<String>()
                )
            } else {
                line
            };
        writeln!(temp_file, "{}", new_line)?;
    }

    // Replace the original file with the modified one
    fs::rename("temp.md", file_path)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = "/home/nt-oogii/.wezterm.lua";
    let args = Args::parse();
    replace_opacity_in_file(file_path, args.opacity)?;
    Ok(())
}
