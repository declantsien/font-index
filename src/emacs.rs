use isolang::Language;
use once_cell::sync::Lazy as LazyLock;
use swash::{Stretch, Style, Weight};

use crate::{
    types::OpentypeSpec,
    util::{fxhash::FxHashMap, string::SmallString},
};

pub static EMACS_CHARSET_MAP: LazyLock<FxHashMap<SmallString, (Vec<u32>, Option<SmallString>)>> =
    LazyLock::new(|| {
        log::trace!("EMACS_CHARSET_MAP is being created...");
        let mut charset_map = FxHashMap::default();
        charset_map.insert(
            SmallString::new("iso8859-1"),
            (vec![0x00A0, 0x00A1, 0x00B4, 0x00BC, 0x00D0], None),
        );
        charset_map.insert(SmallString::new("iso8859-2"), (vec![0x00A0, 0x010E], None));
        charset_map.insert(SmallString::new("iso8859-3"), (vec![0x00A0, 0x0108], None));
        charset_map.insert(
            SmallString::new("iso8859-4"),
            (vec![0x00A0, 0x00AF, 0x0128, 0x0156, 0x02C7], None),
        );
        charset_map.insert(SmallString::new("iso8859-5"), (vec![0x00A0, 0x0401], None));
        charset_map.insert(SmallString::new("iso8859-6"), (vec![0x00A0, 0x060C], None));
        charset_map.insert(SmallString::new("iso8859-7"), (vec![0x00A0, 0x0384], None));
        charset_map.insert(SmallString::new("iso8859-8"), (vec![0x00A0, 0x05D0], None));
        charset_map.insert(
            SmallString::new("iso8859-9"),
            (vec![0x00A0, 0x00A1, 0x00BC, 0x011E], None),
        );
        charset_map.insert(
            SmallString::new("iso8859-10"),
            (vec![0x00A0, 0x00D0, 0x0128, 0x2015], None),
        );
        charset_map.insert(SmallString::new("iso8859-11"), (vec![0x00A0, 0x0E01], None));
        charset_map.insert(SmallString::new("iso8859-13"), (vec![0x00A0, 0x201C], None));
        charset_map.insert(SmallString::new("iso8859-14"), (vec![0x00A0, 0x0174], None));
        charset_map.insert(
            SmallString::new("iso8859-15"),
            (vec![0x00A0, 0x00A1, 0x00D0, 0x0152], None),
        );
        charset_map.insert(SmallString::new("iso8859-16"), (vec![0x00A0, 0x0218], None));
        charset_map.insert(
            SmallString::new("gb2312.1980-0"),
            (vec![0x4E13], Some(SmallString::new("zh-Hans"))),
        );

        charset_map.insert(
            SmallString::new("big5-0"),
            (
                vec![/* 0x9C21 in ftfont.c */ 0x4EDC],
                Some(SmallString::new("zh-Hant")),
            ),
        );
        charset_map.insert(
            SmallString::new("jisx0208.1983-0"),
            (vec![0x4E55], Some(SmallString::new("ja"))),
        );
        charset_map.insert(
            SmallString::new("ksc5601.1987-0"),
            (vec![0xAC00], Some(SmallString::new("ko"))),
        );
        charset_map.insert(
            SmallString::new("cns11643.1992-1"),
            (vec![0xFE32], Some(SmallString::new("zh-Hant"))),
        );
        charset_map.insert(
            SmallString::new("cns11643.1992-2"),
            (vec![0x4E33, 0x7934], None),
        );
        charset_map.insert(SmallString::new("cns11643.1992-3"), (vec![0x201A9], None));
        charset_map.insert(SmallString::new("cns11643.1992-4"), (vec![0x20057], None));
        charset_map.insert(SmallString::new("cns11643.1992-5"), (vec![0x20000], None));
        charset_map.insert(SmallString::new("cns11643.1992-6"), (vec![0x20003], None));
        charset_map.insert(SmallString::new("cns11643.1992-7"), (vec![0x20055], None));
        charset_map.insert(
            SmallString::new("gbk-0"),
            (vec![0x4E06], Some(SmallString::new("zh-Hans"))),
        );
        charset_map.insert(SmallString::new("jisx0212.1990-0"), (vec![0x4E44], None));
        charset_map.insert(
            SmallString::new("jisx0213.2000-1"),
            (vec![0xFA10], Some(SmallString::new("ja"))),
        );
        charset_map.insert(SmallString::new("jisx0213.2000-2"), (vec![0xFA49], None));
        charset_map.insert(SmallString::new("jisx0213.2004-1"), (vec![0x20B9F], None));
        charset_map.insert(
            SmallString::new("viscii1.1-1"),
            (vec![0x1EA0, 0x1EAE, 0x1ED2], Some(SmallString::new("vi"))),
        );
        charset_map.insert(
            SmallString::new("tis620.2529-1"),
            (vec![0x0E01], Some(SmallString::new("th"))),
        );
        charset_map.insert(
            SmallString::new("windows-1251"),
            (vec![0x0401, 0x0490], Some(SmallString::new("ru"))),
        );
        charset_map.insert(
            SmallString::new("koi8-r"),
            (vec![0x0401, 0x2219], Some(SmallString::new("ru"))),
        );
        charset_map.insert(
            SmallString::new("mulelao-1"),
            (vec![0x0E81], Some(SmallString::new("lo"))),
        );
        charset_map.insert(SmallString::new("unicode-sip"), (vec![0x20000], None));

        charset_map
    });

