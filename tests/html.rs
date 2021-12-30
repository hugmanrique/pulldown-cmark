// Tests for HTML spec.

use pulldown_cmark::{html, BrokenLink, Options, Parser};

#[test]
fn html_test_1() {
    let original = r##"Little header

<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_2() {
    let original = r##"Little header

<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<p>Little header</p>
<script
type="text/js">
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_3() {
    let original = r##"Little header

<?
<div></div>
<p>Useless</p>
?>"##;
    let expected = r##"<p>Little header</p>
<?
<div></div>
<p>Useless</p>
?>"##;

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_4() {
    let original = r##"Little header

<!--
<div></div>
<p>Useless</p>
-->"##;
    let expected = r##"<p>Little header</p>
<!--
<div></div>
<p>Useless</p>
-->"##;

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_5() {
    let original = r##"Little header

<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;
    let expected = r##"<p>Little header</p>
<![CDATA[
<div></div>
<p>Useless</p>
]]>"##;

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_6() {
    let original = r##"Little header

<!X
Some things are here...
>"##;
    let expected = r##"<p>Little header</p>
<!X
Some things are here...
>"##;

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_7() {
    let original = r##"Little header
-----------

<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;
    let expected = r##"<h2>Little header</h2>
<script>
function some_func() {
console.log("teeeest");
}


function another_func() {
console.log("fooooo");
}
</script>"##;

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_8() {
    let original = "A | B\n---|---\nfoo | bar";
    let expected = r##"<table><thead><tr><th>A</th><th>B</th></tr></thead><tbody>
<tr><td>foo</td><td>bar</td></tr>
</tbody></table>
"##;

    let mut s = String::new();
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    html::push_html(&mut s, Parser::new_ext(&original, opts));
    assert_eq!(expected, s);
}

#[test]
fn html_test_9() {
    let original = "---";
    let expected = "<hr />\n";

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_10() {
    let original = "* * *";
    let expected = "<hr />\n";

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_11() {
    let original = "hi ~~no~~";
    let expected = "<p>hi ~~no~~</p>\n";

    let mut s = String::new();
    html::push_html(&mut s, Parser::new(&original));
    assert_eq!(expected, s);
}

#[test]
fn html_test_broken_callback() {
    let original = r##"[foo],
[bar],
[baz],

   [baz]: https://example.org
"##;

    let expected = r##"<p><a href="https://replaced.example.org" title="some title">foo</a>,
[bar],
<a href="https://example.org">baz</a>,</p>
"##;

    use pulldown_cmark::{html, Options, Parser};

    let mut s = String::new();

    let mut callback = |broken_link: BrokenLink| {
        if &*broken_link.reference == "foo" || &*broken_link.reference == "baz" {
            Some(("https://replaced.example.org".into(), "some title".into()))
        } else {
            None
        }
    };

    let p = Parser::new_with_broken_link_callback(&original, Options::empty(), Some(&mut callback));
    html::push_html(&mut s, p);

    assert_eq!(expected, s);
}

// Tests for mathematical formulae.
#[cfg(feature = "math")]
mod math {
    use pulldown_cmark::{html, Parser};

    #[test]
    fn inline_test_1() {
        let original = "$a^2 + b^2 = c^2$";
        let expected = r##"<p><span class="katex"><span class="katex-mathml"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></mrow><annotation encoding="application/x-tex">a^2 + b^2 = c^2</annotation></semantics></math></span><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:0.8974em;vertical-align:-0.0833em;"></span><span class="mord"><span class="mord mathnormal">a</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8141em;"><span style="top:-3.063em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight">2</span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.2222em;"></span><span class="mbin">+</span><span class="mspace" style="margin-right:0.2222em;"></span></span><span class="base"><span class="strut" style="height:0.8141em;"></span><span class="mord"><span class="mord mathnormal">b</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8141em;"><span style="top:-3.063em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight">2</span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.2778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2778em;"></span></span><span class="base"><span class="strut" style="height:0.8141em;"></span><span class="mord"><span class="mord mathnormal">c</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8141em;"><span style="top:-3.063em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight">2</span></span></span></span></span></span></span></span></span></span></span></p>
"##;

        let mut s = String::new();
        html::push_html(&mut s, Parser::new(&original));
        assert_eq!(expected, s);
    }

    #[test]
    fn inline_test_2() {
        let original = r"$\cos^2 x + \sin^2 x = 1$";
        let expected = r##"<p><span class="katex"><span class="katex-mathml"><math xmlns="http://www.w3.org/1998/Math/MathML"><semantics><mrow><msup><mrow><mi>cos</mi><mo>⁡</mo></mrow><mn>2</mn></msup><mi>x</mi><mo>+</mo><msup><mrow><mi>sin</mi><mo>⁡</mo></mrow><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></mrow><annotation encoding="application/x-tex">\cos^2 x + \sin^2 x = 1</annotation></semantics></math></span><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:0.8974em;vertical-align:-0.0833em;"></span><span class="mop"><span class="mop">cos</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8141em;"><span style="top:-3.063em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight">2</span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.1667em;"></span><span class="mord mathnormal">x</span><span class="mspace" style="margin-right:0.2222em;"></span><span class="mbin">+</span><span class="mspace" style="margin-right:0.2222em;"></span></span><span class="base"><span class="strut" style="height:0.8719em;"></span><span class="mop"><span class="mop">sin</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height:0.8719em;"><span style="top:-3.1208em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight">2</span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.1667em;"></span><span class="mord mathnormal">x</span><span class="mspace" style="margin-right:0.2778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2778em;"></span></span><span class="base"><span class="strut" style="height:0.6444em;"></span><span class="mord">1</span></span></span></span></p>
"##;

        let mut s = String::new();
        html::push_html(&mut s, Parser::new(&original));
        assert_eq!(expected, s);
    }
}
