use crate::support::{child_elements, element_children, parse};
use mathml_core::mathml_xml::MathMlAttributeName;

// Section: 2.1 Elements and attributes
// Spec: https://www.w3.org/TR/mathml-core/#mathml-elements-and-attributes
//
// What this file proves:
// - Rendering-relevant MathML attributes used by this document-generation
//   pipeline are recognized as typed attribute names.
// - HTML-like interactive attributes remain intentionally out of scope.

#[test]
fn supports_full_rendering_relevant_mathml_attribute_coverage() {
    let document = parse(
        r#"
<math display="block" mathcolor="red" mathbackground="blue" mathsize="1.2em" dir="rtl">
  <mstyle displaystyle="true" scriptlevel="+1"><mi mathvariant="normal">x</mi></mstyle>
  <mo form="prefix" fence="true" separator="false" lspace="0.2em" rspace="150%" stretchy="true" symmetric="false" maxsize="300%" minsize="2em" largeop="true" movablelimits="false">|</mo>
  <mfrac linethickness="200%" numalign="left" denomalign="right" bevelled="true"><mi>x</mi><mn>2</mn></mfrac>
  <mspace width="1em" height="2ex" depth="0.5ex" />
  <mover accent="true"><mi>x</mi><mo>^</mo></mover>
  <munder accentunder="false"><mi>x</mi><mo>_</mo></munder>
  <mtable rowalign="axis" columnalign="left right" columnspacing="1em" rowspacing="2ex" columnlines="solid none" rowlines="solid" frame="solid" framespacing="0.4em 0.5ex" equalrows="true" equalcolumns="false" side="left" minlabelspacing="0.8em">
    <mtr>
      <mtd rowspan="2" columnspan="3"><mi>a</mi></mtd>
    </mtr>
  </mtable>
  <annotation encoding="application/x-tex">x</annotation>
</math>
"#,
    );

    let root = &document.root;
    assert_eq!(root.attributes[0].name, MathMlAttributeName::Display);
    assert_eq!(root.attributes[1].name, MathMlAttributeName::MathColor);
    assert_eq!(root.attributes[2].name, MathMlAttributeName::MathBackground);
    assert_eq!(root.attributes[3].name, MathMlAttributeName::MathSize);
    assert_eq!(root.attributes[4].name, MathMlAttributeName::Dir);

    let children = child_elements(&document);
    let mstyle = children[0];
    let mi = element_children(mstyle)[0];
    let mo = children[1];
    let mfrac = children[2];
    let mspace = children[3];
    let mover = children[4];
    let munder = children[5];
    let mtable = children[6];
    let annotation = children[7];
    let mtr = element_children(mtable)[0];
    let mtd = element_children(mtr)[0];

    assert_eq!(mstyle.attributes[0].name, MathMlAttributeName::DisplayStyle);
    assert_eq!(mstyle.attributes[1].name, MathMlAttributeName::ScriptLevel);
    assert_eq!(mi.attributes[0].name, MathMlAttributeName::MathVariant);

    assert_eq!(mo.attributes[0].name, MathMlAttributeName::Form);
    assert_eq!(mo.attributes[1].name, MathMlAttributeName::Fence);
    assert_eq!(mo.attributes[2].name, MathMlAttributeName::Separator);
    assert_eq!(mo.attributes[3].name, MathMlAttributeName::LSpace);
    assert_eq!(mo.attributes[4].name, MathMlAttributeName::RSpace);
    assert_eq!(mo.attributes[5].name, MathMlAttributeName::Stretchy);
    assert_eq!(mo.attributes[6].name, MathMlAttributeName::Symmetric);
    assert_eq!(mo.attributes[7].name, MathMlAttributeName::MaxSize);
    assert_eq!(mo.attributes[8].name, MathMlAttributeName::MinSize);
    assert_eq!(mo.attributes[9].name, MathMlAttributeName::LargeOp);
    assert_eq!(mo.attributes[10].name, MathMlAttributeName::MovableLimits);

    assert_eq!(mfrac.attributes[0].name, MathMlAttributeName::LineThickness);
    assert_eq!(mfrac.attributes[1].name, MathMlAttributeName::NumAlign);
    assert_eq!(mfrac.attributes[2].name, MathMlAttributeName::DenomAlign);
    assert_eq!(mfrac.attributes[3].name, MathMlAttributeName::Bevelled);

    assert_eq!(mspace.attributes[0].name, MathMlAttributeName::Width);
    assert_eq!(mspace.attributes[1].name, MathMlAttributeName::Height);
    assert_eq!(mspace.attributes[2].name, MathMlAttributeName::Depth);

    assert_eq!(mover.attributes[0].name, MathMlAttributeName::Accent);
    assert_eq!(munder.attributes[0].name, MathMlAttributeName::AccentUnder);

    assert_eq!(mtable.attributes[0].name, MathMlAttributeName::RowAlign);
    assert_eq!(mtable.attributes[1].name, MathMlAttributeName::ColumnAlign);
    assert_eq!(
        mtable.attributes[2].name,
        MathMlAttributeName::ColumnSpacing
    );
    assert_eq!(mtable.attributes[3].name, MathMlAttributeName::RowSpacing);
    assert_eq!(mtable.attributes[4].name, MathMlAttributeName::ColumnLines);
    assert_eq!(mtable.attributes[5].name, MathMlAttributeName::RowLines);
    assert_eq!(mtable.attributes[6].name, MathMlAttributeName::Frame);
    assert_eq!(mtable.attributes[7].name, MathMlAttributeName::FrameSpacing);
    assert_eq!(mtable.attributes[8].name, MathMlAttributeName::EqualRows);
    assert_eq!(mtable.attributes[9].name, MathMlAttributeName::EqualColumns);
    assert_eq!(mtable.attributes[10].name, MathMlAttributeName::Side);
    assert_eq!(
        mtable.attributes[11].name,
        MathMlAttributeName::MinLabelSpacing
    );

    assert_eq!(mtd.attributes[0].name, MathMlAttributeName::RowSpan);
    assert_eq!(mtd.attributes[1].name, MathMlAttributeName::ColumnSpan);

    assert_eq!(annotation.attributes[0].name, MathMlAttributeName::Encoding);
}
