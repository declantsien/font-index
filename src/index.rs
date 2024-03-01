//! Font index.

use super::index_data::*;
use super::types::*;
use super::{
    fallback::Fallbacks,
    types::{FamilyId, GenericFamily},
};
#[cfg(feature = "emacs")]
use crate::emacs::FontSpec;
use crate::util::{
    fxhash::FxHashMap,
    string::{LowercaseString, SmallString},
};
#[cfg(feature = "emacs")]
use fancy_regex::Regex;
#[cfg(feature = "emacs")]
use log::warn;
#[cfg(feature = "emacs")]
use std::collections::HashSet;
use std::path::Path;
use swash::text::{Cjk, Script};
use swash::Tag;
#[cfg(feature = "emacs")]
use swash::{text::Language, Stretch, Style, Weight};
use swash::{Attributes, CacheKey};

/// Type alias for signatures to distinguish between inherent and
/// requested attributes.
pub type RequestedAttributes = Attributes;

#[derive(Default)]
pub struct BaseIndex {
    pub family_map: FxHashMap<SmallString, FamilyId>,
    pub fonts: Vec<FontData>,
    pub sources: Vec<SourceData>,
}

pub struct StaticIndex {
    pub base: BaseIndex,
    pub families: Vec<FamilyData>,
    pub script_map: FxHashMap<Script, Fallbacks>,
    pub script_tag_map: FxHashMap<Tag, Vec<FamilyId>>,
    pub language_tag_map: FxHashMap<Tag, Vec<FamilyId>>,
    #[cfg(feature = "emacs")]
    pub emacs_charset_map: FxHashMap<SmallString, Vec<FamilyId>>,
    #[cfg(feature = "emacs")]
    pub emacs_script_map: FxHashMap<SmallString, Vec<FamilyId>>,
    pub cjk: [Fallbacks; 5],
    pub generic: [Option<FamilyId>; 13],
}

impl Default for StaticIndex {
    fn default() -> Self {
        let fallbacks = Fallbacks::new();
        Self {
            base: BaseIndex::default(),
            families: Vec::new(),
            script_map: Default::default(),
            script_tag_map: Default::default(),
            language_tag_map: Default::default(),
            #[cfg(feature = "emacs")]
            emacs_charset_map: Default::default(),
            #[cfg(feature = "emacs")]
            emacs_script_map: Default::default(),
            cjk: [fallbacks; 5],
            generic: [None; 13],
        }
    }
}

