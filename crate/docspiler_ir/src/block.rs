use std::ops::Range;

type ExtraIndexData = usize;
type TokenIndex = usize; // Token配列を指すインデックス
type FontId = usize;

pub struct LayoutArena {
    pub blocks: Vec<Block>,
    pub extra_index_data: Vec<ExtraIndexData>,
    // 外部から渡された、ドキュメント全体のToken配列の参照（あるいは実体）を保持する
    // pub tokens: Vec<Token>,
}

pub enum BlockKind {
    Run,
    Line,
    TextBlock,
}

// pub struct TextStyle {
//     pub font_id: FontId,
//     pub font_size_pt: f32,
//     pub color: Color,
//     pub letter_spacing: f32,
//     pub emphasis: TextEmphasis,
//     pub decoration: TextDecoration,
//     pub baseline_shift: BaselineShift,
// }

pub enum BlockData {
    /// 分岐ノード: extra_index_data の範囲を指し、子ブロックのリストを表現する
    Branch(Range<ExtraIndexData>),

    /// 末端ノード: 外部の Token 配列の連続した範囲を指す
    LeafText {
        token_range: Range<TokenIndex>,
        font_id: usize,
        font_size_pt: f32,
    },
}

/// Measureフェーズで確定するのはサイズだけだ
#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub struct Block {
    pub kind: BlockKind,
    pub splitable: bool,
    pub size: Size,
    pub data: BlockData,
}
