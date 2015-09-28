use std::env;
use std::fs::File;
use std::io::{Write, Result};
use std::path::Path;

fn write_cond(f: &mut File, name: &str, data: &str) -> Result<()> {
    try!(writeln!(f, "  if name == \"{}\" {{", name));
    try!(writeln!(f, "    let mut widths : BTreeMap<u8, u16> = BTreeMap::new();"));
    for line in data.lines() {
        let words : Vec<&str> = line.split_whitespace().collect();
        if words[0] == "C" && words[3] == "WX" {
            if let (Ok(c), Ok(w)) = (words[1].parse::<i16>(),
                                     words[4].parse::<u16>()) {
                if c != -1 {
                    try!(writeln!(f, "    widths.insert({}, {});", c, w));
                }
            }
        }
    }
    try!(writeln!(f, "    return Some(FontMetrics{{ widths: widths }})"));
    try!(writeln!(f, "  }}"));
    Ok(())
}

fn main() {
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("metrics_data.rs");
    let mut f = &mut File::create(&dst).unwrap();
    writeln!(f, "pub fn get_builtin_metrics(name: &str) -> Option<FontMetrics> {{").unwrap();
    write_cond(f, "Courier",
               include_str!("../data/Courier.afm")).unwrap();
    write_cond(f, "Courier-Bold",
               include_str!("../data/Courier-Bold.afm")).unwrap();
    write_cond(f, "Courier-BoldOblique",
               include_str!("../data/Courier-BoldOblique.afm")).unwrap();
    write_cond(f, "Courier-Oblique",
               include_str!("../data/Courier-Oblique.afm")).unwrap();
    write_cond(f, "Helvetica",
               include_str!("../data/Helvetica.afm")).unwrap();
    write_cond(f, "Helvetica-Bold",
               include_str!("../data/Helvetica-Bold.afm")).unwrap();
    write_cond(f, "Helvetica-BoldOblique",
               include_str!("../data/Helvetica-BoldOblique.afm")).unwrap();
    write_cond(f, "Helvetica-Oblique",
               include_str!("../data/Helvetica-Oblique.afm")).unwrap();
    write_cond(f, "Symbol",
               include_str!("../data/Symbol.afm")).unwrap();
    write_cond(f, "Times-Bold",
               include_str!("../data/Times-Bold.afm")).unwrap();
    write_cond(f, "Times-BoldItalic",
               include_str!("../data/Times-BoldItalic.afm")).unwrap();
    write_cond(f, "Times-Italic",
               include_str!("../data/Times-Italic.afm")).unwrap();
    write_cond(f, "Times-Roman",
               include_str!("../data/Times-Roman.afm")).unwrap();
    write_cond(f, "ZapfDingbats",
               include_str!("../data/ZapfDingbats.afm")).unwrap();
    writeln!(f, "  None").unwrap();
    writeln!(f, "}}").unwrap();
}
