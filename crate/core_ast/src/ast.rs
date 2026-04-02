// core_ast/src/lib.rs

pub type AstId = usize;

/// 有限のセマンティック・ロール（あなたの決定によりカスタム文字列を排除）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockRole {
    Normal,
    CodeBlock,
    Warning,
    Info,
    // 必要ならここに足せ
}

/// 自動採番と参照のためのデータ
#[derive(Debug, Clone)]
pub struct LabelInfo {
    /// 採番のグループ名（例: "figure", "table", "map", "listing"）
    /// 同じグループ名を持つブロックが、出現順に 1, 2, 3... とカウントアップされる
    pub counter_name: String,
    /// 相互参照のためのユニークID（例: "fig:architecture"）
    pub id: String,
}

/// 物理レイアウトのための制約（x, y, 確定サイズは絶対に入れない）
#[derive(Debug, Clone, Default)]
pub struct Constraints {
    pub width_pt: Option<f32>,
    pub width_percent: Option<f32>,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub text_indent: f32,
    pub preserve_whitespace: bool, // コードブロック等のため
}

/// インライン要素
#[derive(Debug, Clone)]
pub enum SemanticRun {
    Text {
        text: String,
        style_id: usize,
    },
    /// 他のラベルへの参照（例: 「図1」として解決されるべきアンカー）
    Reference {
        target_id: String,
    },
}

/// ブロックの種類（Markdownの基本要素を網羅）
#[derive(Debug, Clone)]
pub enum SemanticBlockKind {
    Paragraph {
        runs: Vec<SemanticRun>,
    },
    Heading {
        level: u8,
        runs: Vec<SemanticRun>,
    },
    Image {
        src: String,
    },

    // --- コンテナ系 ---
    Row {
        children: Vec<AstId>,
    },
    Column {
        children: Vec<AstId>,
    },

    // --- Markdown拡張系 ---
    /// リスト本体（番号付きか否かを持つ）
    List {
        is_ordered: bool,
        children: Vec<AstId>,
    },
    /// リストの1項目（中にParagraphや、さらにネストしたListを持つ）
    ListItem {
        children: Vec<AstId>,
    },
    /// 引用ブロック（左側に線が引かれるなどの制約を持つコンテナ）
    Blockquote {
        children: Vec<AstId>,
    },
    /// 水平線（Thematic Break）
    ThematicBreak,

    // --- 表 ---
    Table {
        rows: Vec<AstId>,
    }, // TableRowのID配列
    TableRow {
        cells: Vec<AstId>,
    }, // TableCellのID配列
    TableCell {
        children: Vec<AstId>,
    }, // 中にParagraph等を持つ
}

/// ASTの1ノード
#[derive(Debug, Clone)]
pub struct SemanticBlock {
    pub id: AstId,
    pub role: BlockRole,          // あなたが決定した単一の有限タグ
    pub label: Option<LabelInfo>, // あなたが提案した自動採番・参照情報
    pub constraints: Constraints,
    pub kind: SemanticBlockKind,
}

/// アリーナ本体
#[derive(Debug, Clone)]
pub struct SemanticAst {
    pub blocks: Vec<SemanticBlock>,
    pub root_id: AstId,
}
