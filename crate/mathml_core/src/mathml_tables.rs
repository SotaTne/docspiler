use crate::mathml_xml::{
    AccentPosition, GlyphAssemblyDirection, OperatorForm, OperatorProperty, OperatorStretchAxis,
};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperatorDictionaryStringEntry {
    pub content: &'static str,
    pub stretch_axis: OperatorStretchAxis,
    pub form: OperatorForm,
    pub lspace_em: f32,
    pub rspace_em: f32,
    pub properties: &'static [OperatorProperty],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombiningCharacterEntry {
    pub non_combining: char,
    pub position: AccentPosition,
    pub combining: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonCombiningCharacterEntry {
    pub combining: char,
    pub position: AccentPosition,
    pub non_combining: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphAssemblyCharEntry {
    pub base: char,
    pub direction: GlyphAssemblyDirection,
    pub extender: char,
    pub bottom_or_left: char,
    pub middle: Option<char>,
    pub top_or_right: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItalicMappingEntry {
    pub original: char,
    pub italic: char,
}

#[cfg(rust_analyzer)]
include!("gen/rust_analyzer_appendix_tables.rs");

#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/appendix_tables.rs"));

pub fn full_operator_dictionary_entries() -> &'static [OperatorDictionaryStringEntry] {
    FULL_OPERATOR_DICTIONARY_ENTRIES
}

pub fn find_operator_dictionary_entry(
    content: &str,
    form: OperatorForm,
) -> Option<&'static OperatorDictionaryStringEntry> {
    find_operator_dictionary_entries(content, form).first()
}

pub fn find_operator_dictionary_entries(
    content: &str,
    form: OperatorForm,
) -> &'static [OperatorDictionaryStringEntry] {
    let start = FULL_OPERATOR_DICTIONARY_ENTRIES.partition_point(|entry| {
        compare_operator_key(entry.content, entry.form, content, form) == Ordering::Less
    });
    let end = FULL_OPERATOR_DICTIONARY_ENTRIES.partition_point(|entry| {
        compare_operator_key(entry.content, entry.form, content, form) != Ordering::Greater
    });
    &FULL_OPERATOR_DICTIONARY_ENTRIES[start..end]
}

pub fn full_combining_character_entries() -> &'static [CombiningCharacterEntry] {
    FULL_COMBINING_CHARACTER_ENTRIES
}

pub fn find_combining_character_entry(
    non_combining: char,
    position: AccentPosition,
    combining: char,
) -> Option<&'static CombiningCharacterEntry> {
    FULL_COMBINING_CHARACTER_ENTRIES
        .binary_search_by(|entry| {
            compare_combining_key(
                entry.non_combining,
                entry.position,
                entry.combining,
                non_combining,
                position,
                combining,
            )
        })
        .ok()
        .map(|index| &FULL_COMBINING_CHARACTER_ENTRIES[index])
}

pub fn full_non_combining_character_entries() -> &'static [NonCombiningCharacterEntry] {
    FULL_NON_COMBINING_CHARACTER_ENTRIES
}

pub fn find_non_combining_character_entry(
    combining: char,
    position: AccentPosition,
    non_combining: char,
) -> Option<&'static NonCombiningCharacterEntry> {
    FULL_NON_COMBINING_CHARACTER_ENTRIES
        .binary_search_by(|entry| {
            compare_combining_key(
                entry.combining,
                entry.position,
                entry.non_combining,
                combining,
                position,
                non_combining,
            )
        })
        .ok()
        .map(|index| &FULL_NON_COMBINING_CHARACTER_ENTRIES[index])
}

pub fn full_glyph_assembly_entries() -> &'static [GlyphAssemblyCharEntry] {
    FULL_GLYPH_ASSEMBLY_ENTRIES
}

pub fn find_glyph_assembly_entry(base: char) -> Option<&'static GlyphAssemblyCharEntry> {
    FULL_GLYPH_ASSEMBLY_ENTRIES
        .binary_search_by_key(&base, |entry| entry.base)
        .ok()
        .map(|index| &FULL_GLYPH_ASSEMBLY_ENTRIES[index])
}

pub fn full_italic_mapping_entries() -> &'static [ItalicMappingEntry] {
    FULL_ITALIC_MAPPING_ENTRIES
}

pub fn find_italic_mapping_entry(original: char) -> Option<&'static ItalicMappingEntry> {
    FULL_ITALIC_MAPPING_ENTRIES
        .binary_search_by_key(&original, |entry| entry.original)
        .ok()
        .map(|index| &FULL_ITALIC_MAPPING_ENTRIES[index])
}

pub fn find_italic_variant(original: char) -> Option<char> {
    find_italic_mapping_entry(original).map(|entry| entry.italic)
}

fn compare_operator_key(
    left_content: &str,
    left_form: OperatorForm,
    right_content: &str,
    right_form: OperatorForm,
) -> Ordering {
    left_content
        .cmp(right_content)
        .then(operator_form_rank(left_form).cmp(&operator_form_rank(right_form)))
}

fn compare_combining_key(
    left_a: char,
    left_position: AccentPosition,
    left_b: char,
    right_a: char,
    right_position: AccentPosition,
    right_b: char,
) -> Ordering {
    left_a
        .cmp(&right_a)
        .then(accent_position_rank(left_position).cmp(&accent_position_rank(right_position)))
        .then(left_b.cmp(&right_b))
}

fn operator_form_rank(value: OperatorForm) -> u8 {
    match value {
        OperatorForm::Prefix => 0,
        OperatorForm::Infix => 1,
        OperatorForm::Postfix => 2,
    }
}

fn accent_position_rank(value: AccentPosition) -> u8 {
    match value {
        AccentPosition::Above => 0,
        AccentPosition::Below => 1,
        AccentPosition::Over => 2,
    }
}