impl StaticIndex {
    pub fn setup_default_fallbacks(&mut self) {
        use super::system::*;
        use Cjk::*;
        use Script::*;
        match OS {
            Os::Windows => {
                // Simplified Chinese
                self.cjk[Simplified as usize] =
                    self.find_fallbacks(&["microsoft yahei", "simsun", "simsun-extb"]);
                // Traditional Chinese
                self.cjk[Traditional as usize] =
                    self.find_fallbacks(&["microsoft jhenghei", "pmingliu", "pmingliu-extb"]);
                self.cjk[Cjk::None as usize] = self.cjk[Traditional as usize];
                // Japanese
                self.cjk[Japanese as usize] = self.find_fallbacks(&[
                    "meiryo",
                    "yu gothic",
                    "microsoft yahei",
                    "simsun",
                    "simsun-extb",
                ]);
                // Korean
                self.cjk[Korean as usize] = self.find_fallbacks(&[
                    "malgun gothic",
                    "gulim",
                    "microsoft yahei",
                    "simsun",
                    "simsun-extb",
                ]);
                self.map_script(Latin, &["times new roman"]);
                self.map_script(Arabic, &["tahoma", "segoe ui"]);
                self.map_script(Armenian, &["segoe ui", "sylfaen"]);
                self.map_script(Bengali, &["nirmala ui", "vrinda"]);
                self.map_script(Brahmi, &["segoe ui historic"]);
                self.map_script(Braille, &["segoe ui symbol"]);
                self.map_script(Buginese, &["leelawadee ui"]);
                self.map_script(CanadianAboriginal, &["gadugi", "euphemia"]);
                self.map_script(Carian, &["segoe ui historic"]);
                self.map_script(Devanagari, &["nirmala ui", "mangal"]);
                self.map_script(Hebrew, &["david", "segoe ui", "calibri"]);
                self.map_script(Hangul, &["malgun gothic", "gulim"]);
                self.map_script(Myanmar, &["myanmar text"]);
                self.map_script(Malayalam, &["nirmala ui", "kartika"]);
                self.map_script(Han, &["microsoft yahei", "simsun", "simsun-extb"]);
                self.map_script(
                    Hiragana,
                    &["meiryo", "yu gothic", "ms pgothic", "microsoft yahei"],
                );
                self.map_script(
                    Katakana,
                    &["meiryo", "yu gothic", "ms pgothic", "microsoft yahei"],
                );
                self.map_script(Kharoshthi, &["segoe ui historic"]);
                self.map_script(
                    Khmer,
                    &[
                        "leelawadee ui",
                        "khmer ui",
                        "khmer os",
                        "moolboran",
                        "daunpenh",
                    ],
                );
                self.map_script(
                    Lao,
                    &[
                        "leelawadee ui",
                        "lao ui",
                        "dokchampa",
                        "saysettha ot",
                        "phetsarath ot",
                        "code2000",
                    ],
                );
                self.map_script(Lisu, &["segoe ui"]);
                self.map_script(
                    Syriac,
                    &["estrangelo edessa", "estrangelo nisibin", "code2000"],
                );
                self.map_script(Thai, &["tahoma", "leelawadee ui", "leelawadee"]);
                self.map_script(
                    Tibetan,
                    &["microsoft himalaya", "jomolhari", "tibetan machine uni"],
                );
                self.map_script(Vai, &["ebrima"]);
                self.map_script(Yi, &["microsoft yi baiti", "nuosu sil", "code2000"]);
            }
            Os::MacOs => {
                // Simplified Chinese
                self.cjk[Simplified as usize] = self.find_fallbacks(&["pingfang sc"]);
                // Traditional Chinese
                self.cjk[Traditional as usize] = self.find_fallbacks(&["pingfang tc"]);
                self.cjk[Cjk::None as usize] = self.cjk[Traditional as usize];
                // Japanese
                self.cjk[Japanese as usize] =
                    self.find_fallbacks(&["hiragino kaku gothic pron w3"]);
                // Korean
                self.cjk[Korean as usize] = self.find_fallbacks(&["apple sd gothic neo"]);
                self.map_script(Latin, &["times", "times new roman"]);
                self.map_script(Arabic, &["geeza pro"]);
                self.map_script(
                    Devanagari,
                    &[
                        "itf devanagari",
                        "kohinoor devanagari",
                        "devanagari sangam mn",
                        "devanagari mt",
                    ],
                );
                self.map_script(Bengali, &[]);
                self.map_script(Myanmar, &["noto sans myanmar", "myanmar mn"]);
                self.map_script(Malayalam, &["malayalam mn"]);
                self.map_script(Hebrew, &["lucida grande", "arial hebrew"]);
            }
            _ => {
                self.map_script(
                    Latin,
                    &[
                        "liberation sans",
                        "dejavu sans",
                        "ubuntu",
                        "source sans pro",
                    ],
                );
                self.map_script(Arabic, &["noto sans arabic"]);
                self.map_script(Hebrew, &["noto sans hebrew", "noto serif hebrew"]);
                self.map_script(Bengali, &["noto sans bengali", "noto serif bengali"]);
                self.map_script(
                    Devanagari,
                    &["noto sans devanagari", "noto serif devanagari"],
                );
                self.map_script(Malayalam, &["noto sans malayalam", "noto serif malayalam"]);
                self.map_script(Myanmar, &["noto sans myanmar", "noto serif myanmar"]);
            }
        }
    }

