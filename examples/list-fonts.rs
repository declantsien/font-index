extern crate font_index;

use font_index::FontCache;
use font_index::FontIndex;
use font_index::GenericFamily;
use font_index::SourceKind;
use swash::text::Language;
use swash::text::Script;
use swash::StringId;
use swash::Tag;
use swash::{Attributes, Stretch, Style, Weight};

fn main() {
    // let family_names: Vec<String> = FontIndex::global()
    //     .families
    //     .clone()
    //     .iter()
    //     .map(|data| data.name.to_string())
    //     .collect();
    // println!("names {:?}", family_names);

    let stretch = Stretch::NORMAL;
    let style = Style::Normal;
    let weight = Weight::NORMAL;
    let index = FontIndex::global();

    // if let Some(font) = FontIndex::global().query(
    //     "Monospace",
    //     Attributes::new(stretch, weight, style),
    // ) {
    //     println!("monospace font {:?} {:?}", font.family_name(), font.attributes());
    // }
    // println!("family_map {:?}", FontIndex::global().base.family_map);
    // 文泉驛微米黑
    // WenQuanYi Micro Hei
    // 文泉驿微米黑
    // 文泉驛微米黑
    // 文泉驿微米黑
    // 文泉驛微米黑

    // println!("{:?}", font_index::util::string::SmallString::new("文泉驛微米黑"));
    if let Some(font) =
        FontIndex::global().query("文泉驛微米黑", Attributes::new(stretch, weight, style))
    {
        println!(
            "Sans Serif font {:?} {:?}",
            font.family_name(),
            font.attributes()
        );
    }

    // if let Some(family) = FontIndex::global().family_by_key("serif") {
    //     family
    //         .fonts()
    //         .for_each(|font| println!("font {:?} {:?}", font.family_name(), font.attributes()))
    // }

    // let mut cache = FontCache::default();
    // if let Some(font) = cache.query("WenQuanYi Micro Hei", Attributes::new(stretch, weight, style)) {
    // 	let font = &cache.get(font.id()).unwrap();
    //     println!(
    //         "data {:?}",
    //         &font.as_ref().data[..50]
    //     );

    // 	for name in font.localized_strings()
    //             .clone()
    //             .filter(|name| name.id() == StringId::Family && name.is_unicode())
    //         {
    //             // if count >= self.font.all_names.len() {
    //             //     self.font.all_names.push(String::default());
    //             // }
    //             // let name_buf = &mut self.font.all_names[count];
    //             // count += 1;
    //             // name_buf.clear();
    //             // for ch in name.chars() {
    //             //     name_buf.extend(ch.to_lowercase());
    //             // }
    // 		println!("{}", name);
    //         }
    // 	font.writing_systems().for_each(|w| {
    // 	    println!("language {:?}", w.language());
    // 	    println!("script {:?}", w.script())
    // 	})
    // }

    // FontIndex::global().base.fonts.iter().for_each(|font_data| {
    // 	let font = &cache.get(font_data.id).unwrap();

    // 	println!("--------{:?}-----", font.localized_strings().find_by_id(StringId::Family, None).unwrap().to_string());
    // 	font.writing_systems().for_each(|w| {
    // 	    println!("language {:?}", w.language());
    // 	    println!("script {:?}", w.script());
    // 	})
    // });

    // let tag = "latn";
    // let script = Script::from_opentype(swash::tag_from_str_lossy(tag)).unwrap();
    // println!("scrp {:?}", script);
    // if let Some(fallbacks) = FontIndex::global().script_map.get(&script) {
    // 	println!("{:?}", fallbacks.len());
    // }

    // FontIndex::global().script_map.iter().for_each(|(script, fallbacks)| {
    // 	println!("{:?}: {:?}", script, fallbacks.len());
    // });

    // FontIndex::global().script_tag_map.iter().for_each(|(tag, fonts)| {
    // 	let tag = tag.to_be_bytes();
    //     if let Ok(s) = core::str::from_utf8(&tag) {
    // 	    println!("{:?}: {:?}", s, fonts.len());
    //     }
    // });

    // FontIndex::global().language_tag_map.iter().for_each(|(tag, fonts)| {
    // 	let tag = tag.to_be_bytes();
    //     if let Ok(s) = core::str::from_utf8(&tag) {
    // 	    println!("{:?}: {:?}", s, fonts);
    //     }
    // });

    FontIndex::global().emacs_charset_map.iter().for_each(|(charset, fonts)| {
	println!("{:?}: {:?}", charset, fonts);
    });

    // if let Some(lang) = Language::parse("zh-Hans") {
    // 	println!("{:?}: {:?}", lang, lang.script());
    // }
    // if let Some(lang) = Language::parse("zh-Hant") {
    // 	println!("{:?}: {:?}", lang, lang.script());
    // }
    if let Some(lang) = Language::parse("zh") {
        print_lang(lang);
        if let Some(lang) = Language::from_opentype(lang.to_opentype().unwrap()) {
            print_lang(lang);
        }
    }

    if let Some(lang) = Language::parse("ko") {
        print_lang(lang);
        if let Some(lang) = Language::from_opentype(lang.to_opentype().unwrap()) {
            print_lang(lang);
        }
    }

    // FontIndex::global().base.fonts.iter().for_each(|font_data| {
    //     let font = &cache.get(font_data.id).unwrap();

    //     println!(
    //         "--------{:?}-----",
    //         font.localized_strings()
    //             .find_by_id(StringId::Family, None)
    //             .unwrap()
    //             .to_string()
    //     );
    //     font.writing_systems().for_each(|w| {
    //         if let Some(s) = w.script() {
    //             println!("language {:?}", w.language());
    //             println!("script {:?}", w.script());
    //         }
    //     })
    // });

    // FontIndex::global().base.sources.iter().for_each(|source_data| {
    // 	match &source_data.kind {
    //         SourceKind::File(file) => {
    // 		println!("file {:?}", file.path.as_path());
    // 	    },
    //         SourceKind::Memory(data) => {},
    //     }
    // });
}

fn print_lang(lang: Language) {
    println!("language: {} ", lang.language());
    if let Some(script) = lang.script() {
        println!("script -{}", script);
    }
    if let Some(region) = lang.region() {
        println!("region -{}", region);
    }
    if let Some(tag) = lang.to_opentype() {
        let tag = tag.to_be_bytes();
        if let Ok(s) = core::str::from_utf8(&tag) {
            println!("tag: ({})", s);
        }
    }
    if let Some(name) = lang.name() {
        println!("name: \"{}\"", name);
    }
}
