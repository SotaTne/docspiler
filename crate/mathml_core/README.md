# mathml_core

`mathml_core` is the MathML parsing and interpretation crate for this workspace.

This crate is a MathML layer for mathematical rendering input. It is not a full browser-like MathML implementation.

## Scope

- Only MathML XML documents rooted at `<math>...</math>` are supported.
- The crate preserves MathML structure in typed Rust enums instead of relying on
  raw string dispatch.
- The crate focuses on MathML Core features that are relevant to mathematical
  rendering.
- The crate provides a stable staging layer before conversion into `MathNode`.

## Support Checklist

### Currently supported

- [x] Parse MathML XML rooted at `<math>...</math>`.
- [x] Reject non-MathML-rooted input as valid top-level MathML input for this crate.
- [x] Preserve XML structure as typed nodes: `XmlDocument`, `XmlElement`,
  `XmlNode`, and `XmlText`.
- [x] Parse many MathML Core presentation element names into
  `MathMlElementName`.
- [x] Preserve unknown element names as `MathMlElementName::Unknown`.
- [x] Parse many rendering-relevant MathML attribute names into
  `MathMlAttributeName`.
- [x] Preserve unknown attribute names as `MathMlAttributeName::Unknown`.
- [x] Parse XML text, CDATA, comments, and general entity references needed for
  MathML XML input.
- [x] Interpret a native subset of mathematical operator and symbol text into
  `MathMlSymbol`.
- [x] Provide native lookup helpers for rendering-relevant appendix data.
- [x] Full appendix B.2 operator dictionary lookups.
- [x] Full appendix B.3 combining character equivalence lookups.
- [x] Full appendix B.4 Unicode-based glyph assembly lookups.
- [x] Full appendix C.1 mathematical italic variant lookups.
- [x] Provide integration tests for supported parsing and interpretation
  behavior.
- [x] Generate appendix lookup tables at build time from the bundled W3C
  snapshot in `assets/`.

### Not yet supported

- [ ] Full MathML Core presentation element coverage.
- [ ] Full rendering-relevant MathML attribute coverage.
- [ ] Typed interpretation of rendering-relevant attribute values.
- [ ] `boolean` parsing.
- [ ] `unsigned-integer` parsing.
- [ ] `scriptlevel` values of the form `+U`, `-U`, and `U`.
- [ ] `display="block|inline"` normalization and validation.
- [ ] `mathvariant="normal"` interpretation.
- [ ] `<length-percentage>` parsing for attributes such as `lspace`, `rspace`,
  `minsize`, `maxsize`, `width`, `height`, and `depth`.
- [ ] `linethickness` interpretation.
- [ ] `rowspan` and `columnspan` typed parsing for tabular math.
### Intentionally unsupported in this crate

- [ ] Browser-style layout and painting algorithms.
- [ ] CSS layout behavior such as `display: inline math` and
  `display: block math`.
- [ ] OpenType MATH table shaping and font metric resolution.
- [ ] DOM, focus, event handling, and scripting behavior.
- [ ] Hyperlink behavior and other HTML-like interactive features.
- [ ] General HTML document parsing.
- [ ] Mixed HTML/MathML document handling outside a top-level
  `<math>...</math>` MathML root.

## Bundled W3C Documents

- Specification source: <https://www.w3.org/TR/mathml-core/>
- Snapshot referenced by the local copy:
  <https://www.w3.org/TR/2025/CR-mathml-core-20250624/>
- `assets/` stores the complete local snapshot inputs used by build-time
  generation and implementation checks.
- Local copies:
  - [`assets/CR-mathml-core-20250624.html`](./assets/CR-mathml-core-20250624.html)

## License and attribution

- See [`NOTICE.md`](./NOTICE.md) for W3C attribution and redistribution notice.
