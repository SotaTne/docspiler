# mathml_core

`mathml_core` は、このワークスペースにおける MathML の解析および解釈を担当する crate です。

この crate は、数式レンダリング入力のための MathML レイヤーです。ブラウザのような完全なMathML 実装ではありません。

## スコープ

- ルートが `<math>...</math>` である MathML XML 文書のみをサポートします。
- 生の文字列分岐に頼らず、型付き Rust enum によって MathML 構造を保持します。
- MathML Core のうち、数式レンダリングに関係する機能に注力します。
- `MathNode` へ変換する前段の安定した中間レイヤーを提供します。

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

### まだサポートしていないもの

- [ ] MathML Core プレゼンテーション要素の完全対応
- [ ] レンダリング関連 MathML 属性の完全対応
- [ ] レンダリング関連属性値の型付き解釈
- [ ] `boolean` の解析
- [ ] `unsigned-integer` の解析
- [ ] `+U`、`-U`、`U` 形式の `scriptlevel` 値の解釈
- [ ] `display="block|inline"` の正規化と妥当性検証
- [ ] `mathvariant="normal"` の解釈
- [ ] `lspace`、`rspace`、`minsize`、`maxsize`、`width`、`height`、`depth` などに対する `<length-percentage>` の解析
- [ ] `linethickness` の解釈
- [ ] 表組み数式向けの `rowspan` と `columnspan` の型付き解析
### この crate では意図的にサポートしないもの

- [ ] ブラウザ風のレイアウトおよび描画アルゴリズム
- [ ] `display: inline math` や `display: block math` のような CSS レイアウト挙動
- [ ] OpenType MATH テーブルに基づく shaping およびフォントメトリクス解決
- [ ] DOM、フォーカス、イベント処理、スクリプト挙動
- [ ] ハイパーリンク挙動やその他の HTML 的なインタラクティブ機能
- [ ] 一般的な HTML 文書解析
- [ ] トップレベルの `<math>...</math>` MathML ルート外にある HTML/MathML 混在文書の処理

## バンドルしている W3C 文書

- 仕様原本: <https://www.w3.org/TR/mathml-core/>
- ローカルコピーが参照しているスナップショット:
  <https://www.w3.org/TR/2025/CR-mathml-core-20250624/>
- `assets/` は build 時の生成と実装確認に使う完全なローカルスナップショット入力を置く場所です
- ローカルコピー:
  - [`assets/CR-mathml-core-20250624.html`](./assets/CR-mathml-core-20250624.html)

## ライセンスと帰属表示

- W3C の帰属表示および再配布に関する注意は [`NOTICE.md`](./NOTICE.md) を参照してください。
