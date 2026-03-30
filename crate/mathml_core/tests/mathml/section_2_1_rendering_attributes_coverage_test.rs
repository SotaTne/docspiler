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
    assert!(root.attribute(&MathMlAttributeName::Display).is_some());
    assert!(root.attribute(&MathMlAttributeName::MathColor).is_some());
    assert!(
        root.attribute(&MathMlAttributeName::MathBackground)
            .is_some()
    );
    assert!(root.attribute(&MathMlAttributeName::MathSize).is_some());
    assert!(root.attribute(&MathMlAttributeName::Dir).is_some());

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

    assert!(
        mstyle
            .attribute(&MathMlAttributeName::DisplayStyle)
            .is_some()
    );
    assert!(
        mstyle
            .attribute(&MathMlAttributeName::ScriptLevel)
            .is_some()
    );
    assert!(mi.attribute(&MathMlAttributeName::MathVariant).is_some());

    assert!(mo.attribute(&MathMlAttributeName::Form).is_some());
    assert!(mo.attribute(&MathMlAttributeName::Fence).is_some());
    assert!(mo.attribute(&MathMlAttributeName::Separator).is_some());
    assert!(mo.attribute(&MathMlAttributeName::LSpace).is_some());
    assert!(mo.attribute(&MathMlAttributeName::RSpace).is_some());
    assert!(mo.attribute(&MathMlAttributeName::Stretchy).is_some());
    assert!(mo.attribute(&MathMlAttributeName::Symmetric).is_some());
    assert!(mo.attribute(&MathMlAttributeName::MaxSize).is_some());
    assert!(mo.attribute(&MathMlAttributeName::MinSize).is_some());
    assert!(mo.attribute(&MathMlAttributeName::LargeOp).is_some());
    assert!(mo.attribute(&MathMlAttributeName::MovableLimits).is_some());

    assert!(
        mfrac
            .attribute(&MathMlAttributeName::LineThickness)
            .is_some()
    );
    assert!(mfrac.attribute(&MathMlAttributeName::NumAlign).is_some());
    assert!(mfrac.attribute(&MathMlAttributeName::DenomAlign).is_some());
    assert!(mfrac.attribute(&MathMlAttributeName::Bevelled).is_some());

    assert!(mspace.attribute(&MathMlAttributeName::Width).is_some());
    assert!(mspace.attribute(&MathMlAttributeName::Height).is_some());
    assert!(mspace.attribute(&MathMlAttributeName::Depth).is_some());

    assert!(mover.attribute(&MathMlAttributeName::Accent).is_some());
    assert!(
        munder
            .attribute(&MathMlAttributeName::AccentUnder)
            .is_some()
    );

    assert!(mtable.attribute(&MathMlAttributeName::RowAlign).is_some());
    assert!(
        mtable
            .attribute(&MathMlAttributeName::ColumnAlign)
            .is_some()
    );
    assert!(
        mtable
            .attribute(&MathMlAttributeName::ColumnSpacing)
            .is_some()
    );
    assert!(mtable.attribute(&MathMlAttributeName::RowSpacing).is_some());
    assert!(
        mtable
            .attribute(&MathMlAttributeName::ColumnLines)
            .is_some()
    );
    assert!(mtable.attribute(&MathMlAttributeName::RowLines).is_some());
    assert!(mtable.attribute(&MathMlAttributeName::Frame).is_some());
    assert!(
        mtable
            .attribute(&MathMlAttributeName::FrameSpacing)
            .is_some()
    );
    assert!(mtable.attribute(&MathMlAttributeName::EqualRows).is_some());
    assert!(
        mtable
            .attribute(&MathMlAttributeName::EqualColumns)
            .is_some()
    );
    assert!(mtable.attribute(&MathMlAttributeName::Side).is_some());
    assert!(
        mtable
            .attribute(&MathMlAttributeName::MinLabelSpacing)
            .is_some()
    );

    assert!(mtd.attribute(&MathMlAttributeName::RowSpan).is_some());
    assert!(mtd.attribute(&MathMlAttributeName::ColumnSpan).is_some());

    assert!(
        annotation
            .attribute(&MathMlAttributeName::Encoding)
            .is_some()
    );
}
