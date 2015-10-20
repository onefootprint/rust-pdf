#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{Write, Result};
use std::path::Path;

#[allow(dead_code)]
mod encoding;
use ::encoding::{Encoding, WIN_ANSI_ENCODING, SYMBOL_ENCODING};

fn write_cond(f: &mut File, name: &str, encoding: Encoding, data: &str) -> Result<()> {
    try!(writeln!(f, "  static ref METRICS_{}: FontMetrics = {{",
                  name.to_uppercase()));
    try!(writeln!(f, "    let mut widths = BTreeMap::new();"));
    for line in data.lines() {
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
    let fonts = vec!("Courier", "Courier_Bold",
                     "Courier_Oblique",
                     "Courier_BoldOblique", "Helvetica",
                     "Helvetica_Bold", "Helvetica_Oblique",
                     "Helvetica_BoldOblique", "Times_Roman",
                     "Times_Bold", "Times_Italic",
                     "Times_BoldItalic", "Symbol",
                     "ZapfDingbats");
    writeln!(f, "pub fn get_builtin_metrics(font: &BuiltinFont)").unwrap();
    writeln!(f, "-> &'static FontMetrics {{").unwrap();
    writeln!(f, "match *font {{").unwrap();
    for font in fonts {
        writeln!(f, "BuiltinFont::{} => METRICS_{}.deref(),",
                 font, font.to_uppercase()).unwrap();
    };
    writeln!(f, "}}").unwrap();
    writeln!(f, "}}").unwrap();

    writeln!(f, "lazy_static! {{").unwrap();
    write_cond(f, "Courier", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Courier.afm")).unwrap();
    write_cond(f, "Courier_Bold", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Courier-Bold.afm")).unwrap();
    write_cond(f, "Courier_BoldOblique", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Courier-BoldOblique.afm")).unwrap();
    write_cond(f, "Courier_Oblique", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Courier-Oblique.afm")).unwrap();
    write_cond(f, "Helvetica", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Helvetica.afm")).unwrap();
    write_cond(f, "Helvetica_Bold", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Helvetica-Bold.afm")).unwrap();
    write_cond(f, "Helvetica_BoldOblique", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Helvetica-BoldOblique.afm")).unwrap();
    write_cond(f, "Helvetica_Oblique", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Helvetica-Oblique.afm")).unwrap();
    write_cond(f, "Symbol",  SYMBOL_ENCODING.clone(),
               include_str!("../data/Symbol.afm")).unwrap();
    write_cond(f, "Times_Bold", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Times-Bold.afm")).unwrap();
    write_cond(f, "Times_BoldItalic", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Times-BoldItalic.afm")).unwrap();
    write_cond(f, "Times_Italic", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Times-Italic.afm")).unwrap();
    write_cond(f, "Times_Roman", WIN_ANSI_ENCODING.clone(),
               include_str!("../data/Times-Roman.afm")).unwrap();
    write_cond(f, "ZapfDingbats", WIN_ANSI_ENCODING.clone(), // FIXME
               include_str!("../data/ZapfDingbats.afm")).unwrap();
    writeln!(f, "}}").unwrap();
}
