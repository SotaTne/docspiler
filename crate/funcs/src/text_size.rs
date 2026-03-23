use rustybuzz::{Face, UnicodeBuffer, shape};
use std::mem;

/// 計算結果をまとめた構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextMetrics {
    pub width: f32,
    pub height: f32,
    /// フォントのベースラインから上方向への距離
    pub ascender: f32,
    /// フォントのベースラインから下方向への距離（通常は負の値）
    pub descender: f32,
}

pub struct TextMeasurer {
    buffer: UnicodeBuffer,
}

impl TextMeasurer {
    pub fn new() -> Self {
        Self {
            buffer: UnicodeBuffer::new(),
        }
    }

    pub fn measure_text_metrics(
        &mut self,
        token_text: &str,
        face: &Face,
        font_size_pt: f32,
    ) -> TextMetrics {
        let units_per_em = face.units_per_em() as f32;
        let scale = font_size_pt / units_per_em;

        let mut buf = mem::replace(&mut self.buffer, UnicodeBuffer::new());
        buf.push_str(token_text);

        let glyph_buffer = shape(face, &[], buf);

        let glyph_width: i32 = glyph_buffer
            .glyph_positions()
            .iter()
            .map(|p| p.x_advance)
            .sum();

        self.buffer = glyph_buffer.clear();

        let width = (glyph_width as f32) * scale;

        let ascender = (face.ascender() as f32) * scale;
        let descender = (face.descender() as f32) * scale;
        let height = (face.height() as f32) * scale;

        TextMetrics {
            width,
            height,
            ascender,
            descender,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustybuzz::Face;

    const FONT_DATA: &[u8] = include_bytes!("../../../tests/fixtures/Ahem.ttf");

    #[test]
    fn test_measure_text_metrics() {
        let face = Face::from_slice(FONT_DATA, 0).expect("Failed to parse font fixture");

        let text = "Hello"; // 5文字
        let font_size_pt = 12.0;

        let mut measurer = TextMeasurer::new();
        let metrics = measurer.measure_text_metrics(text, &face, font_size_pt);

        // 幅の検証 (12pt * 5文字 = 60pt)
        assert_eq!(metrics.width, 60.0);

        // 高さの検証 (Ahemフォントの高さは1em、つまり12ptになるはず)
        assert_eq!(metrics.height, 12.0);

        // AscenderとDescenderの検証（Ahemの仕様に基づく）
        assert_eq!(metrics.ascender, 12.0 * 0.8); // 9.6
        assert_eq!(metrics.descender, 12.0 * -0.2); // -2.4
    }

    #[test]
    fn test_measure_text_metrics_changes_with_font_size_pt() {
        let face = Face::from_slice(FONT_DATA, 0).expect("Failed to parse font fixture");
        let text = "Hello";

        let mut measurer = TextMeasurer::new();
        let metrics_12pt = measurer.measure_text_metrics(text, &face, 12.0);
        let metrics_24pt = measurer.measure_text_metrics(text, &face, 24.0);

        assert_eq!(metrics_12pt.width, 60.0);
        assert_eq!(metrics_24pt.width, 120.0);

        assert_eq!(metrics_12pt.height, 12.0);
        assert_eq!(metrics_24pt.height, 24.0);

        assert_eq!(metrics_12pt.ascender, 12.0 * 0.8);
        assert_eq!(metrics_24pt.ascender, 24.0 * 0.8);

        assert_eq!(metrics_12pt.descender, 12.0 * -0.2);
        assert_eq!(metrics_24pt.descender, 24.0 * -0.2);
    }

    #[test]
    fn test_measure_text_metrics_multiple_calls_reuses_buffer() {
        let face = Face::from_slice(FONT_DATA, 0).expect("Failed to parse font fixture");
        let font_size_pt = 12.0;
        let mut measurer = TextMeasurer::new();

        let text1 = "Hello"; // 5文字
        let text2 = "World!"; // 6 length
        let text3 = "Rust"; // 4 length

        // 同じ文字数にして、必要なグリフバッファ容量を揃える。
        let metrics1 = measurer.measure_text_metrics(text1, &face, font_size_pt);
        let metrics2 = measurer.measure_text_metrics(text2, &face, font_size_pt);
        let metrics3 = measurer.measure_text_metrics(text3, &face, font_size_pt);

        assert_eq!(
            metrics1.width, 60.0,
            "Width calculation for text1 failed. Expected 60.0, got {}",
            metrics1.width
        );

        assert_eq!(
            metrics2.width, 72.0,
            "Width calculation for text2 failed. Expected 72.0, got {}",
            metrics2.width
        );

        assert_eq!(
            metrics3.width, 48.0,
            "Width calculation for text3 failed. Expected 48.0, got {}",
            metrics3.width
        );

        // 高さの検証 (Ahemフォントの高さは1em、つまり12ptになるはず)
        // すべてのテキストで同じ高さになるはず
        assert_eq!(metrics1.height, 12.0);
        assert_eq!(metrics2.height, 12.0);
        assert_eq!(metrics3.height, 12.0);
    }
}