pub static SCRIPT_REPRESENTATIVE_CHARS: LazyLock<FxHashMap<SmallString, Vec<u32>>> =
    LazyLock::new(|| {
        log::trace!("SCRIPT_REPRESENTATIVE_CHARS is being created...");
        let mut charset_map = FxHashMap::default();
        charset_map.insert(
            SmallString::new("latin"),
            vec![
                'A'.into(),
                'Z'.into(),
                'a'.into(),
                'z'.into(),
                0x00C0,
                0x0100,
                0x0180,
                0x1e00,
            ],
        );
        charset_map.insert(SmallString::new("phonetic"), vec![0x250, 0x283]);
        charset_map.insert(SmallString::new("greek"), vec![0x3A9]);
        charset_map.insert(SmallString::new("coptic"), vec![0x3E2, 0x2C80, 0x2CAE]);
        charset_map.insert(SmallString::new("cyrillic"), vec![0x42F]);
        charset_map.insert(SmallString::new("armenian"), vec![0x531]);
        charset_map.insert(SmallString::new("hebrew"), vec![0x5D0]);
        charset_map.insert(SmallString::new("vai"), vec![0xA500]);
        // U+06C1 prevents us from using bad fonts, like DejaVu Sans,
        // for Arabic text.
        charset_map.insert(SmallString::new("arabic"), vec![0x628, 0x6C1]);
        charset_map.insert(SmallString::new("syriac"), vec![0x710]);
        charset_map.insert(SmallString::new("thaana"), vec![0x78C]);
        charset_map.insert(SmallString::new("devanagari"), vec![0x915]);
        charset_map.insert(SmallString::new("bengali"), vec![0x995]);
        charset_map.insert(SmallString::new("gurmukhi"), vec![0xA15]);
        charset_map.insert(SmallString::new("gujarati"), vec![0xA95]);
        charset_map.insert(SmallString::new("oriya"), vec![0xB15]);
        charset_map.insert(SmallString::new("tamil"), vec![0xB95]);
        charset_map.insert(SmallString::new("telugu"), vec![0xC15]);
        charset_map.insert(SmallString::new("kannada"), vec![0xC95]);
        charset_map.insert(SmallString::new("malayalam"), vec![0xD15]);
        charset_map.insert(SmallString::new("sinhala"), vec![0xD95]);
        charset_map.insert(SmallString::new("thai"), vec![0xE17]);
        charset_map.insert(SmallString::new("lao"), vec![0xEA5]);
        charset_map.insert(SmallString::new("tibetan"), vec![0xF40]);
        charset_map.insert(SmallString::new("burmese"), vec![0x1000]);
        charset_map.insert(SmallString::new("georgian"), vec![0x10D3]);
        charset_map.insert(SmallString::new("ethiopic"), vec![0x1208]);
        charset_map.insert(SmallString::new("cherokee"), vec![0x13B6]);
        charset_map.insert(SmallString::new("canadian-aboriginal"), vec![0x14C0]);
        charset_map.insert(SmallString::new("ogham"), vec![0x168F]);
        charset_map.insert(SmallString::new("runic"), vec![0x16A0]);
        charset_map.insert(SmallString::new("tagalog"), vec![0x1700]);
        charset_map.insert(SmallString::new("hanunoo"), vec![0x1720]);
        charset_map.insert(SmallString::new("buhid"), vec![0x1740]);
        charset_map.insert(SmallString::new("tagbanwa"), vec![0x1760]);
        charset_map.insert(SmallString::new("khmer"), vec![0x1780]);
        charset_map.insert(SmallString::new("mongolian"), vec![0x1826]);
        charset_map.insert(SmallString::new("limbu"), vec![0x1901, 0x1920, 0x1936]);
        charset_map.insert(SmallString::new("buginese"), vec![0x1A00, 0x1A1E]);
        charset_map.insert(SmallString::new("balinese"), vec![0x1B13, 0x1B35, 0x1B5E]);
        charset_map.insert(SmallString::new("sundanese"), vec![0x1B8A, 0x1BAB, 0x1CC4]);
        charset_map.insert(SmallString::new("batak"), vec![0x1BC2, 0x1BE7, 0x1BFF]);
        charset_map.insert(SmallString::new("lepcha"), vec![0x1C00, 0x1C24, 0x1C40]);
        charset_map.insert(SmallString::new("tai-le"), vec![0x1950]);
        charset_map.insert(SmallString::new("tai-lue"), vec![0x1980]);
        charset_map.insert(
            SmallString::new("tai-tham"),
            vec![0x1A20, 0x1A55, 0x1A61, 0x1A80],
        );
        charset_map.insert(SmallString::new("symbol"), vec![0x201C, 0x2200, 0x2500]);
        charset_map.insert(SmallString::new("braille"), vec![0x2800]);
        charset_map.insert(SmallString::new("ideographic-description"), vec![0x2FF0]);
        charset_map.insert(
            SmallString::new("cjk-misc"),
            vec![0x300E, 0xff0c, 0x300a, 0xff09, 0x5b50],
        );
        charset_map.insert(SmallString::new("kana"), vec![0x304B]);
        charset_map.insert(SmallString::new("bopomofo"), vec![0x3105]);
        charset_map.insert(SmallString::new("kanbun"), vec![0x319D]);
        charset_map.insert(SmallString::new("han"), vec![0x5B57]);
        charset_map.insert(SmallString::new("yi"), vec![0xA288]);
        charset_map.insert(
            SmallString::new("syloti-nagri"),
            vec![0xA807, 0xA823, 0xA82C],
        );
        charset_map.insert(SmallString::new("rejang"), vec![0xA930, 0xA947, 0xA95F]);
        charset_map.insert(SmallString::new("javanese"), vec![0xA98F, 0xA9B4, 0xA9CA]);
        charset_map.insert(SmallString::new("cham"), vec![0xAA00]);
        charset_map.insert(SmallString::new("tai-viet"), vec![0xAA80]);
        charset_map.insert(
            SmallString::new("meetei-mayek"),
            vec![0xABC0, 0xABE3, 0xAAE0, 0xAAF6],
        );
        charset_map.insert(SmallString::new("hangul"), vec![0xAC00]);
        charset_map.insert(SmallString::new("linear-b"), vec![0x10000]);
        charset_map.insert(SmallString::new("aegean-number"), vec![0x10100]);
        charset_map.insert(SmallString::new("ancient-greek-number"), vec![0x10140]);
        charset_map.insert(SmallString::new("ancient-symbol"), vec![0x10190]);
        charset_map.insert(SmallString::new("phaistos-disc"), vec![0x101D0]);
        charset_map.insert(SmallString::new("lycian"), vec![0x10280]);
        charset_map.insert(SmallString::new("carian"), vec![0x102A0]);
        charset_map.insert(SmallString::new("old-italic"), vec![0x10300]);
        charset_map.insert(SmallString::new("gothic"), vec![0x10330, 0x10348]);
        charset_map.insert(SmallString::new("ugaritic"), vec![0x10380]);
        charset_map.insert(SmallString::new("old-permic"), vec![0x10350]);
        charset_map.insert(SmallString::new("old-persian"), vec![0x103A0]);
        charset_map.insert(SmallString::new("deseret"), vec![0x10400]);
        charset_map.insert(SmallString::new("shavian"), vec![0x10450]);
        charset_map.insert(SmallString::new("osmanya"), vec![0x10480]);
        charset_map.insert(SmallString::new("osage"), vec![0x104B0]);
        charset_map.insert(SmallString::new("elbasan"), vec![0x10500]);
        charset_map.insert(SmallString::new("caucasian-albanian"), vec![0x10530]);
        charset_map.insert(SmallString::new("vithkuqi"), vec![0x10570]);
        charset_map.insert(SmallString::new("linear-a"), vec![0x10600]);
        charset_map.insert(SmallString::new("cypriot-syllabary"), vec![0x10800]);
        charset_map.insert(SmallString::new("palmyrene"), vec![0x10860]);
        charset_map.insert(SmallString::new("nabataean"), vec![0x10880]);
        charset_map.insert(SmallString::new("phoenician"), vec![0x10900]);
        charset_map.insert(SmallString::new("lydian"), vec![0x10920]);
        charset_map.insert(SmallString::new("kharoshthi"), vec![0x10A00]);
        charset_map.insert(SmallString::new("manichaean"), vec![0x10AC0]);
        charset_map.insert(
            SmallString::new("hanifi-rohingya"),
            vec![0x10D00, 0x10D24, 0x10D39],
        );
        charset_map.insert(SmallString::new("yezidi"), vec![0x10E80]);
        charset_map.insert(SmallString::new("old-sogdian"), vec![0x10F00]);
        charset_map.insert(SmallString::new("sogdian"), vec![0x10F30]);
        charset_map.insert(SmallString::new("chorasmian"), vec![0x10FB0]);
        charset_map.insert(SmallString::new("elymaic"), vec![0x10FE0]);
        charset_map.insert(SmallString::new("old-uyghur"), vec![0x10F70]);
        charset_map.insert(
            SmallString::new("brahmi"),
            vec![0x11013, 0x11045, 0x11052, 0x11065],
        );
        charset_map.insert(SmallString::new("kaithi"), vec![0x1108D, 0x110B0, 0x110BD]);
        charset_map.insert(SmallString::new("mahajani"), vec![0x11150]);
        charset_map.insert(SmallString::new("sharada"), vec![0x11191, 0x111B3, 0x111CD]);
        charset_map.insert(SmallString::new("khojki"), vec![0x11200]);
        charset_map.insert(SmallString::new("khudawadi"), vec![0x112B0]);
        charset_map.insert(SmallString::new("grantha"), vec![0x11315, 0x1133E, 0x11374]);
        charset_map.insert(SmallString::new("newa"), vec![0x11400]);
        charset_map.insert(SmallString::new("tirhuta"), vec![0x11481, 0x1148F, 0x114D0]);
        charset_map.insert(SmallString::new("siddham"), vec![0x1158E, 0x115AF, 0x115D4]);
        charset_map.insert(SmallString::new("modi"), vec![0x1160E, 0x11630, 0x11655]);
        charset_map.insert(SmallString::new("takri"), vec![0x11680]);
        charset_map.insert(SmallString::new("dogra"), vec![0x11800]);
        charset_map.insert(SmallString::new("warang-citi"), vec![0x118A1]);
        charset_map.insert(SmallString::new("dives-akuru"), vec![0x11900]);
        charset_map.insert(SmallString::new("nandinagari"), vec![0x119a0]);
        charset_map.insert(SmallString::new("zanabazar-square"), vec![0x11A00]);
        charset_map.insert(SmallString::new("soyombo"), vec![0x11A50]);
        charset_map.insert(SmallString::new("pau-cin-hau"), vec![0x11AC0]);
        charset_map.insert(SmallString::new("bhaiksuki"), vec![0x11C00]);
        charset_map.insert(SmallString::new("marchen"), vec![0x11C72]);
        charset_map.insert(SmallString::new("masaram-gondi"), vec![0x11D00]);
        charset_map.insert(SmallString::new("gunjala-gondi"), vec![0x11D60]);
        charset_map.insert(SmallString::new("makasar"), vec![0x11EE0, 0x11EF7]);
        charset_map.insert(SmallString::new("kawi"), vec![0x11F04, 0x11F41, 0x11F4F]);
        charset_map.insert(SmallString::new("cuneiform"), vec![0x12000]);
        charset_map.insert(SmallString::new("cypro-minoan"), vec![0x12F90]);
        charset_map.insert(SmallString::new("egyptian"), vec![0x13000]);
        charset_map.insert(SmallString::new("mro"), vec![0x16A40]);
        charset_map.insert(SmallString::new("tangsa"), vec![0x16A70, 0x16AC0]);
        charset_map.insert(SmallString::new("bassa-vah"), vec![0x16AD0]);
        charset_map.insert(SmallString::new("pahawh-hmong"), vec![0x16B11]);
        charset_map.insert(SmallString::new("medefaidrin"), vec![0x16E40]);
        charset_map.insert(SmallString::new("tangut"), vec![0x17000]);
        charset_map.insert(SmallString::new("khitan-small-script"), vec![0x18B00]);
        charset_map.insert(SmallString::new("nushu"), vec![0x1B170]);
        charset_map.insert(SmallString::new("duployan-shorthand"), vec![0x1BC20]);
        charset_map.insert(
            SmallString::new("znamenny-musical-notation"),
            vec![0x1CF00, 0x1CF42, 0x1CF50],
        );
        charset_map.insert(SmallString::new("byzantine-musical-symbol"), vec![0x1D000]);
        charset_map.insert(SmallString::new("musical-symbol"), vec![0x1D100]);
        charset_map.insert(
            SmallString::new("ancient-greek-musical-notation"),
            vec![0x1D200],
        );
        charset_map.insert(SmallString::new("kaktovik-numeral"), vec![0x1D2C0]);
        charset_map.insert(SmallString::new("tai-xuan-jing-symbol"), vec![0x1D300]);
        charset_map.insert(SmallString::new("counting-rod-numeral"), vec![0x1D360]);
        charset_map.insert(SmallString::new("nyiakeng-puachue-hmong"), vec![0x1e100]);
        charset_map.insert(SmallString::new("toto"), vec![0x1E290, 0x1E295, 0x1E2AD]);
        charset_map.insert(SmallString::new("wancho"), vec![0x1E2C0, 0x1E2E8, 0x1E2EF]);
        charset_map.insert(
            SmallString::new("nag-mundari"),
            vec![0x1E4D0, 0x1E4EB, 0x1E4F0],
        );
        charset_map.insert(SmallString::new("mende-kikakui"), vec![0x1E810, 0x1E8A6]);
        charset_map.insert(SmallString::new("adlam"), vec![0x1E900, 0x1E943]);
        charset_map.insert(
            SmallString::new("indic-siyaq-number"),
            vec![0x1EC71, 0x1EC9F],
        );
        charset_map.insert(
            SmallString::new("ottoman-siyaq-number"),
            vec![0x1ED01, 0x1ED27],
        );
        charset_map.insert(SmallString::new("mahjong-tile"), vec![0x1F000]);
        charset_map.insert(SmallString::new("domino-tile"), vec![0x1F030]);
        charset_map.insert(SmallString::new("emoji"), vec![0x1F300, 0x1F600]);
        charset_map.insert(SmallString::new("chess-symbol"), vec![0x1FA00, 0x1FA67]);

        charset_map
    });

