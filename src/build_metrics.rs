#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{Write, Result};
use std::path::Path;

mod encoding;
use ::encoding::WIN_ANSI_ENCODING;

fn write_cond(f: &mut File, name: &str, data: &str) -> Result<()> {
    try!(writeln!(f, "  static ref METRICS_{}: FontMetrics = {{",
                  name.to_uppercase()));
    try!(writeln!(f, "    let mut widths = BTreeMap::new();"));
    for line in data.lines() {
        let words : Vec<&str> = line.split_whitespace().collect();
        if words[0] == "C" && words[3] == "WX" && words[6] == "N" {
            if let (Some(c), Ok(w)) = (WIN_ANSI_ENCODING.get_code(words[7]),
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
    writeln!(f, "pub fn get_builtin_metrics(font: &FontSource)").unwrap();
    writeln!(f, "-> Option<&'static FontMetrics> {{").unwrap();
    writeln!(f, "match *font {{").unwrap();
    for font in fonts {
        writeln!(f, "FontSource::{} => Some(METRICS_{}.deref()),", font, font.to_uppercase()).unwrap();
    };
    // When we support non-builtin fonts: writeln!(f, "_ => None").unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f, "}}").unwrap();

    writeln!(f, "lazy_static! {{").unwrap();
    write_cond(f, "Courier",
               include_str!("../data/Courier.afm")).unwrap();
    write_cond(f, "Courier_Bold",
               include_str!("../data/Courier-Bold.afm")).unwrap();
    write_cond(f, "Courier_BoldOblique",
               include_str!("../data/Courier-BoldOblique.afm")).unwrap();
    write_cond(f, "Courier_Oblique",
               include_str!("../data/Courier-Oblique.afm")).unwrap();
    write_cond(f, "Helvetica",
               include_str!("../data/Helvetica.afm")).unwrap();
    write_cond(f, "Helvetica_Bold",
               include_str!("../data/Helvetica-Bold.afm")).unwrap();
    write_cond(f, "Helvetica_BoldOblique",
               include_str!("../data/Helvetica-BoldOblique.afm")).unwrap();
    write_cond(f, "Helvetica_Oblique",
               include_str!("../data/Helvetica-Oblique.afm")).unwrap();
    write_cond(f, "Symbol",
               include_str!("../data/Symbol.afm")).unwrap();
    write_cond(f, "Times_Bold",
               include_str!("../data/Times-Bold.afm")).unwrap();
    write_cond(f, "Times_BoldItalic",
               include_str!("../data/Times-BoldItalic.afm")).unwrap();
    write_cond(f, "Times_Italic",
               include_str!("../data/Times-Italic.afm")).unwrap();
    write_cond(f, "Times_Roman",
               include_str!("../data/Times-Roman.afm")).unwrap();
    write_cond(f, "ZapfDingbats",
               include_str!("../data/ZapfDingbats.afm")).unwrap();
    writeln!(f, "}}").unwrap();
}