    pub fn setup_default_generic(&mut self) {
        use super::system::*;
        use GenericFamily::*;
        match OS {
            Os::Windows => {
                self.generic[SansSerif as usize] = self.find_family(&["arial"]);
                self.generic[Serif as usize] = self.find_family(&["times new roman"]);
                self.generic[Monospace as usize] = self.find_family(&["courier new"]);
                self.generic[Fantasy as usize] = self.find_family(&["impact"]);
                self.generic[Cursive as usize] = self.find_family(&["comic sans ms"]);
                self.generic[SystemUI as usize] = self.find_family(&["segoe ui"]);
                self.generic[Emoji as usize] = self.find_family(&["segoe ui emoji"]);
            }
            Os::MacOs => {
                self.generic[SansSerif as usize] = self.find_family(&["helvetica"]);
                self.generic[Serif as usize] = self.find_family(&["times"]);
                self.generic[Monospace as usize] = self.find_family(&["courier"]);
                self.generic[Fantasy as usize] = self.find_family(&["papyrus"]);
                self.generic[Cursive as usize] = self.find_family(&["apple chancery"]);
                self.generic[SystemUI as usize] = self.find_family(&["system font", "helvetica"]);
                self.generic[Emoji as usize] = self.find_family(&["apple color emoji"]);
            }
            Os::Ios => {
                self.generic[SansSerif as usize] = self.find_family(&["helvetica"]);
                self.generic[Serif as usize] = self.find_family(&["times new roman"]);
                self.generic[Monospace as usize] = self.find_family(&["courier"]);
                self.generic[Fantasy as usize] = self.find_family(&["papyrus"]);
                self.generic[Cursive as usize] = self.find_family(&["snell roundhand"]);
                self.generic[SystemUI as usize] = self.find_family(&["system font", "helvetica"]);
                self.generic[Emoji as usize] = self.find_family(&["apple color emoji"]);
            }
            Os::Android => {
                self.generic[SansSerif as usize] = self.find_family(&["roboto"]);
                self.generic[Serif as usize] = self.find_family(&["noto serif", "droid serif"]);
                self.generic[Monospace as usize] = self.find_family(&["droid sans mono"]);
                self.generic[Fantasy as usize] = self.find_family(&["noto serif"]);
                self.generic[Cursive as usize] = self.find_family(&["dancing script"]);
                self.generic[SystemUI as usize] = self.find_family(&["roboto"]);
                self.generic[Emoji as usize] = self.find_family(&["noto color emoji"]);
            }
            Os::Unix | Os::Other => {
                self.generic[SansSerif as usize] =
                    self.find_family(&["liberation sans", "dejavu sans"]);
                self.generic[Serif as usize] = self.find_family(&[
                    "liberation serif",
                    "dejavu serif",
                    "noto serif",
                    "times new roman",
                ]);
                self.generic[Monospace as usize] = self.find_family(&["dejavu sans mono"]);
                self.generic[Fantasy as usize] =
                    self.find_family(&["liberation serif", "dejavu serif"]);
                self.generic[Cursive as usize] =
                    self.find_family(&["liberation serif", "dejavu serif"]);
                self.generic[SystemUI as usize] =
                    self.find_family(&["liberation sans", "dejavu sans"]);
                self.generic[Emoji as usize] = self.find_family(&["noto color emoji", "emoji one"]);
            }
        }
    }

    pub fn emoji_family(&self) -> Option<FamilyId> {
        self.generic[GenericFamily::Emoji as usize]
    }

    pub fn fallbacks(&self, script: Script, cjk: Cjk) -> &[FamilyId] {
        if script == Script::Han {
            self.cjk[cjk as usize].get()
        } else {
            self.script_map.get(&script).map(|f| f.get()).unwrap_or(&[])
        }
    }

    fn map_script(&mut self, script: Script, families: &[&str]) {
        let fallbacks = self.find_fallbacks(families);
        if fallbacks.len() != 0 {
            self.script_map.insert(script, fallbacks);
        }
    }

    fn find_family(&self, families: &[&str]) -> Option<FamilyId> {
        for family in families {
            if let Some(id) = self.base.family_map.get(*family) {
                return Some(*id);
            }
        }
        None
    }

