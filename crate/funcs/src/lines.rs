use rustybuzz::{Face, shape};
use std::ops::Range;
use unicode_linebreak::{BreakOpportunity, linebreaks};

use crate::UnicodeBufferScratch;

fn is_break_whitespace_byte(text: &str, byte_index: usize) -> bool {
    matches!(text.as_bytes().get(byte_index), Some(b' ' | b'\t'))
}

pub fn break_into_lines(
    text: &str,
    face: &Face,
    font_size_pt: f32,
    max_width: f32,
    tracking_em: f32,
    buffer_scratch: &mut UnicodeBufferScratch,
) -> Vec<Range<usize>> {
    let units_per_em = face.units_per_em() as f32;
    let scale = font_size_pt / units_per_em;
    let tracking_pt = tracking_em * font_size_pt; // トラッキングをポイントに変換

    // 1. 文字列全体を一括シェーピングする
    let mut buf = buffer_scratch.steal_buffer();
    buf.push_str(text);
    let glyph_buffer = shape(face, &[], buf);

    let infos = glyph_buffer.glyph_infos();
    let positions = glyph_buffer.glyph_positions();

    // 2. UAX #14 の改行イテレータを準備
    let mut breaks = linebreaks(text);
    let mut next_break = breaks.next();

    let mut lines = Vec::new();
    let mut line_start_byte = 0;
    let mut current_width = 0.0;

    // 直近で通過した「安全な改行位置」を記憶する (バイトインデックス, グリフインデックス)
    let mut last_safe_break: Option<(usize, usize)> = None;

    let mut i = 0;
    while i < infos.len() {
        // infos[i].cluster は、このグリフが「元の文字列(text)の何バイト目か」を示す
        let cluster = infos[i].cluster as usize;

        // A. 現在のバイト位置(cluster)が、改行可能位置を通り過ぎたかチェックし、安全な位置を記憶する
        while let Some((break_byte, opp)) = next_break {
            if break_byte <= cluster {
                if opp == BreakOpportunity::Mandatory {
                    if break_byte > line_start_byte {
                        lines.push(line_start_byte..break_byte);
                        line_start_byte = break_byte;
                        current_width = 0.0;
                        last_safe_break = None;
                    }
                } else if opp == BreakOpportunity::Allowed {
                    last_safe_break = Some((break_byte, i));
                }
                next_break = breaks.next();
            } else {
                break; // まだ到達していない
            }
        }

        // B. 現在のグリフの幅を加算（スケール適用済み）
        let base_glyph_width = (positions[i].x_advance as f32) * scale;

        // 行の先頭（current_width == 0.0）でなければ、文字の「前」にトラッキングを足す
        let added_tracking = if current_width > 0.0 {
            tracking_pt
        } else {
            0.0
        };

        // 今回のステップで加算される実質的な幅
        let step_width = base_glyph_width + added_tracking;
        // C. 溢れ判定
        if current_width + step_width > max_width && current_width > 0.0 {
            if is_break_whitespace_byte(text, cluster) {
                i += 1;
                continue;
            }

            if let Some((safe_byte, safe_glyph_idx)) = last_safe_break {
                if safe_byte > line_start_byte {
                    // 【正常系】記憶しておいた安全な改行位置で切る
                    lines.push(line_start_byte..safe_byte);
                    line_start_byte = safe_byte;
                    // ループのインデックスを「切った位置」まで巻き戻し、次の行の計算を開始する
                    i = safe_glyph_idx;
                } else {
                    // 【異常系】単語が長すぎて1行に1度も改行可能位置がなかった（強制的にここで切る）
                    lines.push(line_start_byte..cluster);
                    line_start_byte = cluster;
                }
            } else {
                // 安全な改行位置が全く見つからなかった場合（強制改行）
                lines.push(line_start_byte..cluster);
                line_start_byte = cluster;
            }

            // 状態をリセットして次の行へ
            current_width = 0.0;
            last_safe_break = None;
            continue; // iをインクリメントせずに、巻き戻した位置（または強制改行位置）から再開
        }

        current_width += step_width;
        i += 1;
    }

    // D. ループを抜けたら、残った文字列を最後の1行として追加する
    if line_start_byte < text.len() {
        lines.push(line_start_byte..text.len());
    }

    // E. 所有権を奪ったバッファを初期化し、Scratchに戻す（ゼロアロケーションの維持）
    buffer_scratch.buffer = glyph_buffer.clear();

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustybuzz::Face;

    const FONT_DATA: &[u8] = include_bytes!("../../../tests/fixtures/Ahem.ttf");

    fn break_lines(text: &str, max_width: f32, tracking_em: f32) -> Vec<Range<usize>> {
        let face = Face::from_slice(FONT_DATA, 0).unwrap();
        let mut buffer_scratch = UnicodeBufferScratch::new();

        break_into_lines(
            text,
            &face,
            10.0,
            max_width,
            tracking_em,
            &mut buffer_scratch,
        )
    }

    fn assert_line_texts(text: &str, lines: &[Range<usize>], expected: &[&str]) {
        let actual: Vec<&str> = lines.iter().map(|range| &text[range.clone()]).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_breaking_forces_break_on_long_word() {
        let text = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // 長い単語（改行位置がない）をテスト
        let lines = break_lines(text, 100.0, 0.0);

        assert_eq!(lines.len(), 3);
        assert_line_texts(text, &lines, &["ABCDEFGHIJ", "KLMNOPQRST", "UVWXYZ"]);
    }

    #[test]
    fn test_line_breaking_applies_tracking_em_to_wrap_decision() {
        let text = "ABCDE";

        let without_tracking = break_lines(text, 54.0, 0.0);
        let with_tracking = break_lines(text, 54.0, 0.5);

        assert_line_texts(text, &without_tracking, &["ABCDE"]);
        assert_line_texts(text, &with_tracking, &["ABC", "DE"]);
    }

    #[test]
    fn test_line_breaking_prefers_allowed_break_before_forcing_split() {
        let text = "AB CD";
        let lines = break_lines(text, 35.0, 0.0);

        assert_line_texts(text, &lines, &["AB ", "CD"]);
    }

    #[test]
    fn test_line_breaking_honors_mandatory_newline_even_when_width_allows_more() {
        let text = "ABC\nDEF";
        let lines = break_lines(text, 1_000.0, 0.0);

        assert_line_texts(text, &lines, &["ABC\n", "DEF"]);
    }

    #[test]
    fn test_line_breaking_keeps_exact_fit_on_single_line() {
        let text = "ABCDE";
        let lines = break_lines(text, 50.0, 0.0);

        assert_line_texts(text, &lines, &["ABCDE"]);
    }

    #[test]
    fn test_line_breaking_returns_no_lines_for_empty_text() {
        let text = "";
        let lines = break_lines(text, 50.0, 0.0);

        assert!(lines.is_empty());
    }

    #[test]
    fn test_line_breaking_trims_trailing_spaces_at_wrap() {
        let text = "ABCD EFGH";
        let lines = break_lines(text, 45.0, 0.0);

        assert_line_texts(text, &lines, &["ABCD ", "EFGH"]);
    }

    #[test]
    fn test_line_breaking_skips_leading_spaces_on_next_line_after_wrap() {
        let text = "ABCD  EFGH";
        let lines = break_lines(text, 45.0, 0.0);

        assert_line_texts(text, &lines, &["ABCD  ", "EFGH"]);
    }

    #[test]
    fn test_line_breaking_treats_tab_as_breakable_whitespace() {
        let text = "ABCD\tEFGH";
        let lines = break_lines(text, 45.0, 0.0);

        assert_line_texts(text, &lines, &["ABCD\t", "EFGH"]);
    }

    #[test]
    fn test_line_breaking_applies_tracking_when_wrapping_at_space() {
        let text = "AB CD";

        let without_tracking = break_lines(text, 54.0, 0.0);
        let with_tracking = break_lines(text, 54.0, 0.5);

        assert_line_texts(text, &without_tracking, &["AB CD"]);
        assert_line_texts(text, &with_tracking, &["AB ", "CD"]);
    }

    #[test]
    fn test_line_breaking_does_not_emit_whitespace_only_line_when_spaces_overflow() {
        let text = "AA      B";
        let lines = break_lines(text, 40.0, 0.0);

        assert_line_texts(text, &lines, &["AA      ", "B"]);
    }

    #[test]
    fn test_line_breaking_keeps_exact_fit_and_moves_following_text_to_next_line() {
        let text = "  AA  B";
        let lines = break_lines(text, 60.0, 0.0);

        assert_line_texts(text, &lines, &["  AA  ", "B"]);
    }

    #[test]
    fn test_line_breaking_keeps_japanese_closing_quote_with_previous_text() {
        let text = "「こんにちは」世界";
        let lines = break_lines(text, 60.0, 0.0);

        assert_line_texts(text, &lines, &["「こんにち", "は」世界"]);
    }
}
