// https://docs.rs/sxd-xpath/0.4.2/sxd_xpath/
extern crate sxd_document;
extern crate sxd_xpath;

use sxd_document::parser;
use sxd_xpath::evaluate_xpath;

fn main() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
    <result>
        <message>OK</message>
        <description>
            <![CDATA[<p>
                <div class="foo">foo</div>
            </p>]]></description>
        <comment>&lt;p>bar&lt;/p></comment>
    </result>
    "#;

    let package = parser::parse(xml).expect("failed to parse XML");
    let document = package.as_document();

    let message = evaluate_xpath(&document, "//message").expect("Not found 'message'");
    assert_eq!("OK", message.string());

    let description = evaluate_xpath(&document, "//description").expect("Not found 'description'");
    println!("{}", description.string());

    let comment = evaluate_xpath(&document, "//comment").expect("Not found 'comment'");
    assert_eq!("<p>bar</p>", comment.string());
}
