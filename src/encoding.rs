use std::collections::BTreeMap;

/// Represent a text encoding used in PDF.
/// An encoding maintains the connection between unicode code points,
/// bytes in PDF strings, and glyph names.
///
/// Currently, only WIN_ANSI_ENCODING and SYMBOL_ENCODING are supported,
/// and they are provided as built-in.
///
/// # Example
/// ````
/// use pdf_canvas::{BuiltinFont, FontSource};
/// assert_eq!("WinAnsiEncoding",
///            BuiltinFont::Helvetica.get_encoding().get_name());
/// assert_eq!("SymbolEncoding",
///            BuiltinFont::Symbol.get_encoding().get_name());
/// ````
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Encoding {
    name: String,
    name_to_code: BTreeMap<&'static str, u8>,
    unicode_to_code: BTreeMap<char, u8>,
}

impl Encoding {
    /// The name of the encoding, as used in the font object.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Get the encoded code point from a type1 character name.
    /// Character names are case sensitive and contains only ascii letters.
    /// If the name is not available in the encoding, or is not a proper
    /// character name, None is returned.
    ///
    /// # Example
    /// ````
    /// use pdf_canvas::{BuiltinFont, FontSource};
    /// let enc = BuiltinFont::Helvetica.get_encoding();
    /// assert_eq!(Some(32),  enc.get_code("space"));
    /// assert_eq!(Some(65),  enc.get_code("A"));
    /// assert_eq!(Some(229), enc.get_code("aring"));
    /// assert_eq!(None,      enc.get_code("Lslash"));
    /// assert_eq!(None,      enc.get_code(""));
    /// assert_eq!(None,      enc.get_code("☺"));
    /// ````
    pub fn get_code(&self, name: &str) -> Option<u8> {
        match self.name_to_code.get(name) {
            Some(&code) => Some(code),
            None => None,
        }
    }

    /// Convert a rust string to a vector of bytes in the encoding.
    /// # Example
    /// ````
    /// use pdf_canvas::{BuiltinFont, FontSource};
    /// let enc = BuiltinFont::Helvetica.get_encoding();
    /// let symb_enc = BuiltinFont::Symbol.get_encoding();
    /// assert_eq!(vec!(65, 66, 67), enc.encode_string("ABC"));
    /// assert_eq!(vec!(82, 228, 107, 115, 109, 246, 114, 103, 229, 115),
    ///            enc.encode_string("Räksmörgås"));
    /// assert_eq!(vec!(67, 111, 102, 102, 101, 101, 32, 128, 49, 46, 50, 48),
    ///            enc.encode_string("Coffee €1.20"));
    /// assert_eq!(vec!(97, 32, 206, 32, 194),
    ///            symb_enc.encode_string("α ∈ ℜ"));
    /// ````
    pub fn encode_string(&self, text: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for ch in text.chars() {
            match ch {
                '\\' => {
                    result.push(b'\\');
                    result.push(b'\\')
                }
                '(' => {
                    result.push(b'\\');
                    result.push(b'(')
                }
                ')' => {
                    result.push(b'\\');
                    result.push(b')')
                }
                ch => result
                    .push(*self.unicode_to_code.get(&ch).unwrap_or(&(b'?'))),
            }
        }
        result
    }

    fn init_block(&mut self, start: u8, data: Vec<&'static str>) {
        let mut i = start - 1;
        for name in data {
            i += 1;
            self.name_to_code.insert(name, i);
        }
    }
}

