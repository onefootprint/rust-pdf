use std::collections::BTreeMap;

/// Represent a text encoding used in PDF.
/// An encoding maintains the connection between unicode code points,
/// bytes in PDF strings, and glyph names.
///
/// Currently, only WIN_ANSI_ENCODING is supported, and that encoding
/// is provided as a built-in.
/// # Example
/// ````
/// use pdf;
/// let encoding = &pdf::WIN_ANSI_ENCODING;
/// ````
#[derive(Debug)]
pub struct Encoding {
    name_to_code: BTreeMap<&'static str, u8>
}

impl Encoding {
    /// Get the encoded code point from a type1 character name.
    /// Character names are case sensitive and contains only ascii letters.
    /// If the name is not available in the encoding, or is not a proper
    /// character name, None is returned.
    ///
    /// # Example
    /// ````
    /// use pdf::WIN_ANSI_ENCODING;
    /// assert_eq!(Some(32),  WIN_ANSI_ENCODING.get_code("space"));
    /// assert_eq!(Some(65),  WIN_ANSI_ENCODING.get_code("A"));
    /// assert_eq!(Some(229), WIN_ANSI_ENCODING.get_code("aring"));
    /// assert_eq!(None,      WIN_ANSI_ENCODING.get_code("Lslash"));
    /// assert_eq!(None,      WIN_ANSI_ENCODING.get_code(""));
    /// assert_eq!(None,      WIN_ANSI_ENCODING.get_code("☺"));
    /// ````
    pub fn get_code(&self, name: &str) -> Option<u8> {
        match self.name_to_code.get(name) {
            Some(&code) => Some(code),
            None => None
        }
    }

    /// Convert a rust string to a vector of bytes in the encoding.
    /// FIXME This is currently hardcoded to /WinAnsiEncoding
    /// # Example
    /// ````
    /// use pdf;
    /// let e = &pdf::WIN_ANSI_ENCODING;
    /// assert_eq!(vec!(65, 66, 67), e.encode_string("ABC"));
    /// assert_eq!(vec!(82, 228, 107, 115, 109, 246, 114, 103, 229, 115),
    ///            e.encode_string("Räksmörgås"));
    /// assert_eq!(vec!(67, 111, 102, 102, 101, 101, 32, 128, 49, 46, 50, 48),
    ///            e.encode_string("Coffee €1.20"));
    /// ````
    pub fn encode_string(&self, text: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for ch in text.chars() {
            match ch {
                '\\' => { result.push('\\' as u8); result.push('\\' as u8) },
                '(' =>  { result.push('\\' as u8); result.push('(' as u8) },
                ')' =>  { result.push('\\' as u8); result.push(')' as u8) },
                // /WinAnsiEncoding is kind of close to first byte of unicode
                // Except for the 16 chars that are reserved in 8859-1 and used
                // in Windows-1252.
                '€' => { result.push(128) },
                '‚' => { result.push(130) },
                'ƒ' => { result.push(131) },
                '„' => { result.push(132) },
                '…' => { result.push(133) },
                '†' => { result.push(134) },
                '‡' => { result.push(135) },
                'ˆ' => { result.push(136) },
                '‰' => { result.push(137) },
                'Š' => { result.push(138) },
                '‹' => { result.push(139) },
                'Œ' => { result.push(140) },
                'Ž' => { result.push(142) },
                '‘' => { result.push(145) },
                '’' => { result.push(146) },
                '“' => { result.push(147) },
                '”' => { result.push(148) },
                '•' => { result.push(149) },
                '–' => { result.push(150) },
                '—' => { result.push(151) },
                '˜' => { result.push(152) },
                '™' => { result.push(153) },
                'š' => { result.push(154) },
                '›' => { result.push(155) },
                'ž' => { result.push(158) },
                'Ÿ' => { result.push(159) },
                ch @ ' '...'ÿ' => { result.push(ch as u8) }
                _ =>    { result.push('?' as u8); }
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
        let mut result = Encoding { name_to_code: BTreeMap::new() };
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
