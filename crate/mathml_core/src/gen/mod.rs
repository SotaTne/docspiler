use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct OperatorDictionaryEntry {
    pub content: String,
    pub stretch_axis: String,
    pub form: String,
    pub lspace_em: f32,
    pub rspace_em: f32,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombiningEntry {
    pub non_combining: char,
    pub position: &'static str,
    pub combining: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonCombiningEntry {
    pub combining: char,
    pub position: &'static str,
    pub non_combining: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphAssemblyEntry {
    pub base: char,
    pub direction: &'static str,
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

#[derive(Debug, Clone, PartialEq)]
pub struct AppendixTables {
    pub b2: Vec<OperatorDictionaryEntry>,
    pub b3_combining: Vec<CombiningEntry>,
    pub b3_non_combining: Vec<NonCombiningEntry>,
    pub b4: Vec<GlyphAssemblyEntry>,
    pub c1: Vec<ItalicMappingEntry>,
}

pub fn parse_appendix_tables(html: &str) -> AppendixTables {
    let mut tables = AppendixTables {
        b2: parse_b2(html),
        b3_combining: parse_b3_combining(html),
        b3_non_combining: parse_b3_non_combining(html),
        b4: parse_b4(html),
        c1: parse_c1(html),
    };

    tables.b2.sort_by(|a, b| {
        a.content
            .cmp(&b.content)
            .then(form_rank(&a.form).cmp(&form_rank(&b.form)))
    });
    tables.b3_combining.sort_by(|a, b| {
        cmp_combining(
            a.non_combining,
            a.position,
            a.combining,
            b.non_combining,
            b.position,
            b.combining,
        )
    });
    tables.b3_non_combining.sort_by(|a, b| {
        cmp_combining(
            a.combining,
            a.position,
            a.non_combining,
            b.combining,
            b.position,
            b.non_combining,
        )
    });
    tables.b4.sort_by(|a, b| a.base.cmp(&b.base));
    tables.c1.sort_by(|a, b| a.original.cmp(&b.original));

    tables
}

pub fn generate_appendix_tables_rs(tables: &AppendixTables) -> String {
    let mut out = String::new();
    out.push_str(
        "// This file is generated at build time from assets/CR-mathml-core-20250624.html.\n",
    );
    out.push_str("// Do not edit by hand.\n\n");
    out.push_str(
        "pub(crate) static FULL_OPERATOR_DICTIONARY_ENTRIES: &[OperatorDictionaryStringEntry] = &[\n",
    );
    for row in &tables.b2 {
        out.push_str("    OperatorDictionaryStringEntry {\n");
        out.push_str(&format!("        content: {},\n", rust_str(&row.content)));
        out.push_str(&format!(
            "        stretch_axis: {},\n",
            rust_stretch_axis(&row.stretch_axis)
        ));
        out.push_str(&format!("        form: {},\n", rust_form(&row.form)));
        out.push_str(&format!("        lspace_em: {:?}f32,\n", row.lspace_em));
        out.push_str(&format!("        rspace_em: {:?}f32,\n", row.rspace_em));
        out.push_str(&format!(
            "        properties: {},\n",
            rust_properties(&row.properties)
        ));
        out.push_str("    },\n");
    }
    out.push_str("];\n\n");

    out.push_str(
        "pub(crate) static FULL_COMBINING_CHARACTER_ENTRIES: &[CombiningCharacterEntry] = &[\n",
    );
    for row in &tables.b3_combining {
        out.push_str("    CombiningCharacterEntry {\n");
        out.push_str(&format!(
            "        non_combining: {},\n",
            rust_char(row.non_combining)
        ));
        out.push_str(&format!(
            "        position: {},\n",
            rust_position(row.position)
        ));
        out.push_str(&format!(
            "        combining: {},\n",
            rust_char(row.combining)
        ));
        out.push_str("    },\n");
    }
    out.push_str("];\n\n");

    out.push_str(
        "pub(crate) static FULL_NON_COMBINING_CHARACTER_ENTRIES: &[NonCombiningCharacterEntry] = &[\n",
    );
    for row in &tables.b3_non_combining {
        out.push_str("    NonCombiningCharacterEntry {\n");
        out.push_str(&format!(
            "        combining: {},\n",
            rust_char(row.combining)
        ));
        out.push_str(&format!(
            "        position: {},\n",
            rust_position(row.position)
        ));
        out.push_str(&format!(
            "        non_combining: {},\n",
            rust_char(row.non_combining)
        ));
        out.push_str("    },\n");
    }
    out.push_str("];\n\n");

    out.push_str("pub(crate) static FULL_GLYPH_ASSEMBLY_ENTRIES: &[GlyphAssemblyCharEntry] = &[\n");
    for row in &tables.b4 {
        out.push_str("    GlyphAssemblyCharEntry {\n");
        out.push_str(&format!("        base: {},\n", rust_char(row.base)));
        out.push_str(&format!(
            "        direction: {},\n",
            rust_direction(row.direction)
        ));
        out.push_str(&format!("        extender: {},\n", rust_char(row.extender)));
        out.push_str(&format!(
            "        bottom_or_left: {},\n",
            rust_char(row.bottom_or_left)
        ));
        out.push_str(&format!(
            "        middle: {},\n",
            rust_option_char(row.middle)
        ));
        out.push_str(&format!(
            "        top_or_right: {},\n",
            rust_option_char(row.top_or_right)
        ));
        out.push_str("    },\n");
    }
    out.push_str("];\n\n");

    out.push_str("pub(crate) static FULL_ITALIC_MAPPING_ENTRIES: &[ItalicMappingEntry] = &[\n");
    for row in &tables.c1 {
        out.push_str("    ItalicMappingEntry {\n");
        out.push_str(&format!("        original: {},\n", rust_char(row.original)));
        out.push_str(&format!("        italic: {},\n", rust_char(row.italic)));
        out.push_str("    },\n");
    }
    out.push_str("];\n");

    out
}

fn parse_b2(html: &str) -> Vec<OperatorDictionaryEntry> {
    let section = extract_between(
        html,
        "<div id=\"operator-dictionary-entries\">",
        "</figure></div>",
    );
    let table = extract_first_table(section);
    parse_html_table_rows(table)
        .into_iter()
        .map(|row| OperatorDictionaryEntry {
            content: parse_character_codepoint_cell(&row[0]).to_string(),
            stretch_axis: parse_keyword(&row[1]),
            form: parse_keyword(strip_code_wrappers(&row[2])),
            lspace_em: parse_numeric_or_em(strip_code_wrappers(&row[3])),
            rspace_em: parse_numeric_or_em(strip_code_wrappers(&row[4])),
            properties: parse_properties(strip_code_wrappers(&row[5])),
        })
        .collect()
}

fn parse_b3_combining(html: &str) -> Vec<CombiningEntry> {
    let section = extract_between(
        html,
        "<section class=\"informative\" id=\"combining-character-equivalences\">",
        "<section class=\"informative\" id=\"unicode-based-glyph-assemblies\">",
    );
    let tables = extract_tables(section);
    parse_html_table_rows(tables[0])
        .into_iter()
        .map(|row| CombiningEntry {
            non_combining: parse_codepoint_only_cell(&row[0]),
            position: parse_position(&row[2]),
            combining: parse_codepoint_only_cell(&row[3]),
        })
        .collect()
}

fn parse_b3_non_combining(html: &str) -> Vec<NonCombiningEntry> {
    let section = extract_between(
        html,
        "<section class=\"informative\" id=\"combining-character-equivalences\">",
        "<section class=\"informative\" id=\"unicode-based-glyph-assemblies\">",
    );
    let tables = extract_tables(section);
    parse_html_table_rows(tables[1])
        .into_iter()
        .map(|row| NonCombiningEntry {
            combining: parse_codepoint_only_cell(&row[0]),
            position: parse_position(&row[2]),
            non_combining: parse_codepoint_only_cell(&row[3]),
        })
        .collect()
}

fn parse_b4(html: &str) -> Vec<GlyphAssemblyEntry> {
    let section = extract_between(
        html,
        "<section class=\"informative\" id=\"unicode-based-glyph-assemblies\">",
        "id=\"italic-mappings\"",
    );
    let table = extract_first_table(section);
    parse_html_table_rows(table)
        .into_iter()
        .map(|row| GlyphAssemblyEntry {
            base: parse_character_codepoint_cell(&row[0]),
            direction: parse_direction(&row[1]),
            extender: parse_character_codepoint_cell(&row[2]),
            bottom_or_left: parse_character_codepoint_cell(&row[3]),
            middle: parse_optional_character_codepoint_cell(&row[4]),
            top_or_right: parse_optional_character_codepoint_cell(&row[5]),
        })
        .collect()
}

fn parse_c1(html: &str) -> Vec<ItalicMappingEntry> {
    let section = extract_between(html, "<section id=\"italic-mappings\">", "</section>");
    let table = extract_first_table(section);
    parse_html_table_rows(table)
        .into_iter()
        .map(|row| ItalicMappingEntry {
            original: parse_character_codepoint_cell(&row[0]),
            italic: parse_character_codepoint_cell(&row[1]),
        })
        .collect()
}

fn extract_between<'a>(source: &'a str, start_marker: &str, end_marker: &str) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("missing marker: {start_marker}"));
    let tail = &source[start..];
    let end = tail
        .find(end_marker)
        .unwrap_or_else(|| panic!("missing marker: {end_marker}"));
    &tail[..end]
}

fn extract_tables(fragment: &str) -> Vec<&str> {
    let mut tables = Vec::new();
    let mut offset = 0;
    while let Some(start_rel) = fragment[offset..].find("<table") {
        let start = offset + start_rel;
        let end = fragment[start..]
            .find("</table>")
            .map(|i| start + i + "</table>".len())
            .expect("unterminated table");
        tables.push(&fragment[start..end]);
        offset = end;
    }
    tables
}

fn extract_first_table(fragment: &str) -> &str {
    extract_tables(fragment)
        .into_iter()
        .next()
        .expect("missing table")
}

fn parse_html_table_rows(table_html: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut offset = 0;
    while let Some(start_rel) = table_html[offset..].find("<tr") {
        let start = offset + start_rel;
        let open_end = table_html[start..]
            .find('>')
            .map(|i| start + i + 1)
            .expect("unterminated tr");
        let end = table_html[open_end..]
            .find("</tr>")
            .map(|i| open_end + i)
            .expect("unterminated tr close");
        let row_html = &table_html[open_end..end];
        if row_html.contains("<td") {
            rows.push(parse_html_cells(row_html));
        }
        offset = end + "</tr>".len();
    }
    rows
}

fn parse_html_cells(row_html: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut offset = 0;
    while let Some(start_rel) = row_html[offset..].find("<td") {
        let start = offset + start_rel;
        let open_end = row_html[start..]
            .find('>')
            .map(|i| start + i + 1)
            .expect("unterminated td");
        let end = row_html[open_end..]
            .find("</td>")
            .map(|i| open_end + i)
            .expect("unterminated td close");
        cells.push(clean_html_text(&row_html[open_end..end]));
        offset = end + "</td>".len();
    }
    cells
}

fn clean_html_text(value: &str) -> String {
    decode_html_entities(&strip_html_tags(value))
        .replace("\\_", "_")
        .trim()
        .to_string()
}

fn strip_html_tags(value: &str) -> String {
    let mut output = String::new();
    let mut in_tag = false;
    for ch in value.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => output.push(ch),
            _ => {}
        }
    }
    output
}

