//! Data structs for a font index.

use super::{
    shared_data::{SharedData, WeakSharedData},
    types::{FamilyId, FontId, SourceId},
};
use crate::{emacs::OpentypeSpec, util::string::SmallString};
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;
use swash::{Attributes, CacheKey, Stretch, Style, Tag, Weight};

#[derive(Clone)]
pub struct FamilyData {
    pub id: FamilyId,
    pub name: SmallString,
    pub fonts: Vec<FamilyFontData>,
    pub has_stretch: bool,
}

#[derive(Clone)]
pub struct FamilyFontData {
    pub id: FontId,
    pub stretch: Stretch,
    pub weight: Weight,
    pub style: Style,
    pub writing_systems: Vec<(Tag, Tag, Vec<Tag>)>,
}

impl FamilyData {
    pub fn contains(&self, stretch: Stretch, weight: Weight, style: Style) -> bool {
        for font in &self.fonts {
            if font.stretch == stretch && font.weight == weight && font.style == style {
                return true;
            }
        }
        false
    }

    pub fn list(
        &self,
        stretch: Option<Stretch>,
        weight: Option<Weight>,
        style: Option<Style>,
        otf: Option<OpentypeSpec>,
    ) -> Vec<FontId> {
        self.fonts
            .iter()
            .filter(|font| {
                if let Some((script_tag, language_tag, included_tags, excluded_tags)) = &otf {
                    if font
                        .writing_systems
                        .iter()
                        .find(|w| {
                            w.0 == *script_tag
                                && (language_tag.is_some() && w.1 == language_tag.unwrap())
                                && (included_tags.is_some()
                                    && included_tags
                                        .as_ref()
                                        .unwrap()
                                        .iter()
                                        .all(|feature_tag| w.2.contains(feature_tag)))
                                && (excluded_tags.is_some()
                                    && excluded_tags
                                        .as_ref()
                                        .unwrap()
                                        .iter()
                                        .all(|feature_tag| !w.2.contains(feature_tag)))
                        })
                        .is_none()
                    {
                        return false;
                    }
                }
                true
            })
            .filter(|font| {
                if stretch.is_some() && stretch.unwrap() != font.stretch {
                    return false;
                }
                true
            })
            .filter(|font| {
                if style.is_some() && style.unwrap() != font.style {
                    return false;
                }
                true
            })
            .filter(|font| {
                if weight.is_some() && weight.unwrap() != font.weight {
                    return false;
                }
                true
            })
            .map(|font| font.id)
            .collect()
    }

