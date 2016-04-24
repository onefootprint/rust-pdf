#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, Result};
use std::path::Path;

#[allow(dead_code)]
mod encoding;
use ::encoding::{Encoding, WIN_ANSI_ENCODING, SYMBOL_ENCODING};

fn write_cond(f: &mut File, name: &str, encoding: &Encoding) -> Result<()> {
    try!(writeln!(f, "  static ref METRICS_{}: FontMetrics = {{",
                  name.to_uppercase()));
    try!(writeln!(f, "    let mut widths = BTreeMap::new();"));
    let afm_file = try!(File::open(format!("data/{}.afm", name.replace("_", "-"))));
    for lineresult in BufReader::new(afm_file).lines() {
        let line = try!(lineresult);
        let words : Vec<&str> = line.split_whitespace().collect();
        if words[0] == "C" && words[3] == "WX" && words[6] == "N" {
            if let (Some(c), Ok(w)) = (encoding.get_code(words[7]),
                                       words[4].parse::<u16>()) {
                try!(writeln!(f, "    widths.insert({}, {});", c, w));
            }
        }
    }
    try!(writeln!(f, "    FontMetrics{{ widths: widths }}"));
    try!(writeln!(f, "  }};"));
    Ok(())
}

fn main() {
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("metrics_data.rs");
    let mut f = &mut File::create(&dst).unwrap();
    let textfonts = vec!("Courier", "Courier_Bold",
                         "Courier_Oblique", "Courier_BoldOblique",
                         "Helvetica", "Helvetica_Bold",
                         "Helvetica_Oblique", "Helvetica_BoldOblique",
                         "Times_Roman", "Times_Bold",
                         "Times_Italic", "Times_BoldItalic");
    writeln!(f, "pub fn get_builtin_metrics(font: &BuiltinFont)").unwrap();
    writeln!(f, "-> &'static FontMetrics {{").unwrap();
    writeln!(f, "match *font {{").unwrap();
    for font in textfonts.iter().chain(vec!("Symbol", "ZapfDingbats").iter()) {
        writeln!(f, "BuiltinFont::{} => METRICS_{}.deref(),",
                 font, font.to_uppercase()).unwrap();
    };
    writeln!(f, "}}").unwrap();
    writeln!(f, "}}").unwrap();

    writeln!(f, "lazy_static! {{").unwrap();
    for font in textfonts {
        write_cond(f, font, &WIN_ANSI_ENCODING).unwrap();
    }
    write_cond(f, "Symbol",  &SYMBOL_ENCODING).unwrap();
    // FIXME There is a special encoding for ZapfDingbats
    write_cond(f, "ZapfDingbats", &WIN_ANSI_ENCODING).unwrap();
    writeln!(f, "}}").unwrap();
}