#[derive(Clone)]
pub struct FontSpec {
    // VALUE must be a string specifying the font family
    // (e.g. "Monospace").
    pub family: Option<String>,

    // VALUE must be a string or a symbol specifying the font foundry, e.g. misc.
    pub foundry: Option<String>,

    pub width: Option<Stretch>,

    // VALUE specifies the weight of the font to use.  It must be one of
    // the symbols ultra-heavy, heavy (a.k.a. black),
    // ultra-bold (a.k.a. extra-bold), bold,
    // semi-bold (a.k.a. demi-bold), medium, normal (a.k.a. regular,
    // a.k.a. book), semi-light (a.k.a. demi-light),
    // light, extra-light (a.k.a. ultra-light), or thin.
    pub weight: Option<Weight>,

    // VALUE specifies the slant of the font to use.  It must be one of the
    // symbols italic, oblique, normal, reverse-italic, or
    // reverse-oblique.
    pub slant: Option<Style>,

    // VALUE must be a string or a symbol specifying the additional
    // typographic style information of a font, e.g. sans.
    pub adstyle: Option<String>,

    // VALUE must be a string or a symbol specifying the charset registry and
    // encoding of a font, e.g. iso8859-1.
    pub registry: Option<String>,

    // VALUE must be a non-negative integer or a floating point number
    // specifying the font size.  It specifies the font size in pixels (if
    // VALUE is an integer), or in points (if VALUE is a float).
    pub size: Option<i32>,