    /// Returns the font that most closely matches the specified attributes.
    pub fn match_(&self, attributes: Attributes, otf: Option<OpentypeSpec>) -> Option<FontId> {
        let filtered_fonts = self.fonts.iter().filter(|font| {
            if let Some((script_tag, language_tag, included_tags, excluded_tags)) = &otf {
                if font
                    .writing_systems
                    .iter()
                    .find(|w| {
                        w.0 == *script_tag
                            && (language_tag.is_some() && w.1 == language_tag.unwrap())
                            && (included_tags.is_some()
                                && included_tags
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .all(|feature_tag| w.2.contains(feature_tag)))
                            && (excluded_tags.is_some()
                                && excluded_tags
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .all(|feature_tag| !w.2.contains(feature_tag)))
                    })
                    .is_none()
                {
                    return false;
                }
            }
            true
        });

        let style = attributes.style();
        let weight = attributes.weight();
        let stretch = attributes.stretch();

        let mut matching_stretch = Stretch::NORMAL;
        if self.has_stretch {
            if let Some(val) = filtered_fonts.clone().find_map(|font| {
                let val = font.stretch;
                let font_stretch = if stretch <= Stretch::NORMAL {
                    if val > Stretch::NORMAL {
                        val.raw() as i32 - Stretch::NORMAL.raw() as i32
                            + Stretch::ULTRA_EXPANDED.raw() as i32
                    } else {
                        val.raw() as i32
                    }
                } else {
                    if val < Stretch::NORMAL {
                        val.raw() as i32 - Stretch::NORMAL.raw() as i32
                            + Stretch::ULTRA_EXPANDED.raw() as i32
                    } else {
                        val.raw() as i32
                    }
                };

                let offset = (font_stretch - stretch.raw() as i32).abs();
                if offset < i32::MAX {
                    return Some(val);
                }
                None
            }) {
                matching_stretch = val;
            }
        }

        let filtered_fonts = filtered_fonts.filter(|f| f.stretch == matching_stretch);

        let mut matching_style;
        match style {
            Style::Normal => {
                matching_style = Style::Italic;
                for font in filtered_fonts.clone() {
                    let val = font.style;
                    match val {
                        Style::Normal => {
                            matching_style = style;
                            break;
                        }
                        Style::Oblique(_) => {
                            matching_style = val;
                        }
                        _ => {}
                    }
                }
            }
            Style::Oblique(_) => {
                matching_style = Style::Normal;
                for font in filtered_fonts.clone() {
                    let val = font.style;
                    match val {
                        Style::Oblique(_) => {
                            matching_style = style;
                            break;
                        }
                        Style::Italic => {
                            matching_style = val;
                        }
                        _ => {}
                    }
                }
            }
            Style::Italic => {
                matching_style = Style::Normal;
                for font in filtered_fonts.clone() {
                    let val = font.style;
                    match val {
                        Style::Italic => {
                            matching_style = style;
                            break;
                        }
                        Style::Oblique(_) => {
                            matching_style = val;
                        }
                        _ => {}
                    }
                }
            }
        }

        let filtered_fonts = filtered_fonts.filter(|f| f.style == matching_style);

        // If the desired weight is inclusively between 400 and 500
        if weight >= Weight(400) && weight <= Weight(500) {
            // weights greater than or equal to the target weight are checked
            // in ascending order until 500 is hit and checked
            for font in filtered_fonts
                .clone()
                .filter(|f| f.weight >= weight && f.weight <= Weight(500))
            {
                return Some(font.id);
            }
            // followed by weights less than the target weight in descending
            // order
            for font in filtered_fonts.clone().rev().filter(|f| f.weight < weight) {
                return Some(font.id);
            }
            // followed by weights greater than 500, until a match is found
            return filtered_fonts
                .filter(|f| f.weight > Weight(500))
                .map(|f| f.id)
                .next();
        // If the desired weight is less than 400
        } else if weight < Weight(400) {
            // weights less than or equal to the desired weight are checked in
            // descending order
            for font in filtered_fonts.clone().rev().filter(|f| f.weight <= weight) {
                return Some(font.id);
            }
            // followed by weights above the desired weight in ascending order
            // until a match is found
            return filtered_fonts
                .filter(|f| f.weight > weight)
                .map(|f| f.id)
                .next();
        // If the desired weight is greater than 500
        } else {
            // weights greater than or equal to the desired weight are checked
            // in ascending order
            for font in filtered_fonts.clone().filter(|f| f.weight >= weight) {
                return Some(font.id);
            }
            // followed by weights below the desired weight in descending order
            // until a match is found
            return filtered_fonts
                .rev()
                .filter(|f| f.weight < weight)
                .map(|f| f.id)
                .next();
        }
    }

    /// Returns the font that most closely matches the specified attributes.
    pub fn query(&self, attributes: Attributes) -> Option<FontId> {
        self.match_(attributes, None)
    }
}

#[derive(Copy, Clone)]
pub struct FontData {
    pub id: FontId,
    pub family: FamilyId,
    pub source: SourceId,
    pub index: u32,
    pub offset: u32,
    pub attributes: Attributes,
    pub key: CacheKey,
}

pub struct FileData {
    pub path: PathBuf,
    pub timestamp: SystemTime,
    pub mmap: bool,
    pub status: RwLock<FileDataStatus>,
}

impl FileData {
    pub fn get(&self) -> Option<SharedData> {
        {
            let status = self.status.read().unwrap();
            match *status {
                FileDataStatus::Error => return None,
                FileDataStatus::Present(ref data) => {
                    if let Some(data) = data.upgrade() {
                        return Some(data);
                    }
                }
                FileDataStatus::Empty => {}
            }
        }
        let mut status = self.status.write().unwrap();
        // If we raced with another writer, the data may have already been
        // loaded, so check again.
        match *status {
            FileDataStatus::Error => return None,
            FileDataStatus::Present(ref data) => {
                if let Some(data) = data.upgrade() {
                    return Some(data);
                }
            }
            _ => {}
        }
        if let Ok(data) = SharedData::from_file(&self.path, self.mmap, Some(self.timestamp)) {
            *status = FileDataStatus::Present(data.downgrade());
            Some(data)
        } else {
            *status = FileDataStatus::Error;
            None
        }
    }
}

pub enum FileDataStatus {
    Empty,
    Present(WeakSharedData),
    Error,
}

pub struct SourceData {
    pub id: SourceId,
    pub kind: SourceKind,
}

impl SourceData {
    pub fn get(&self) -> Option<SharedData> {
        match &self.kind {
            SourceKind::File(file) => file.get(),
            SourceKind::Memory(data) => Some(data.clone()),
        }
    }
}

pub enum SourceKind {
    Memory(SharedData),
    File(FileData),
}
