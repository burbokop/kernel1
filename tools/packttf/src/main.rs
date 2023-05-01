#![feature(path_file_prefix)]

use std::{
    error::Error,
    path::{PathBuf, Path},
    str::FromStr,
    fmt::Display,
    fs::File,
    io::{Write, Read}, env::args, ffi::OsStr,
};

use clap::{Parser, builder::Str};
use fontdue::{Font, Metrics};
use trim_margin::MarginTrimmable;
use zip::ZipArchive;

#[derive(Clone, Debug, PartialEq)]
enum Format {
    CppInlineHeader,
    Obj
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::CppInlineHeader => f.write_str("cpp-inline-header"),
            Format::Obj => f.write_str("obj"),
        }
    }
}

impl From<String> for Format {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cpp-inline-header" => Format::CppInlineHeader,
            "obj" => Format::Obj,
            _ => panic!("unrecognized format: {}", value)
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to input .ttf or .zip (will be searched for .ttf files) file
    #[arg(short, long)]
    input: String,

    /// Path to output .h file (= ${input}.h if empty)
    #[arg(short, long)]
    output: Option<String>,

    /// Variable name in result file
    #[arg(short, long)]
    name: Option<String>,

    /// Format of output
    #[arg(short, long, default_value_t = Format::CppInlineHeader)]
    format: Format,

    /// Pixel size of glyph
    #[arg(short, long, default_value_t = 7.)]
    size: f32,

    /// x offset of all glyphs (relative value from -1. to 1., positive - move down, negative - move up)
    #[arg(short, long, default_value_t = 0.)]
    x_offset: f32,

    /// y offset of all glyphs (relative value from -1. to 1., positive - move down, negative - move up)
    #[arg(short, long, default_value_t = -0.25)]
    y_offset: f32,
}

fn var_name_from_file(path: PathBuf) -> Result<String, Box<dyn Error>> {
    Ok(path
        .file_prefix()
        .ok_or("file not contains preffix")?
        .to_str()
        .ok_or("file name is not utf8")?
        .replace("-", "_")
    )
}

fn grayscale_aligned(metrics: &Metrics, bitmap: &Vec<u8>, x: usize, y: usize, offset_x: i32, offset_y: i32) -> u8 {
    let x = x as i32 - offset_x + metrics.xmin;
    let y = y as i32 - offset_y + metrics.ymin;

    if x >= 0 && x < metrics.width as i32 && y >= 0 && y < metrics.height as i32 {
        let x = x as usize;
        let y = y as usize;
        let index = x + y * metrics.width;
        if index < bitmap.len() {
            bitmap[index]
        } else { 0 }
    } else { 0 }
}

fn process(
    input: Vec<u8>,
    output: &mut impl Write,
    format: Format,
    name: String,
    size: f32,
    x_global_offset: f32,
    y_global_offset: f32,
    banner: String
) -> Result<(), Box<dyn Error>> {
    #[allow(unused_mut)]
    let font = Font::from_bytes(input, fontdue::FontSettings::default())?;

    let glyphs: Vec<_> = (0..u8::MAX).map(|c| (c, font.rasterize(c as char, size))).collect();

    let mut max_w = 0;
    let mut max_h = 0;
    for g in &glyphs {
        if g.1.0.width > max_w { max_w = g.1.0.width }
        if g.1.0.height > max_h { max_h = g.1.0.height }
    }

    writeln!(output, "{}", banner)?;
    writeln!(output, "")?;
    match format {
        Format::CppInlineHeader => {
            writeln!(output, "inline const unsigned char {}_count = {};", name, glyphs.len())?;
            writeln!(output, "inline const unsigned char {}_width = {};", name, max_w)?;
            writeln!(output, "inline const unsigned char {}_height = {};", name, max_h)?;
            writeln!(output, "inline const unsigned char {}[] = {{", name)?;
        },
        Format::Obj => {
            writeln!(output, "const unsigned char {}_count = {};", name, glyphs.len())?;
            writeln!(output, "const unsigned char {}_width = {};", name, max_w)?;
            writeln!(output, "const unsigned char {}_height = {};", name, max_h)?;
            writeln!(output, "const unsigned char {}[] = {{", name)?;
        },
    }

    for (ascii, (metrics, bitmap)) in glyphs {

        let offset_x = max_w as i32 - metrics.width as i32 + (x_global_offset * max_w as f32) as i32;
        let offset_y = max_h as i32 - metrics.height as i32 + (y_global_offset * max_h as f32) as i32;

        let w = max_w;
        let h = max_h;

        for y in 0..h {
            write!(output, "\t")?;
            for x in 0..w {
                let gc = grayscale_aligned(&metrics, &bitmap, x, y, offset_x, offset_y);

                if ascii < u8::MAX - 1 || y < h - 1 || x < w - 1 {
                    write!(output, "{0:<3}, ", gc)?;
                } else {
                    write!(output, "{0:<3} ", gc)?;
                }
            }
            writeln!(output, "")?;
        }
        if ascii < u8::MAX - 1 {
            writeln!(output, "")?;
        }
    }
    writeln!(output, "}};")?;
    Ok(())
}

struct TtfFile {
    name: String,
    data: Vec<u8>
}

fn read_input_file(path: &str) -> Result<TtfFile, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;

    if let Ok(mut arch) = ZipArchive::new(file) {
        for i in 0..arch.len() {
            if let Ok(mut file) = arch.by_index(i) {
                if let Some(extension) = Path::new(file.name()).extension() {
                    if extension == "ttf" {
                        let mut result: Vec<u8> = Vec::new();
                        file.read_to_end(&mut result)?;
                        return Ok(TtfFile {
                            name: Path::new(file.name())
                                .file_prefix()
                                .ok_or(format!("{} is not file path", path))?
                                .to_str()
                                .ok_or(format!("path {} is not convertable to utf8", path))?
                                .to_string(),
                            data: result
                        });
                    }
                }
            }
        }
        Err("input is archive but it does not contain .ttf files".into())
    } else {
        Ok(TtfFile {
            name: Path::new(path)
                .file_prefix()
                .ok_or(format!("{} is not file path", path))?
                .to_str()
                .ok_or(format!("path {} is not convertable to utf8", path))?
                .to_string(),
            data: std::fs::read(path)?
        })
    }
}

fn main() {
    let args = Args::parse();
    let input = args.input.clone();

    let suffix = match args.format {
        Format::CppInlineHeader => ".h",
        Format::Obj => ".c",
    };

    let input_ttf = read_input_file(&input)
        .expect(format!("Can not read input file {}", input).as_str());

    let output = args.output.unwrap_or(input_ttf.name + suffix);

    let name = if let Some(name) = args.name {
        name
    } else {
        var_name_from_file(PathBuf::from_str(&output)
            .expect(format!("{} is not a path", &output).as_str())
        ).unwrap()
    };

    let mut output_file = File::create(&output)
        .expect(format!("Can not open output file {}", &output).as_str());

    let banner: String = format!("
        |/*
        | * This file GENERATED by `packttf` tool
        | * !!!!!!!!!!!! DO NOT EDIT !!!!!!!!!!!!
        | *
        | * Source font  : '{}'
        | * Pixel size   : {}
        | * Pixel format : grayscale
        | */
    ",
        input,
        args.size
    )
        .trim_margin()
        .unwrap();

    process(
        input_ttf.data,
        &mut output_file,
        args.format,
        name,
        args.size,
        args.x_offset,
        args.y_offset,
        banner
    )
        .expect("Failed to convert");
}

