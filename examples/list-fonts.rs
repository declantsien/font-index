extern crate font_index;

#[cfg(feature = "emacs")]
use font_index::emacs::FontSpec;
use font_index::FontCache;
use font_index::FontIndex;
use font_index::SourceKind;
use swash::text::Language;
use swash::text::Script;
use swash::StringId;
use swash::{Attributes, Stretch, Style, Weight};

fn main() {
    // List all family names
    let family_names: Vec<String> = FontIndex::global()
        .families
        .clone()
        .iter()
        .map(|data| data.name.to_string())
        .collect();
    println!("family names {:?}", family_names);

    let stretch = Stretch::NORMAL;
    let style = Style::Normal;
    let weight = Weight::NORMAL;

    // Query fontconfig alias
    if let Some(font) =
        FontIndex::global().query("Monospace", Attributes::new(stretch, weight, style))
    {
        println!(
            "monospace font {:?} {:?}",
            font.family_name(),
            font.attributes()
        );
    }

    //Query localized family name
    if let Some(font) =
        FontIndex::global().query("文泉驛微米黑", Attributes::new(stretch, weight, style))
    {
        println!(
            "Query localized family name 文泉驛微米黑: {:?} {:?}",
            font.family_name(),
            font.attributes()
        );
    }

    // Match font according FontSpec
    #[cfg(feature = "emacs")]
    if let Some(font) = FontIndex::global().match_(FontSpec {
        family: None,
        foundry: None,
        width: Some(Stretch::CONDENSED),
        weight: Some(Weight::BOLD),
        slant: None,
        adstyle: None,
        registry: Some("iso8859-1".to_string()),
        size: None,
        dpi: None,
        spacing: None,
        avgwidth: None,
        name: None,
        // script: Some("han".to_string()),
        script: None,
        lang: None,
        otf: None,
    }) {
        println!("gb18030 {:?} {:?}", font.family_name(), font.attributes());
    }

    // List font according FontSpec
    #[cfg(feature = "emacs")]
    FontIndex::global()
        .list(FontSpec {
            // family: Some("Droid Sans".to_string()),
            family: None,
            foundry: None,
            width: None,
            weight: Some(Weight::BOLD),
            slant: None,
            adstyle: None,
            registry: Some("iso8859-1".to_string()),
            size: None,
            dpi: None,
            spacing: None,
            avgwidth: None,
            name: None,
            // script: Some("han".to_string()),
            script: None,
            lang: None,
            otf: None,
        })
        .iter()
        .for_each(|font| {
            println!(
                "list gb18030 {:?} {:?}",
                font.family_name(),
                font.attributes()
            );
        });

    // Query font meta using FontCache
    let mut cache = FontCache::default();
    if let Some(font) = cache.query("Noto Color Emoji", Attributes::new(stretch, weight, style)) {
        let font = &cache.get(font.id()).unwrap();
        println!("data {:?}", &font.as_ref().data[..50]);

        for name in font
            .localized_strings()
            .clone()
            .filter(|name| name.id() == StringId::Manufacturer && name.is_unicode())
        {
            println!("{}", name);
        }
        font.writing_systems().for_each(|w| {
            println!("language {:?}", w.language());
            println!("script {:?}", w.script())
        })
    }

    // FontIndex::global().base.fonts.iter().for_each(|font_data| {
    // 	let font = &cache.get(font_data.id).unwrap();

    // 	println!("--------{:?}-----", font.localized_strings().find_by_id(StringId::Family, None).unwrap().to_string());
    // 	font.writing_systems().for_each(|w| {
    // 	    println!("language {:?}", w.language());
    // 	    println!("script {:?}", w.script());
    // 	})
    // });

    println!("--------Query font for script using opentype tag-----");
    let tag = "latn";
    let script = Script::from_opentype(swash::tag_from_str_lossy(tag)).unwrap();
    if let Some(fallbacks) = FontIndex::global().script_map.get(&script) {
        println!("tag: {tag}, script {script:?}, fonts {:?}", fallbacks.len());
    }
    println!("End--------Query font for script using opentype tag-----\n");

    println!("--------Platform default fonts for script-----");
    FontIndex::global()
        .script_map
        .iter()
        .for_each(|(script, fallbacks)| {
            println!("{:?}: {:?}", script, fallbacks.len());
        });
    println!("End--------Platform default fonts for script-----\n");

    println!("--------List all fonts for opentype script tag-----");
    FontIndex::global()
        .script_tag_map
        .iter()
        .for_each(|(tag, fonts)| {
            let tag = tag.to_be_bytes();
            if let Ok(s) = core::str::from_utf8(&tag) {
                println!("{:?}: {:?}", s, fonts.len());
            }
        });
    println!("End --------List all fonts for opentype script tag-----\n");

    println!("--------List all fonts for opentype language tag-----");
    FontIndex::global()
        .language_tag_map
        .iter()
        .for_each(|(tag, fonts)| {
            let tag = tag.to_be_bytes();
            if let Ok(s) = core::str::from_utf8(&tag) {
                println!("{:?}: {:?}", s, fonts.len());
            }
        });
    println!("End--------List all fonts for opentype language tag-----\n\n");

    #[cfg(feature = "emacs")]
    {
        println!("--------List all fonts for Emacs charset-----");
        FontIndex::global()
            .emacs_charset_map
            .iter()
            .for_each(|(charset, fonts)| {
                println!("{:?}: {:?}", charset, fonts.len());
            });
        println!("End--------List all fonts for Emacs charset-----\n\n");
    }

    // Debug lang
    if let Some(lang) = Language::parse("zh-Hans") {
        debug_lang(lang);
        println!("{:?}: {:?}", lang, lang.script());
    }

    if let Some(lang) = Language::parse("zh-Hant") {
        println!("{:?}: {:?}", lang, lang.script());
    }

    if let Some(lang) = Language::parse("zh") {
        if let Some(lang) = Language::from_opentype(lang.to_opentype().unwrap()) {
            println!("{lang}");
        }
    }

    // if let Some(lang) = Language::parse("ko") {
    //     print_lang(lang);
    //     if let Some(lang) = Language::from_opentype(lang.to_opentype().unwrap()) {
    //         print_lang(lang);
    //     }
    // }

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
    // 	    let features = w.features();
    // 	    features.for_each(|feature| {
    // 		let tag = feature.tag().to_be_bytes();
    // 		if let Ok(tag) = core::str::from_utf8(&tag) {
    // 		    println!("tag: {}, name: {:?}, action: {:?} ", tag, feature.name(), feature.action());
    // 		}

    // 	    });

    //     })
    // });

    println!("--------Source kind-----");
    FontIndex::global()
        .base
        .sources
        .iter()
        .for_each(|source_data| match &source_data.kind {
            SourceKind::File(file) => {
                println!("file {:?}", file.path.as_path());
            }
            SourceKind::Memory(_data) => {
                println!("Shared data");
            }
        });
    println!("End--------Source kind-----");
}

fn debug_lang(lang: Language) {
    print!("language: {} ", lang.language());
    if let Some(script) = lang.script() {
        print!("script: {} ", script);
    }
    if let Some(region) = lang.region() {
        print!("region {} ", region);
    }
    if let Some(tag) = lang.to_opentype() {
        let tag = tag.to_be_bytes();
        if let Ok(s) = core::str::from_utf8(&tag) {
            print!("tag: ({})", s);
        }
    }
    if let Some(name) = lang.name() {
        println!("name: \"{}\"", name);
    }
}