fn decode_html_entities(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

fn strip_code_wrappers(value: &str) -> &str {
    value.trim_matches('`')
}

fn parse_character_codepoint_cell(value: &str) -> char {
    if let Some((prefix, _)) = value.rsplit_once(" U+") {
        let prefix = prefix.trim();
        if prefix.chars().count() == 1 {
            return prefix.chars().next().unwrap();
        }
    }
    let code = find_codepoint(value);
    char::from_u32(code).expect("valid unicode scalar")
}

fn parse_optional_character_codepoint_cell(value: &str) -> Option<char> {
    if value == "N/A" {
        None
    } else {
        Some(parse_character_codepoint_cell(value))
    }
}

fn parse_codepoint_only_cell(value: &str) -> char {
    char::from_u32(find_codepoint(value)).expect("valid unicode scalar")
}

fn find_codepoint(value: &str) -> u32 {
    let start = value.find("U+").expect("missing codepoint") + 2;
    let hex: String = value[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_hexdigit())
        .collect();
    u32::from_str_radix(&hex, 16).expect("valid hex codepoint")
}

fn parse_numeric_or_em(value: &str) -> f32 {
    let value = value.strip_suffix("em").unwrap_or(value);
    value.parse().expect("valid f32")
}

fn parse_properties(value: &str) -> Vec<String> {
    if value == "N/A" {
        return Vec::new();
    }
    value.split_whitespace().map(parse_keyword).collect()
}

fn parse_keyword(value: &str) -> String {
    value.trim().to_string()
}

fn parse_position(value: &str) -> &'static str {
    match value.trim() {
        "above" => "above",
        "below" => "below",
        "over" => "over",
        other => panic!("unknown position: {other}"),
    }
}