    fn find_fallbacks(&self, families: &[&str]) -> Fallbacks {
        let mut fallbacks = Fallbacks::new();
        for family in families {
            if let Some(id) = self.base.family_map.get(*family) {
                if !fallbacks.push(*id) {
                    break;
                }
            }
        }
        fallbacks
    }
}

impl StaticIndex {
    /// Returns a font entry that matches the specified family and
    /// attributes.
    pub fn query<'a>(
        &'a self,
        family: impl Into<FamilyKey<'a>>,
        attributes: impl Into<Attributes>,
    ) -> Option<FontEntry<'a>> {
        let family = self.family_by_key(family)?;
        let attrs = attributes.into();
        let font_id = family.data.query(attrs)?;
        let data = self.base.fonts.get(font_id.to_usize())?;
        Some(FontEntry {
            index: &self.base,
            family: family.data,
            data,
        })
    }

    /// Returns a list font entries that matches the specified family and
    /// attributes.
    #[cfg(feature = "emacs")]
    pub fn list<'a>(&'a self, spec: FontSpec) -> Vec<FontEntry<'a>> {
        let filter = |family: FamilyKey<'a>,
                      stretch: Option<Stretch>,
                      weight: Option<Weight>,
                      style: Option<Style>,
                      otf: Option<OpentypeSpec>| {
            let family = self.family_by_key(family);
            if family.is_none() {
                return vec![];
            }
            let family = family.unwrap();
            let fonts = family.data.list(stretch, weight, style, otf);
            let fonts = fonts
                .iter()
                .filter_map(|font_id| {
                    if let Some(data) = self.base.fonts.get(font_id.to_usize()) {
                        return Some(FontEntry {
                            index: &self.base,
                            family: family.data,
                            data,
                        });
                    }
                    None
                })
                .collect();
            return fonts;
        };

        //TODO impl spacing
        let FontSpec {
            width,
            weight,
            slant,
            spacing,
            otf,
            ..
        } = spec.clone();

        if let Some(_) = spacing {
            warn!("spacing is not yet supported");
        }
        self.families_by_spec(spec)
            .iter()
            .map(|family_id| FamilyKey::from(*family_id))
            .map(|key| filter(key, width, weight, slant, otf.clone()))
            .flatten()
            .collect()
    }

    #[cfg(feature = "emacs")]
    pub fn match_<'a>(&'a self, spec: FontSpec) -> Option<FontEntry<'a>> {
        //TODO impl spacing
        let FontSpec {
            width,
            weight,
            slant,
            spacing,
            otf,
            ..
        } = spec.clone();
        if let Some(_) = spacing {
            warn!("spacing is not yet supported");
        }
        let attrs = Attributes::new(
            width.unwrap_or(Stretch::NORMAL),
            weight.unwrap_or(Weight::NORMAL),
            slant.unwrap_or(Style::Normal),
        );

        let query = |family: FamilyId, attributes: Attributes, otf: Option<OpentypeSpec>| {
            let family = self.family_by_key(family)?;
            let attrs = attributes.into();
            let font_id = family.data.match_(attrs, otf)?;
            let data = self.base.fonts.get(font_id.to_usize())?;
            Some(FontEntry {
                index: &self.base,
                family: family.data,
                data,
            })
        };

        self.families_by_spec(spec)
            .iter()
            .find_map(|family| query(*family, attrs, otf.clone()))
    }

    //TODO impl foundry, ref to fontconfig foundry implementation
    //TODO impl XLFD-style or fontconfig-style font name, more defails from Emacs info
    #[cfg(feature = "emacs")]
    pub fn families_by_spec<'a>(
        &'a self,
        FontSpec {
            family,
            foundry,
            registry,
            name,
            script,
            lang,
            otf,
            ..
        }: FontSpec,
    ) -> Vec<FamilyId> {
        if let Some(_) = foundry {
            warn!("foundry is not yet supported");
        }

        if let Some(_) = name {
            warn!("name is not yet supported");
        }

        let intersection = |families: Vec<FamilyId>, _families: Option<Vec<FamilyId>>| {
            if _families.is_none() {
                return vec![];
            }
            let _families = _families.unwrap();

            if families.is_empty() && !_families.is_empty() {
                return _families.clone();
            } else {
                let unique_a = families.iter().collect::<HashSet<_>>();
                let unique_b = _families.iter().collect::<HashSet<_>>();

                return unique_a
                    .intersection(&unique_b)
                    .map(|id| **id)
                    .collect::<Vec<_>>();
            }
        };

        let mut families = family
            .map(|family| {
                self.family_by_name(family.as_str())
                    .map(|entry| vec![entry.data.id])
            })
            .flatten()
            .unwrap_or(vec![]);

        if let Some((script, ..)) = otf {
            let _families = self.families_by_script(script);
            families = intersection(families, _families);
        }

        if let Some(regexp) = registry {
            let _families = self.families_by_charset(regexp);
            families = intersection(families, _families);
        }

        if let Some(script) = script {
            let _families = self.families_by_emacs_script(script);
            families = intersection(families, _families);
        }

        if let Some(lang) = lang {
            let _f = lang
                .to_639_1()
                .map(|lang| Language::parse(lang))
                .flatten()
                .map(|lang| lang.to_opentype())
                .flatten()
                .map(|lang| self.families_by_lang(lang))
                .flatten();
            families = intersection(families, _f);
        }

        families.dedup();
        families
    }

    #[cfg(feature = "emacs")]
    pub fn families_by_charset(&self, regexp: String) -> Option<Vec<FamilyId>> {
        let string_match_p = |regexp: &str, string: &str, _start: Option<i64>| {
            let re = Regex::new(regexp).ok().unwrap();
            re.is_match(string).ok().unwrap()
        };
        self.emacs_charset_map
            .iter()
            .filter_map(|(charset, families)| {
                if string_match_p(regexp.as_str(), charset.as_str(), Some(0)) {
                    return Some(families.clone());
                }
                None
            })
            .reduce(|acc, e| {
                let mut families = [&acc[..], &e[..]].concat();
                families.dedup();
                families
            })
    }

    #[cfg(feature = "emacs")]
    pub fn families_by_emacs_script(&self, script: String) -> Option<Vec<FamilyId>> {
        self.emacs_script_map
            .iter()
            .find_map(|(script_, families)| {
                if script.as_str() == script_.as_str() {
                    return Some(families.clone());
                }
                None
            })
    }

    pub fn families_by_script(&self, lang: Tag) -> Option<Vec<FamilyId>> {
        self.script_tag_map.iter().find_map(|(lang_, families)| {
            if lang == *lang_ {
                return Some(families.clone());
            }
            None
        })
    }

    pub fn families_by_lang(&self, lang: Tag) -> Option<Vec<FamilyId>> {
        self.language_tag_map.iter().find_map(|(lang_, families)| {
            if lang == *lang_ {
                return Some(families.clone());
            }
            None
        })
    }

    /// Returns a font family entry for the specified family key.
    pub fn family_by_key<'a>(&'a self, key: impl Into<FamilyKey<'a>>) -> Option<FamilyEntry<'a>> {
        match key.into() {
            FamilyKey::Id(id) => self.family_by_id(id),
            FamilyKey::Name(name) => self.family_by_name(name),
            FamilyKey::Generic(generic) => {
                self.family_by_id(self.generic.get(generic as usize).copied()??)
            }
        }
    }

    /// Returns a font family entry for the specified name.
    pub fn family_by_name<'a>(&'a self, name: &str) -> Option<FamilyEntry<'a>> {
        let mut s = LowercaseString::new();
        let name = s.get(name)?;
        let id = if let Some(generic) = GenericFamily::parse(name) {
            self.generic.get(generic as usize).copied()??
        } else {
            *self.base.family_map.get(name)?
        };

        self.family_by_id(id)
    }

    /// Returns a font family entry for the specified identifier.
    pub fn family_by_id<'a>(&'a self, id: FamilyId) -> Option<FamilyEntry<'a>> {
        let data = self.families.get(id.to_usize())?;
        Some(FamilyEntry {
            index: &self.base,
            data,
        })
    }

    /// Returns a font entry for the specified identifier.
    pub fn font_by_id<'a>(&'a self, id: FontId) -> Option<FontEntry<'a>> {
        let data = self.base.fonts.get(id.to_usize())?;
        let family = self.families.get(data.family.to_usize())?;
        Some(FontEntry {
            index: &self.base,
            family,
            data,
        })
    }
}

