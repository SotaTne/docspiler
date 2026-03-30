# mathml_core

`mathml_core` は、このワークスペースにおける MathML の解析および解釈を担当する crate です。

この crate は、数式レンダリング入力のための MathML レイヤーです。ブラウザのような完全なMathML 実装ではありません。

## この crate の役割

このプロジェクト全体の最終目標は、`docx` と `pdf` の文書生成です。

そのため、この crate はブラウザ互換の MathML 実装ではなく、Rust で完結する
文書生成パイプライン向けの MathML レイヤーとして設計しています。

- 数式の見た目や構造に効く MathML Core の要素だけを抽出・正規化する
- ブラウザの DOM 実行環境や HTML/CSS プラットフォームを再現しない
- 後段のレイアウト処理や文書出力に渡すための安定した中間表現を作る

## スコープ

- ルートが `<math>...</math>` である MathML XML 文書のみをサポートします。
- 生の文字列分岐に頼らず、型付き Rust enum によって MathML 構造を保持します。
- MathML Core のうち、数式レンダリングに関係する機能に注力します。
- その対象は主に `docx` と `pdf` 生成に必要な機能です。
- `MathNode` へ変換する前段の安定した中間レイヤーを提供します。

## この crate に本当に必要なこと

このプロジェクトでは、MathML Core の完全準拠そのものは目標ではありません。
必要なのは、文書生成に必要な数式構造と数式表示の情報を正しく扱うことです。

そのため、この crate の目標は次のとおりです。

- `MathML Core のうち、文書生成に必要な数式レンダリング機能をサポートする`

そして、次は目標に含めません。

- `ブラウザ相当の MathML Core 完全準拠`

## サポートチェックリスト

### 現在サポートしているもの

- [x] ルートが `<math>...</math>` の MathML XML を解析する
- [x] この crate における妥当なトップレベル MathML 入力として、MathML ルート以外の入力を拒否する
- [x] `XmlDocument`、`XmlElement`、`XmlNode`、`XmlText` として XML 構造を保持する
- [x] 多くの MathML Core プレゼンテーション要素名を `MathMlElementName` として解析する
- [x] 未知の要素名を `MathMlElementName::Unknown` として保持する
- [x] 多くのレンダリング関連 MathML 属性名を `MathMlAttributeName` として解析する
- [x] 未知の属性名を `MathMlAttributeName::Unknown` として保持する
- [x] MathML XML 入力に必要な XML テキスト、CDATA、コメント、一般実体参照を解析する
- [x] 数学演算子および記号テキストの一部を `MathMlSymbol` としてネイティブ解釈する
- [x] レンダリング関連の付録データに対するネイティブ lookup helper を提供する
- [x] 付録 B.2 演算子辞書 lookup を完全にサポートする
- [x] 付録 B.3 結合文字等価表 lookup を完全にサポートする
- [x] 付録 B.4 Unicode ベース glyph assembly lookup を完全にサポートする
- [x] 付録 C.1 数学イタリック変換 lookup を完全にサポートする
- [x] サポート済みの解析・解釈挙動に対する統合テストを提供する
- [x] `assets/` にバンドルした W3C スナップショットから build 時に付録 lookup table を生成する
- [x] 文書レンダリングに関係する MathML Core プレゼンテーション要素の完全対応
  （意図的に対象外としているインタラクティブ要素を除く）
- [x] この文書生成パイプラインで使うレンダリング関連 MathML 属性の完全対応
- [x] レンダリング関連属性値の型付き解釈
- [x] `boolean` の解析
- [x] `unsigned-integer` の解析
- [x] `+U`、`-U`、`U` 形式の `scriptlevel` 値の解釈
- [x] `display="block|inline"` の正規化と妥当性検証
- [x] `mathvariant="normal"` の解釈
- [x] `lspace`、`rspace`、`minsize`、`maxsize`、`width`、`height`、`depth` などに対する `<length-percentage>` の解析
- [x] `linethickness` の解釈
- [x] 表組み数式向けの `rowspan` と `columnspan` の型付き解析

### この crate では意図的にサポートしないもの

- [ ] ブラウザ風のレイアウトおよび描画アルゴリズム
- [ ] `display: inline math` や `display: block math` のような CSS レイアウト挙動
- [ ] OpenType MATH テーブルに基づく shaping およびフォントメトリクス解決
- [ ] DOM、フォーカス、イベント処理、スクリプト挙動
- [ ] ハイパーリンク挙動やその他の HTML 的なインタラクティブ機能
- [ ] 一般的な HTML 文書解析
- [ ] トップレベルの `<math>...</math>` MathML ルート外にある HTML/MathML 混在文書の処理

これらは、このプロジェクトの主目的である `docx` / `pdf` 生成には直接必要ないため、
この crate では意図的に対象外としています。

## バンドルしている W3C 文書

- 仕様原本: <https://www.w3.org/TR/mathml-core/>
- ローカルコピーが参照しているスナップショット:
  <https://www.w3.org/TR/2025/CR-mathml-core-20250624/>
- `assets/` は build 時の生成と実装確認に使う完全なローカルスナップショット入力を置く場所です
- ローカルコピー:
  - [`assets/CR-mathml-core-20250624.html`](./assets/CR-mathml-core-20250624.html)

## ライセンスと帰属表示

- W3C の帰属表示および再配布に関する注意は [`NOTICE.md`](./NOTICE.md) を参照してください。