fn parse_direction(value: &str) -> &'static str {
    match value.trim() {
        "Horizontal" => "Horizontal",
        "Vertical" => "Vertical",
        other => panic!("unknown direction: {other}"),
    }
}

fn form_rank(value: &str) -> u8 {
    match value {
        "prefix" => 0,
        "infix" => 1,
        "postfix" => 2,
        _ => panic!("unknown form: {value}"),
    }
}

fn position_rank(value: &str) -> u8 {
    match value {
        "above" => 0,
        "below" => 1,
        "over" => 2,
        _ => panic!("unknown position: {value}"),
    }
}

fn cmp_combining(
    a_left: char,
    a_pos: &str,
    a_right: char,
    b_left: char,
    b_pos: &str,
    b_right: char,
) -> Ordering {
    a_left
        .cmp(&b_left)
        .then(position_rank(a_pos).cmp(&position_rank(b_pos)))
        .then(a_right.cmp(&b_right))
}

fn rust_str(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        let code = ch as u32;
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ if (0x20..=0x7E).contains(&code) => out.push(ch),
            _ => out.push_str(&format!("\\u{{{code:X}}}")),
        }
    }
    out.push('"');
    out
}

fn rust_char(value: char) -> String {
    let code = value as u32;
    match value {
        '\'' => "'\\\''".to_string(),
        '\\' => "'\\\\'".to_string(),
        _ if (0x20..=0x7E).contains(&code) => format!("'{value}'"),
        _ => format!("'\\u{{{code:X}}}'"),
    }
}