    // VALUE must be a non-negative number that specifies the resolution
    // (dot per inch) for which the font is designed.
    pub dpi: Option<u32>,

    // VALUE specifies the spacing of the font: mono, proportional, charcell,
    // or dual.  It can be either a number (0 for proportional, 90 for dual,
    // 100 for mono, 110 for charcell) or a 1-letter symbol: P, D, M,
    // or C (lower-case variants are also accepted).
    pub spacing: Option<Spacing>,

    // VALUE must be a non-negative integer specifying the average width of
    // the font in 1/10 pixel units.
    pub avgwidth: Option<i32>,

    // VALUE must be a string of XLFD-style or fontconfig-style font name.
    pub name: Option<String>,

    // VALUE must be a symbol representing a script that the font must
    // support.  It may be a symbol representing a subgroup of a script
    // listed in the variable script-representative-chars.
    pub script: Option<String>,

    // VALUE must be a symbol whose name is a two-letter ISO-639 language
    // name, e.g. ja.  The value is matched against the "Additional Style"
    // field of the XLFD spec of a font, if it's non-empty, on X, and
    // against the codepages supported by the font on w32.
    pub lang: Option<Language>,

    // VALUE must be a list (SCRIPT-TAG, LANGSYS-TAG, GSUB, GPOS) to specify
    // required OpenType features.
    //   script_tag: OpenType script tag symbol (e.g. deva).
    //   langsys_tag: OpenType language system tag symbol,
    //      or None for the default language system.
    //   gsub: List of OpenType GSUB feature tag symbols, or None if none required.
    //   GPOS: List of OpenType GPOS feature tag symbols, or None if none required.

