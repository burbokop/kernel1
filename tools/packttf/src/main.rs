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
use fontdue::Font;
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
    /// Path to input ttf file
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

fn process(
    input: Vec<u8>,
    output: &mut impl Write,
    format: Format,
    name: String,
    size: f32,
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

    for (ascii, (metrix, bitmap)) in glyphs {

        let offset_x = if max_w >= metrix.width { (max_w - metrix.width) / 2 } else { 0 };
        let offset_y = if max_h >= metrix.height { (max_h - metrix.height) / 2 } else { 0 };


        for y in 0..max_h {
            write!(output, "\t")?;
            for x in 0..max_w {
                //println!("\twh: {}x{}, b {}x{}", metrix.width, metrix.height, metrix.bounds.width, metrix.bounds.height);
                //println!("\t{}x{}, min {}x{}", x, y, metrix.xmin, metrix.ymin);
                let mut o_x = x as i32;
                let mut o_y = y as i32;

                //let mut o_x = x as i32 + offset_x as i32;
                //let mut o_y = y as i32 + offset_y as i32;

                if o_x < 0 { o_x = 0 }
                if o_y < 0 { o_y = 0 }

                let o_x = o_x as usize;
                let o_y = o_y as usize;

                let index = o_x + o_y * metrix.width;
                let gc = if x < metrix.width && y < metrix.height && index < bitmap.len() {
                    bitmap[index]
                } else {
                    0
                };

                if ascii < u8::MAX - 1 || y < max_h - 1 || x < max_w - 1 {
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
        banner
    )
        .expect("Failed to convert");
}