lazy_static! {
    pub static ref WIN_ANSI_ENCODING: Encoding = {
        let mut codes = BTreeMap::new();
        // /WinAnsiEncoding is kind of close to first byte of unicode
        // Except for the 16 chars that are reserved in 8859-1 and
        // used in Windows-1252.
        for code in 32..255 {
            codes.insert(code as char, code);
        }
        codes.insert('€', 128);
        codes.insert('‚', 130);
        codes.insert('ƒ', 131);
        codes.insert('„', 132);
        codes.insert('…', 133);
        codes.insert('†', 134);
        codes.insert('‡', 135);
        codes.insert('ˆ', 136);
        codes.insert('‰', 137);
        codes.insert('Š', 138);
        codes.insert('‹', 139);
        codes.insert('Œ', 140);
        codes.insert('Ž', 142);
        codes.insert('‘', 145);
        codes.insert('’', 146);
        codes.insert('“', 147);
        codes.insert('”', 148);
        codes.insert('•', 149);
        codes.insert('–', 150);
        codes.insert('—', 151);
        codes.insert('˜', 152);
        codes.insert('™', 153);
        codes.insert('š', 154);
        codes.insert('›', 155);
        codes.insert('ž', 158);
        codes.insert('Ÿ', 159);
        let mut result = Encoding {
            name: "WinAnsiEncoding".to_string(),
            name_to_code: BTreeMap::new(),
            unicode_to_code: codes
        };
        result.init_block(0o40, vec!(
            "space", "exclam", "quotedbl", "numbersign",
            "dollar", "percent", "ampersand", "quotesingle"));
        result.init_block(0o50, vec!(
            "parenleft", "parenright", "asterisk", "plus",
            "comma", "hyphen", "period", "slash"));
        result.init_block(0o60, vec!(
            "zero", "one", "two", "three", "four", "five", "six", "seven"));
        result.init_block(0o70, vec!(
            "eight", "nine", "colon", "semicolon",
            "less", "equal", "greater", "question"));
        result.init_block(0o100, vec!(
            "at", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
            "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
            "W", "X", "Y", "Z"));
        result.init_block(0o133, vec!(
            "bracketleft",
            "backslash", "bracketright", "asciicircum", "underscore"));
        result.init_block(0o140, vec!(
            "grave", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
            "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
            "w", "x", "y", "z"));
        result.init_block(0o173, vec!(
            "braceleft", "bar", "braceright", "asciitilde"));
        result.init_block(0o200, vec!(
            "Euro", "..1", "quotesinglbase", "florin",
            "quotedblbase", "ellipsis", "dagger", "daggerdbl"));
        result.init_block(0o210, vec!(
            "circumflex", "perthousand", "Scaron", "guilsinglleft",
            "OE", "..5", "Zcaron", "..7"));
        result.init_block(0o220, vec!(
            "..0", "quoteleft", "quoteright", "quotedblleft",
            "quotedblright", "bullet", "endash", "emdash"));
        result.init_block(0o230, vec!(
            "tilde", "trademark", "scaron", "guilsinglright",
            "oe", "..5", "zcaron", "Ydieresis"));
        result.init_block(0o240, vec!(
            "..0", "exclamdown", "cent", "sterling",
            "currency", "yen", "brokenbar", "section"));
        result.init_block(0o250, vec!(
            "dieresis", "copyright", "ordfeminine", "guillemotleft",
            "logicalnot", "..5", "registered", "macron"));
        result.init_block(0o260, vec!(
            "degree", "plusminus", "twosuperior", "threesuperior",
            "acute", "mu", "paragraph", "periodcentered"));
        result.init_block(0o270, vec!(
            "cedilla", "onesuperior", "ordmasculine", "guillemotright",
            "onequarter", "onehalf", "threequarters", "questiondown"));
        result.init_block(0o300, vec!(
            "Agrave", "Aacute", "Acircumflex", "Atilde",
            "Adieresis", "Aring", "AE", "Ccedilla"));
        result.init_block(0o310, vec!(
            "Egrave", "Eacute", "Ecircumflex", "Edieresis",
            "Igrave", "Iacute", "Icircumflex", "Idieresis"));
        result.init_block(0o320, vec!(
            "Eth", "Ntilde", "Ograve", "Oacute",
            "Ocircumflex", "Otilde", "Odieresis", "multiply"));
        result.init_block(0o330, vec!(
            "Oslash", "Ugrave", "Uacute", "Ucircumflex",
            "Udieresis", "Yacute", "Thorn", "germandbls"));
        result.init_block(0o340, vec!(
            "agrave", "aacute", "acircumflex", "atilde",
            "adieresis", "aring", "ae", "ccedilla"));
        result.init_block(0o350, vec!(
            "egrave", "eacute", "ecircumflex", "edieresis",
            "igrave", "iacute", "icircumflex", "idieresis"));
        result.init_block(0o360, vec!(
            "eth", "ntilde", "ograve", "oacute",
            "ocircumflex", "otilde", "odieresis", "divide"));
        result.init_block(0o370, vec!(
            "oslash", "ugrave", "uacute", "ucircumflex",
            "udieresis", "yacute", "thorn", "ydieresis"));
        result
    };

    pub static ref SYMBOL_ENCODING: Encoding = {
        let mut codes = BTreeMap::new();
        let mut names = BTreeMap::new();
        for code in 32..255 {
            codes.insert(code as char, code);
        }
        {
            let mut enc = |ch: char, name: &'static str, code: u8| {
                codes.insert(ch, code);
                names.insert(name, code);
            };
            enc('Α', "Alpha",          0o101);
            enc('Β', "Beta",           0o102);
            enc('Χ', "Chi",            0o103);
            enc('Δ', "Delta",          0o104);
            enc('Ε', "Epsilon",        0o105);
            enc('Η', "Eta",            0o110);
            enc('€', "Euro",           0o240);
            enc('Γ', "Gamma",          0o107);
            enc('ℑ', "Ifraktur",       0o301);
            enc('Ι', "Iota",           0o111);
            enc('Κ', "Kappa",          0o113);
            enc('Λ', "Lambda",         0o114);
            enc('Μ', "Mu",             0o115);
            enc('Ν', "Nu",             0o116);
            enc('Ω', "Omega",          0o127);
            enc('Ο', "Omicron",        0o117);
            enc('Φ', "Phi",            0o106);
            enc('Π', "Pi",             0o120);
            enc('Ψ', "Psi",            0o131);
            enc('ℜ', "Rfraktur",       0o302);
            enc('Ρ', "Rho",            0o122);
            enc('Σ', "Sigma",          0o123);
            enc('Τ', "Tau",            0o124);
            enc('Θ', "Theta",          0o121);
            enc('Υ', "Upsilon",        0o125);
            enc('ϒ', "Upsilon1",       0o241);
            enc('Ξ', "Xi",             0o130);
            enc('Ζ', "Zeta",           0o132);
            enc('ℵ', "aleph",          0o141);
            enc('α', "alpha",          0o141);
            enc('&', "ampersand",      0o046);
            enc('∠', "angle",          0o320);
            enc('〈', "angleleft",      0o341);
            enc('〉', "angleright",     0o361);
            enc('≈', "approxequal",    0o273);
            enc('↔', "arrowboth",      0o253);
            enc('⇔', "arrowdblboth",   0o333);
            enc('⇓', "arrowdbldown",   0o337);
            enc('⇐', "arrowdblleft",   0o334);
            enc('⇒', "arrowdblright",  0o336);
            enc('⇑', "arrowdblup",     0o335);
            enc('↓', "arrowdown",      0o257);
            enc('\u{23af}', "arrowhorizex", 0o276);
            enc('←', "arrowleft",      0o254);
            enc('→', "arrowright",     0o256);
            enc('↑', "arrowup",        0o255);
            enc('\u{23d0}', "arrowvertex", 0o275);
            enc('*', "asteriskmath",   0o052);
            enc('|', "bar",            0o175);
            enc('β', "beta",           0o142);
            enc('{', "braceleft",      0o173);
            enc('}', "braceright",     0o175);
            enc('⎧', "bracelefttp",    0o354);
            enc('⎨', "braceleftmid",   0o355);
            enc('⎩', "braceleftbt",    0o356);
            enc('⎫', "bracerighttp",   0o374);
            enc('⎬', "bracerightmid",  0o375);
            enc('⎭', "bracerightbt",   0o376);
            enc('⎪', "braceex",        0o357);
            enc('[', "bracketleft",    0o133);
            enc(']', "bracketright",   0o135);
            enc('⎡', "bracketlefttp",  0o351);
            enc('⎢', "bracketleftex",  0o352);
            enc('⎣', "bracketleftbt",  0o353);
            enc('⎤', "bracketrighttp", 0o371);
            enc('⎥', "bracketrightex", 0o372);
            enc('⎦', "bracketrightbt", 0o373);
            enc('•', "bullet",         0o267);
            enc('↵', "carriagereturn", 0o277);
            enc('χ', "chi",            0o143);
            enc('⊗', "circlemultiply", 0o304);
            enc('⊕', "circleplus",     0o305);
            enc('♣', "club",           0o247);
            enc(':', "colon",          0o072);
            enc(',', "comma",          0o054);
            enc('≅', "congruent",      0o100);
            // NOTE: copyrightsans and copyrightserif is a single unicode point
            enc('©', "copyrightsans",  0o343);
            enc('©', "copyrightserif", 0o323);
            enc('°', "degree",         0o260);
            enc('δ', "delta",          0o144);
            enc('♦', "diamond",        0o250);
            enc('÷', "divide",         0o270);
            enc('⋅', "dotmath",        0o327);
            enc('8', "eight",          0o070);
            enc('∈', "element",        0o316); // NOTE: and ∊ ?
            enc('…', "ellipsis",       0o274);
            enc('∅', "emptyset",       0o306);
            enc('ε', "epsilon",        0o145);
            enc('=', "equal",          0o075);
            enc('≡', "equivalence",    0o272);
            enc('η', "eta",            0o150);
            enc('!', "exclam",         0o041);
            enc('∃', "existential",    0o044);
            enc('5', "five",           0o065);
            enc('ƒ', "florin",         0o246);
            enc('4', "four",           0o064);
            enc('⁄', "fraction",       0o244);
            enc('γ', "gamma",          0o147);
            enc('∇', "gradient",       0o321);
            enc('>', "greater",        0o076);
            enc('≥', "greaterequal",   0o263);
            enc('♥', "heart",          0o251);
            enc('∞', "infinity",       0o245);
            enc('∫', "integral",       0o362);
            enc('⌠', "integraltp",     0o363);
            enc('⎮', "integralex",     0o364);
            enc('⌡', "integralbt",     0o365);
            enc('∩', "intersection",   0o307);
            enc('ι', "iota",           0o151);
            enc('κ', "kappa",          0o153);
            enc('λ', "lambda",         0o154);
            enc('<', "less",           0o074);
            enc('≤', "lessequal",      0o243);
            enc('∧', "logicaland",     0o331);
            enc('¬', "logicalnot",     0o330);
            enc('∨', "logicalor",      0o332);
            enc('◊', "lozenge",        0o340);
            enc('-', "minus",          0o055);
            enc('\u{2032}', "minute",  0o242); // prime / minutes / feet
            enc('μ', "mu",             0o155);
            enc('×', "multiply",       0o264); // small and large in unicode
            enc('⨯', "multiply",       0o264); // only one in symbol
            enc('9', "nine",           0o071);
            enc('∉', "notelement",     0o317);
            enc('≠', "notequal",       0o271);
            enc('⊄', "notsubset",      0o313);
            enc('ν', "nu",             0o156);
            enc('#', "numbersign",     0o043);
            enc('ω', "omega",          0o167);
            enc('ϖ', "omega1",         0o166);
            enc('ο', "omicron",        0o157);
            enc('1', "one",            0o060);
            enc('(', "parenleft",      0o050);
            enc(')', "parenright",     0o051);
            enc('⎛', "parenlefttp",    0o346);
            enc('⎜', "parenleftex",    0o347);
            enc('⎝', "parenleftbt",    0o350);
            enc('⎞', "parenrighttp",   0o366);
            enc('⎟', "parenrightex",   0o367);
            enc('⎠', "parenrightbt",   0o360);
            enc('∂', "partialdiff",    0o266);
            enc('%', "percent",        0o045);
            enc('.', "period",         0o056);
            enc('⟂', "perpendicular",  0o136);
            enc('ɸ', "phi",            0o146);
            enc('φ', "phi1",           0o152);
            enc('π', "pi",             0o160);
            enc('+', "plus",           0o053);
            enc('±', "plusminus",      0o261);
            enc('∏', "product",        0o325);
            enc('⊂', "propersubset",   0o314);
            enc('⊃', "propersuperset", 0o311);
            enc('∝', "proportional",   0o265);
            enc('ψ', "psi",            0o171);
            enc('?', "question",       0o077);
            enc('√', "radical",        0o326);
            enc('⎺', "radicalex",      0o140); // Very approximate unicode
            enc('⊆', "reflexsubset",   0o315);
            enc('⊇', "reflexsuperset", 0o312);
            enc('®', "registersans",   0o342);
            enc('®', "registerserif",  0o322); // NOTE No distinct unicode?
            enc('ρ', "rho",            0o162);
            enc('\u{2033}', "second",  0o262); // Double prime/seconds/inches
            enc(';', "semicolon",      0o073);
            enc('7', "seven",          0o067);
            enc('σ', "sigma",          0o163);
            enc('ς', "sigma1",         0o126);
            enc('∼', "similar",        0o176);
            enc('6', "six",            0o066);
            enc('/', "slash",          0o057);
            enc(' ', "space",          0o040);
            enc('♠', "spade",          0o252);
            enc('∋', "suchthat",       0o047);
            enc('∑', "summation",      0o345);
            enc('τ', "tau",            0o164);
            enc('∴', "therefore",      0o134);
            enc('θ', "theta",          0o161);
            enc('ϑ', "theta1",         0o112);
            enc('3', "three",          0o063);
            enc('™', "trademarksans",  0o344);
            enc('™', "trademarkserif", 0o324); // NOTE No distinct unicode?
            enc('2', "two",            0o062);
            enc('_', "underscore",     0o137);
            enc('∪', "union",          0o310);
            enc('∀', "universal",      0o042);
            enc('υ', "upsilon",        0o165);
            enc('℘', "weierstrass",    0o303); // Maybe not correct unicode?
            enc('ξ', "xi",             0o170);
            enc('0', "zero",           0o060);
            enc('ζ', "zeta",           0o172);
        }
        Encoding {
            name: "SymbolEncoding".to_string(),
            name_to_code: names,
            unicode_to_code: codes
        }
    };

    // https://unicode.org/Public/MAPPINGS/VENDORS/ADOBE/zdingbat.txt
    pub static ref ZAPFDINGBATS_ENCODING: Encoding = {
        let mut codes = BTreeMap::new();
        let mut names = BTreeMap::new();
        for code in 32..255 {
            codes.insert(code as char, code);
        }
        {
            let mut enc = |ch: char, name: &'static str, code: u8| {
                codes.insert(ch, code);
                names.insert(name, code);
            };
            enc(' ', "space", 0o40);
            enc(' ', "space", 0o40);
            enc('✁', "a1", 0o41);
            enc('✂', "a2", 0o42);
            enc('✃', "a202", 0o43);
            enc('✄', "a3", 0o44);
            enc('☎', "a4", 0o45);
            enc('✆', "a5", 0o46);
            enc('✇', "a119", 0o47);
            enc('✈', "a118", 0o50);
            enc('✉', "a117", 0o51);
            enc('☛', "a11", 0o52);
            enc('☞', "a12", 0o53);
            enc('✌', "a13", 0o54);
            enc('✍', "a14", 0o55);
            enc('✎', "a15", 0o56);
            enc('✏', "a16", 0o57);
            enc('✐', "a105", 0o60);
            enc('✑', "a17", 0o61);
            enc('✒', "a18", 0o62);
            enc('✓', "a19", 0o63);
            enc('✔', "a20", 0o64);
            enc('✕', "a21", 0o65);
            enc('✖', "a22", 0o66);
            enc('✗', "a23", 0o67);
            enc('✘', "a24", 0o70);
            enc('✙', "a25", 0o71);
            enc('✚', "a26", 0o72);
            enc('✛', "a27", 0o73);
            enc('✜', "a28", 0o74);
            enc('✝', "a6", 0o75);
            enc('✞', "a7", 0o76);
            enc('✟', "a8", 0o77);
            enc('✠', "a9", 0o100);
            enc('✡', "a10", 0o101);
            enc('✢', "a29", 0o102);
            enc('✣', "a30", 0o103);
            enc('✤', "a31", 0o104);
            enc('✥', "a32", 0o105);
            enc('✦', "a33", 0o106);
            enc('✧', "a34", 0o107);
            enc('★', "a35", 0o110);
            enc('✩', "a36", 0o111);
            enc('✪', "a37", 0o112);
            enc('✫', "a38", 0o113);
            enc('✬', "a39", 0o114);
            enc('✭', "a40", 0o115);
            enc('✮', "a41", 0o116);
            enc('✯', "a42", 0o117);
            enc('✰', "a43", 0o120);
            enc('✱', "a44", 0o121);
            enc('✲', "a45", 0o122);
            enc('✳', "a46", 0o123);
            enc('✴', "a47", 0o124);
            enc('✵', "a48", 0o125);
            enc('✶', "a49", 0o126);
            enc('✷', "a50", 0o127);
            enc('✸', "a51", 0o130);
            enc('✹', "a52", 0o131);
            enc('✺', "a53", 0o132);
            enc('✻', "a54", 0o133);
            enc('✼', "a55", 0o134);
            enc('✽', "a56", 0o135);
            enc('✾', "a57", 0o136);
            enc('✿', "a58", 0o137);
            enc('❀', "a59", 0o140);
            enc('❁', "a60", 0o141);
            enc('❂', "a61", 0o142);
            enc('❃', "a62", 0o143);
            enc('❄', "a63", 0o144);
            enc('❅', "a64", 0o145);
            enc('❆', "a65", 0o146);
            enc('❇', "a66", 0o147);
            enc('❈', "a67", 0o150);
            enc('❉', "a68", 0o151);
            enc('❊', "a69", 0o152);
            enc('❋', "a70", 0o153);
            enc('●', "a71", 0o154);
            enc('❍', "a72", 0o155);
            enc('■', "a73", 0o156);
            enc('❏', "a74", 0o157);
            enc('❐', "a203", 0o160);
            enc('❑', "a75", 0o161);
            enc('❒', "a204", 0o162);
            enc('▲', "a76", 0o163);
            enc('▼', "a77", 0o164);
            enc('◆', "a78", 0o165);
            enc('❖', "a79", 0o166);
            enc('◗', "a81", 0o167);
            enc('❘', "a82", 0o170);
            enc('❙', "a83", 0o171);
            enc('❚', "a84", 0o172);
            enc('❛', "a97", 0o173);
            enc('❜', "a98", 0o174);
            enc('❝', "a99", 0o175);
            enc('❞', "a100", 0o176);
            enc('❨', "a89", 0o200); // Note: (CUS) cannot be displayed
            enc('❩', "a90", 0o201); // Note: (CUS) cannot be displayed
            enc('❪', "a93", 0o202); // Note: (CUS) cannot be displayed
            enc('❫', "a94", 0o203); // Note: (CUS) cannot be displayed
            enc('❬', "a91", 0o204); // Note: (CUS) cannot be displayed
            enc('❭', "a92", 0o205); // Note: (CUS) cannot be displayed
            enc('❮', "a205", 0o206); // Note: (CUS) cannot be displayed
            enc('❯', "a85", 0o207); // Note: (CUS) cannot be displayed
            enc('❰', "a206", 0o210); // Note: (CUS) cannot be displayed
            enc('❱', "a86", 0o211); // Note: (CUS) cannot be displayed
            enc('❲', "a87", 0o212); // Note: (CUS) cannot be displayed
            enc('❳', "a88", 0o213); // Note: (CUS) cannot be displayed
            enc('❴', "a95", 0o214); // Note: (CUS) cannot be displayed
            enc('❵', "a96", 0o215); // Note: (CUS) cannot be displayed
            enc('❡', "a101", 0o241);
            enc('❢', "a102", 0o242);
            enc('❣', "a103", 0o243);
            enc('❤', "a104", 0o244);
            enc('❥', "a106", 0o245);
            enc('❦', "a107", 0o246);
            enc('❧', "a108", 0o247);
            enc('♣', "a112", 0o250);
            enc('♦', "a111", 0o251);
            enc('♥', "a110", 0o252);
            enc('♠', "a109", 0o253);
            enc('①', "a120", 0o254);
            enc('②', "a121", 0o255);
            enc('③', "a122", 0o256);
            enc('④', "a123", 0o257);
            enc('⑤', "a124", 0o260);
            enc('⑥', "a125", 0o261);
            enc('⑦', "a126", 0o262);
            enc('⑧', "a127", 0o263);
            enc('⑨', "a128", 0o264);
            enc('⑩', "a129", 0o265);
            enc('❶', "a130", 0o266);
            enc('❷', "a131", 0o267);
            enc('❸', "a132", 0o270);
            enc('❹', "a133", 0o271);
            enc('❺', "a134", 0o272);
            enc('❻', "a135", 0o273);
            enc('❼', "a136", 0o274);
            enc('❽', "a137", 0o275);
            enc('❾', "a138", 0o276);
            enc('❿', "a139", 0o277);
            enc('➀', "a140", 0o300);
            enc('➁', "a141", 0o301);
            enc('➂', "a142", 0o302);
            enc('➃', "a143", 0o303);
            enc('➄', "a144", 0o304);
            enc('➅', "a145", 0o305);
            enc('➆', "a146", 0o306);
            enc('➇', "a147", 0o307);
            enc('➈', "a148", 0o310);
            enc('➉', "a149", 0o311);
            enc('➊', "a150", 0o312);
            enc('➋', "a151", 0o313);
            enc('➌', "a152", 0o314);
            enc('➍', "a153", 0o315);
            enc('➎', "a154", 0o316);
            enc('➏', "a155", 0o317);
            enc('➐', "a156", 0o320);
            enc('➑', "a157", 0o321);
            enc('➒', "a158", 0o322);
            enc('➓', "a159", 0o323);
            enc('➔', "a160", 0o324);
            enc('→', "a161", 0o325);
            enc('↔', "a163", 0o326);
            enc('↕', "a164", 0o327);
            enc('➘', "a196", 0o330);
            enc('➙', "a165", 0o331);
            enc('➚', "a192", 0o332);
            enc('➛', "a166", 0o333);
            enc('➜', "a167", 0o334);
            enc('➝', "a168", 0o335);
            enc('➞', "a169", 0o336);
            enc('➟', "a170", 0o337);
            enc('➠', "a171", 0o340);
            enc('➡', "a172", 0o341);
            enc('➢', "a173", 0o342);
            enc('➣', "a162", 0o343);
            enc('➤', "a174", 0o344);
            enc('➥', "a175", 0o345);
            enc('➦', "a176", 0o346);
            enc('➧', "a177", 0o347);
            enc('➨', "a178", 0o350);
            enc('➩', "a179", 0o351);
            enc('➪', "a193", 0o352);
            enc('➫', "a180", 0o353);
            enc('➬', "a199", 0o354);
            enc('➭', "a181", 0o355);
            enc('➮', "a200", 0o356);
            enc('➯', "a182", 0o357);
            enc('➱', "a201", 0o361);
            enc('➲', "a183", 0o362);
            enc('➳', "a184", 0o363);
            enc('➴', "a197", 0o364);
            enc('➵', "a185", 0o365);
            enc('➶', "a194", 0o366);
            enc('➷', "a198", 0o367);
            enc('➸', "a186", 0o370);
            enc('➹', "a195", 0o371);
            enc('➺', "a187", 0o372);
            enc('➻', "a188", 0o373);
            enc('➼', "a189", 0o374);
            enc('➽', "a190", 0o375);
            enc('➾', "a191", 0o376);
        }
        Encoding {
            name: "ZapfDingbatsEncoding".to_string(),
            name_to_code: names,
            unicode_to_code: codes
        }
    };
}

#[test]
fn test_get_winansi_points() {
    let ref enc = WIN_ANSI_ENCODING;
    assert_eq!(Some('A' as u8), enc.get_code("A"));
    assert_eq!(Some('Z' as u8), enc.get_code("Z"));
    assert_eq!(Some('a' as u8), enc.get_code("a"));
    assert_eq!(Some('z' as u8), enc.get_code("z"));
    assert_eq!(Some(' ' as u8), enc.get_code("space"));
    assert_eq!(Some('&' as u8), enc.get_code("ampersand"));
}