    // GSUB and GPOS may contain None elements.  In such a case, the font
    // must not have any of the remaining elements.

    // For instance, if the VALUE is
    // (thai, None, None, vec![mark]),
    // the font must
    // be an OpenType font whose GPOS table of thai script's default
    // language system must contain mark feature.
    pub otf: Option<OpentypeSpec>,
}

// mono, proportional, charcell,
// or dual.  It can be either a number (0 for proportional, 90 for dual,
// 100 for mono, 110 for charcell) or a 1-letter symbol: P, D, M,
// or C (lower-case variants are also accepted).
#[derive(Clone)]
pub enum Spacing {
    Mono = 0,
    Proportional = 90,
    Charcell = 100,
    Dual = 110,
}

impl Spacing {
    pub fn from_number(num: i32) -> Option<Self> {
        match num {
            0 => Some(Self::Proportional),
            90 => Some(Self::Dual),
            100 => Some(Self::Mono),
            110 => Some(Self::Charcell),
            _ => None,
        }
    }

    pub fn from_symbol(sym: &str) -> Option<Self> {
        match sym {
            "P" | "p" => Some(Self::Proportional),
            "D" | "d" => Some(Self::Dual),
            "M" | "m" => Some(Self::Mono),
            "C" | "c" => Some(Self::Charcell),
            _ => None,
        }
    }
}