/// Font family entry in a library.
#[derive(Copy, Clone)]
pub struct FamilyEntry<'a> {
    index: &'a BaseIndex,
    data: &'a FamilyData,
}

impl<'a> FamilyEntry<'a> {
    /// Returns the family identifier.
    pub fn id(&self) -> FamilyId {
        self.data.id
    }

    /// Returns the name of the family.
    pub fn name(&self) -> &str {
        self.data.name.as_str()
    }

    /// Returns an iterator over the fonts in the family.
    pub fn fonts(&'a self) -> impl Iterator<Item = FontEntry<'a>> + 'a {
        self.data.fonts.iter().filter_map(move |f| {
            let data = self.index.fonts.get(f.id.to_usize())?;
            Some(FontEntry {
                index: self.index,
                family: self.data,
                data,
            })
        })
    }
}

/// Font entry in a library.
#[derive(Copy, Clone)]
pub struct FontEntry<'a> {
    index: &'a BaseIndex,
    family: &'a FamilyData,
    data: &'a FontData,
}

impl<'a> FontEntry<'a> {
    /// Returns the font identifier.
    pub fn id(&self) -> FontId {
        self.data.id
    }

    /// Returns the font source.
    pub fn source(&self) -> SourceEntry<'a> {
        SourceEntry {
            index: self.index,
            data: &self.index.sources[self.data.source.to_usize()],
        }
    }

    /// Returns the index of the font in the source.
    pub fn index(&self) -> u32 {
        self.data.index
    }

    /// Returns the offset to the font table directory in the source.
    pub fn offset(&self) -> u32 {
        self.data.offset
    }

    /// Returns the family entry.
    pub fn family(&self) -> FamilyEntry<'a> {
        FamilyEntry {
            index: self.index,
            data: self.family,
        }
    }

    /// Returns the family name.
    pub fn family_name(&self) -> &str {
        self.family.name.as_str()
    }

    /// Returns the font attributes.
    pub fn attributes(&self) -> Attributes {
        self.data.attributes
    }

    pub fn cache_key(&self) -> CacheKey {
        self.data.key
    }

    pub fn selector(
        &self,
        attrs: RequestedAttributes,
    ) -> (FontId, Attributes, RequestedAttributes) {
        (self.data.id, self.data.attributes, attrs)
    }
}

/// Source entry in a library.
#[derive(Copy, Clone)]
pub struct SourceEntry<'a> {
    pub index: &'a BaseIndex,
    data: &'a SourceData,
}

impl<'a> SourceEntry<'a> {
    /// Returns the source identifier.
    pub fn id(&self) -> SourceId {
        self.data.id
    }

    /// Returns the path of the source, if it is represented by a file.
    pub fn path(&self) -> Option<&Path> {
        match &self.data.kind {
            SourceKind::Memory(..) => None,
            SourceKind::File(data) => Some(&data.path),
        }
    }
}

// Invert the escaping of parens. i.e. \( => ( and ( => \(
// copied from https://github.com/CeleritasCelery/rune/blob/master/src/search.rs#L38
#[cfg(feature = "emacs")]
#[allow(dead_code)]
fn lisp_regex_to_rust(regexp: &str) -> String {
    let mut norm_regex = String::new();
    let mut chars = regexp.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '(' => norm_regex.push_str("\\("),
            ')' => norm_regex.push_str("\\)"),
            '\\' if matches!(chars.peek(), Some('(' | ')')) => {
                norm_regex.push(chars.next().unwrap());
            }
            c => norm_regex.push(c),
        }
    }
    norm_regex
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
