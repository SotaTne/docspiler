use mathml_core::mathml_tables::{
    find_combining_character_entry, find_glyph_assembly_entry, find_italic_variant,
    find_non_combining_character_entry, find_operator_dictionary_entries,
    find_operator_dictionary_entry, full_combining_character_entries, full_glyph_assembly_entries,
    full_italic_mapping_entries, full_non_combining_character_entries,
    full_operator_dictionary_entries,
};
use mathml_core::mathml_xml::{
    AccentPosition, GlyphAssemblyDirection, OperatorForm, OperatorProperty, OperatorStretchAxis,
};

#[test]
fn supports_full_b2_operator_dictionary_table() {
    let entries = full_operator_dictionary_entries();
    assert_eq!(entries.len(), 1177);

    let sum = find_operator_dictionary_entry("∑", OperatorForm::Prefix)
        .expect("summation should exist in the full operator dictionary");
    assert_eq!(sum.stretch_axis, OperatorStretchAxis::Block);
    assert_eq!(sum.lspace_em, 0.16666667);
    assert_eq!(sum.rspace_em, 0.16666667);
    assert!(sum.properties.contains(&OperatorProperty::LargeOp));
    assert!(sum.properties.contains(&OperatorProperty::MovableLimits));
    assert!(sum.properties.contains(&OperatorProperty::Symmetric));

    let fence = find_operator_dictionary_entry("|", OperatorForm::Prefix)
        .expect("vertical bar fence operator should exist");
    assert_eq!(fence.stretch_axis, OperatorStretchAxis::Block);
    assert!(fence.properties.contains(&OperatorProperty::Fence));
}

#[test]
fn supports_lookup_for_every_b2_operator_dictionary_entry() {
    for entry in full_operator_dictionary_entries() {
        let found = find_operator_dictionary_entries(entry.content, entry.form);
        assert!(
            found.iter().any(|candidate| candidate == entry),
            "every B.2 operator dictionary entry should be retrievable"
        );
    }
}

#[test]
fn supports_full_b3_combining_character_equivalence_tables() {
    let combining = full_combining_character_entries();
    let non_combining = full_non_combining_character_entries();

    assert_eq!(combining.len(), 39);
    assert_eq!(non_combining.len(), 41);

    let long_right_arrow = find_combining_character_entry('⟶', AccentPosition::Above, '\u{20D7}')
        .expect("long rightwards arrow combining mapping should exist");
    assert_eq!(long_right_arrow.non_combining, '⟶');

    let combining_low_line =
        find_non_combining_character_entry('\u{0332}', AccentPosition::Below, '_')
            .expect("combining low line reverse mapping should exist");
    assert_eq!(combining_low_line.non_combining, '_');
}

#[test]
fn supports_lookup_for_every_b3_combining_character_equivalence_entry() {
    for entry in full_combining_character_entries() {
        let found =
            find_combining_character_entry(entry.non_combining, entry.position, entry.combining)
                .expect("every B.3 combining entry should be retrievable");
        assert_eq!(found, entry);
    }

    for entry in full_non_combining_character_entries() {
        let found = find_non_combining_character_entry(
            entry.combining,
            entry.position,
            entry.non_combining,
        )
        .expect("every B.3 non-combining entry should be retrievable");
        assert_eq!(found, entry);
    }
}

#[test]
fn supports_full_b4_unicode_based_glyph_assembly_table() {
    let entries = full_glyph_assembly_entries();
    assert_eq!(entries.len(), 41);

    let left_paren =
        find_glyph_assembly_entry('(').expect("left parenthesis glyph assembly should exist");
    assert_eq!(left_paren.direction, GlyphAssemblyDirection::Vertical);
    assert_eq!(left_paren.extender, '⎜');
    assert_eq!(left_paren.bottom_or_left, '⎝');
    assert_eq!(left_paren.top_or_right, Some('⎛'));

    let right_arrow =
        find_glyph_assembly_entry('→').expect("right arrow glyph assembly should exist");
    assert_eq!(right_arrow.direction, GlyphAssemblyDirection::Horizontal);
    assert_eq!(right_arrow.extender, '⎯');
    assert_eq!(right_arrow.bottom_or_left, '⎯');
    assert_eq!(right_arrow.top_or_right, Some('→'));
}

#[test]
fn supports_lookup_for_every_b4_unicode_based_glyph_assembly_entry() {
    for entry in full_glyph_assembly_entries() {
        let found = find_glyph_assembly_entry(entry.base)
            .expect("every B.4 glyph assembly entry should be retrievable");
        assert_eq!(found, entry);
    }
}

#[test]
fn supports_full_c1_italic_mapping_table() {
    let entries = full_italic_mapping_entries();
    assert_eq!(entries.len(), 112);

    assert_eq!(find_italic_variant('A'), Some('𝐴'));
    assert_eq!(find_italic_variant('h'), Some('ℎ'));
    assert_eq!(find_italic_variant('Δ'), Some('𝛥'));
    assert_eq!(find_italic_variant('α'), Some('𝛼'));
}

#[test]
fn supports_lookup_for_every_c1_italic_mapping_entry() {
    for entry in full_italic_mapping_entries() {
        let found = find_italic_variant(entry.original)
            .expect("every C.1 italic mapping entry should be retrievable");
        assert_eq!(found, entry.italic);
    }
}