fn rust_option_char(value: Option<char>) -> String {
    value
        .map(rust_char)
        .map(|v| format!("Some({v})"))
        .unwrap_or_else(|| "None".to_string())
}

fn rust_stretch_axis(value: &str) -> &'static str {
    match value {
        "inline" => "OperatorStretchAxis::Inline",
        "block" => "OperatorStretchAxis::Block",
        _ => panic!("unknown stretch axis: {value}"),
    }
}

fn rust_form(value: &str) -> &'static str {
    match value {
        "prefix" => "OperatorForm::Prefix",
        "infix" => "OperatorForm::Infix",
        "postfix" => "OperatorForm::Postfix",
        _ => panic!("unknown form: {value}"),
    }
}

fn rust_position(value: &str) -> &'static str {
    match value {
        "above" => "AccentPosition::Above",
        "below" => "AccentPosition::Below",
        "over" => "AccentPosition::Over",
        _ => panic!("unknown position: {value}"),
    }
}

fn rust_direction(value: &str) -> &'static str {
    match value {
        "Horizontal" => "GlyphAssemblyDirection::Horizontal",
        "Vertical" => "GlyphAssemblyDirection::Vertical",
        _ => panic!("unknown direction: {value}"),
    }
}

fn rust_properties(values: &[String]) -> String {
    if values.is_empty() {
        return "&[]".to_string();
    }
    let props = values
        .iter()
        .map(|value| match value.as_str() {
            "fence" => "OperatorProperty::Fence",
            "separator" => "OperatorProperty::Separator",
            "stretchy" => "OperatorProperty::Stretchy",
            "symmetric" => "OperatorProperty::Symmetric",
            "largeop" => "OperatorProperty::LargeOp",
            "movablelimits" => "OperatorProperty::MovableLimits",
            _ => panic!("unknown property: {value}"),
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("&[{props}]")
}
