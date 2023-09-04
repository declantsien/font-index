extern crate font_index;

use font_index::FontCache;
use font_index::FontIndex;
use font_index::GenericFamily;
use font_index::SourceKind;
use swash::StringId;
use swash::{Attributes, Stretch, Style, Weight};

fn main() {

    let family_names: Vec<String> = FontIndex::global()
        .families
        .clone()
        .iter()
        .map(|data| data.name.to_string())
        .collect();
    println!("names {:?}", family_names);

    let stretch = Stretch::NORMAL;
    let style = Style::Normal;
    let weight = Weight::NORMAL;

    if let Some(font) = FontIndex::global().query(
        "Monospace",
        Attributes::new(stretch, weight, style),
    ) {
        println!("monospace font {:?} {:?}", font.family_name(), font.attributes());
    }

    if let Some(font) = FontIndex::global().query(
        "Sans Serif",
        Attributes::new(stretch, weight, style),
    ) {
        println!("Sans Serif font {:?} {:?}", font.family_name(), font.attributes());
    }

    if let Some(family) = FontIndex::global().family_by_key("serif") {
        family
            .fonts()
            .for_each(|font| println!("font {:?} {:?}", font.family_name(), font.attributes()))
    }


    let mut cache = FontCache::default();
    if let Some(font) = cache.query("WenQuanYi Micro Hei", Attributes::new(stretch, weight, style)) {
	let font = &cache.get(font.id()).unwrap();
        println!(
            "data {:?}",
            &font.as_ref().data[..50]
        );
	font.writing_systems().for_each(|w| {
	    println!("language {:?}", w.language());
	    println!("script {:?}", w.script())
	})

    }

    FontIndex::global().base.fonts.iter().for_each(|font_data| {
	let font = &cache.get(font_data.id).unwrap();
	println!("--------{:?}-----", font.localized_strings().find_by_id(StringId::Family, None).unwrap().to_string());
	font.writing_systems().for_each(|w| {
	    println!("language {:?}", w.language());
	    println!("script {:?}", w.script())
	})
    });

    FontIndex::global().base.sources.iter().for_each(|source_data| {
	match &source_data.kind {
            SourceKind::File(file) => {
		println!("file {:?}", file.path.as_path());
	    },
            SourceKind::Memory(data) => {},
        }
    });
}
