use crate::support::{child_elements, element_children, parse};
use mathml_core::mathml_xml::MathMlElementName;

// Section: 2.1 Elements and attributes
// Spec: https://www.w3.org/TR/mathml-core/#mathml-elements-and-attributes
//
// What this file proves:
// - All MathML Core presentation elements that matter for document rendering
//   are recognized as typed element names.
// - Intentionally unsupported interactive elements such as <a> and <maction>
//   are excluded from this coverage target.

#[test]
fn supports_full_rendering_relevant_mathml_element_coverage() {
    let document = parse(
        r#"
<math>
  <mi>x</mi>
  <mn>2</mn>
  <mo>+</mo>
  <mtext>text</mtext>
  <mspace width="1em" height="2ex" depth="0.5ex"/>
  <ms lquote="&quot;" rquote="&quot;">quoted</ms>
  <mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>
  <mfrac><mi>x</mi><mn>2</mn></mfrac>
  <msqrt><mi>x</mi></msqrt>
  <mroot><mi>x</mi><mn>3</mn></mroot>
  <mstyle displaystyle="true"><mi>x</mi></mstyle>
  <merror><mtext>err</mtext></merror>
  <mpadded width="1em"><mi>x</mi></mpadded>
  <mphantom><mi>x</mi></mphantom>
  <msub><mi>x</mi><mn>1</mn></msub>
  <msup><mi>x</mi><mn>2</mn></msup>
  <msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup>
  <munder><mo>∑</mo><mi>i</mi></munder>
  <mover><mo>→</mo><mi>x</mi></mover>
  <munderover><mo>∑</mo><mi>i</mi><mi>j</mi></munderover>
  <mmultiscripts>
    <mi>T</mi><mi>a</mi><mi>b</mi><mprescripts/><mi>c</mi><mi>d</mi>
  </mmultiscripts>
  <mtable>
    <mtr><mtd><mi>a</mi></mtd></mtr>
  </mtable>
  <semantics>
    <mi>x</mi>
    <annotation encoding="application/x-tex">x</annotation>
    <annotation-xml encoding="application/mathml-content+xml"><ci>x</ci></annotation-xml>
  </semantics>
</math>
"#,
    );

    let children = child_elements(&document);

    assert_eq!(children[0].name, MathMlElementName::Mi);
    assert_eq!(children[1].name, MathMlElementName::Mn);
    assert_eq!(children[2].name, MathMlElementName::Mo);
    assert_eq!(children[3].name, MathMlElementName::Mtext);
    assert_eq!(children[4].name, MathMlElementName::Mspace);
    assert_eq!(children[5].name, MathMlElementName::Ms);
    assert_eq!(children[6].name, MathMlElementName::Mrow);
    assert_eq!(children[7].name, MathMlElementName::Mfrac);
    assert_eq!(children[8].name, MathMlElementName::Msqrt);
    assert_eq!(children[9].name, MathMlElementName::Mroot);
    assert_eq!(children[10].name, MathMlElementName::Mstyle);
    assert_eq!(children[11].name, MathMlElementName::Merror);
    assert_eq!(children[12].name, MathMlElementName::Mpadded);
    assert_eq!(children[13].name, MathMlElementName::Mphantom);
    assert_eq!(children[14].name, MathMlElementName::Msub);
    assert_eq!(children[15].name, MathMlElementName::Msup);
    assert_eq!(children[16].name, MathMlElementName::Msubsup);
    assert_eq!(children[17].name, MathMlElementName::Munder);
    assert_eq!(children[18].name, MathMlElementName::Mover);
    assert_eq!(children[19].name, MathMlElementName::Munderover);
    assert_eq!(children[20].name, MathMlElementName::Mmultiscripts);
    assert_eq!(
        element_children(children[20])[3].name,
        MathMlElementName::Mprescripts
    );
    assert_eq!(children[21].name, MathMlElementName::Mtable);
    assert_eq!(
        element_children(children[21])[0].name,
        MathMlElementName::Mtr
    );
    assert_eq!(
        element_children(element_children(children[21])[0])[0].name,
        MathMlElementName::Mtd
    );
    assert_eq!(children[22].name, MathMlElementName::Semantics);
    assert_eq!(
        element_children(children[22])[1].name,
        MathMlElementName::Annotation
    );
    assert_eq!(
        element_children(children[22])[2].name,
        MathMlElementName::AnnotationXml
    );
}
