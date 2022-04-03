use browserslist::{resolve, Opts};
use clap::{Parser, Subcommand};
use glob::glob;
use parcel_css::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use parcel_css::targets::Browsers;
use serde_json::{json, Value};
use std::error::Error;
use std::{fmt, fs, path::PathBuf};

/// 4 in 5 sardines recommend this CLI
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[clap(short, long, value_name = "FILE")]
    config: Option<String>,

    /// Turn debugging information on
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds your project
    Build {
        /// The relative path to a file
        #[clap(short, long, value_name = "FILE")]
        file: Option<String>,
        /// Destination file for the output
        #[clap(short, long)]
        output_file: Option<PathBuf>,
        /// The relative path to the source directory
        #[clap(short = 's', long, value_name = "DIR")]
        dir: Option<String>,
        /// Destination directory for the output
        #[clap(short = 'd', long)]
        output_dir: Option<PathBuf>,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Args::parse();

    // load configuration file
    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }
    // enable debug mode
    if cli.debug {
        println!("verbose: {:?}", cli.debug);
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Build {
            file,
            output_file,
            dir,
            output_dir,
        }) => {
            // reads and build an individual css file
            if let Some(file_path) = file {
                if let Some(file_path_out) = output_file {
                    match build_css(&PathBuf::from(file_path), file_path_out) {
                        Ok(css_modules_map) => println!("{}", css_modules_map),
                        Err(e) => println!("{:?}", e),
                    }
                }
            }

            // reads and builds all css files in a directory
            if let Some(src_dir) = dir.as_deref() {
                let dir = &fmt::format(format_args!("{}/**/*.css", src_dir));
                if let Some(dist) = output_dir {
                    for entry in glob(dir).expect("Failed to read glob pattern") {
                        match entry {
                            Ok(file_path) => {
                                println!("{:?}", file_path);
                                match build_css(&file_path, dist) {
                                    Ok(css_modules_map) => println!("{}", css_modules_map),
                                    Err(e) => println!("{:?}", e),
                                }
                            }
                            Err(e) => println!("{:?}", e),
                        }
                    }
                 }

            }
        }

        None => {}
    }
    Ok(())
}

fn build_css(path_to_file: &PathBuf, path_to_output: &PathBuf) -> Result<Value, Box<dyn Error>> {
    let src_file_path = path_to_file.to_str().unwrap();
    let mut output_file_path = PathBuf::new();
    output_file_path.push(path_to_output);

    if !path_to_output.is_file() {
        output_file_path.push(path_to_file);
    }
    println!("Dist {:?}", output_file_path.display());
    println!("Source {:?}", path_to_file.display());

    let source = fs::read_to_string(path_to_file);
    let contents = source.unwrap();

    let options = ParserOptions {
        nesting: true,
        css_modules: src_file_path.contains(".module.css"),
        ..ParserOptions::default()
    };
    let mut stylesheet = StyleSheet::parse(src_file_path.into(), &contents, options).unwrap();

    let targets = browserslist_to_targets(vec!["chrome 98".to_string()]).unwrap();
    stylesheet
        .minify(MinifyOptions {
            targets,
            ..MinifyOptions::default()
        })
        .unwrap();

    let res = stylesheet
        .to_css(PrinterOptions {
            targets,
            minify: true,
            ..PrinterOptions::default()
        })
        .unwrap();
    let code = res.code;

    // creates all sub-directories if they don't exist yet
    if let Some(parent_dir) = output_file_path.parent() {
        fs::create_dir_all(parent_dir)?;
    };

    // writes file to disk
    fs::write(output_file_path, code.as_bytes())?;

    // generates a map for the CSS Modules
    let modules_map = json!(res.exports);
    Ok(modules_map)
}

// This should from package.json
fn browserslist_to_targets(query: Vec<String>) -> Result<Option<Browsers>, browserslist::Error> {
    if query.is_empty() {
        return Ok(None);
    }

    let res = resolve(query, &Opts::new())?;

    let mut browsers = Browsers::default();
    let mut has_any = false;
    for distrib in res {
        macro_rules! browser {
            ($browser: ident) => {{
                if let Some(v) = parse_version(distrib.version()) {
                    if browsers.$browser.is_none() || v < browsers.$browser.unwrap() {
                        browsers.$browser = Some(v);
                        has_any = true;
                    }
                }
            }};
        }

        match distrib.name() {
            "android" => browser!(android),
            "chrome" | "and_chr" => browser!(chrome),
            "edge" => browser!(edge),
            "firefox" | "and_ff" => browser!(firefox),
            "ie" => browser!(ie),
            "ios_saf" => browser!(ios_saf),
            "opera" | "op_mob" => browser!(opera),
            "safari" => browser!(safari),
            "samsung" => browser!(samsung),
            _ => {}
        }
    }

    if !has_any {
        return Ok(None);
    }

    Ok(Some(browsers))
}

// will remove soon
fn parse_version(version: &str) -> Option<u32> {
    let version = version.split('-').next();
    version?;

    let mut version = version.unwrap().split('.');
    let major = version.next().and_then(|v| v.parse::<u32>().ok());
    if let Some(major) = major {
        let minor = version
            .next()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0);
        let patch = version
            .next()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0);
        let v: u32 = (major & 0xff) << 16 | (minor & 0xff) << 8 | (patch & 0xff);
        return Some(v);
    }

    None
}
