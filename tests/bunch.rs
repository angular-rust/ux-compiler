use pest::Parser;
use std::fs;

use ux_compiler::{Rule, UxParser};

#[test]
fn parse_001_abc_com_html() {
    let input = fs::read_to_string("tests/data/abc.com.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_002_aljazeera_com_html() {
    let input = fs::read_to_string("tests/data/aljazeera.com.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_003_antlr_html() {
    let input = fs::read_to_string("tests/data/antlr.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_004_attvalues_html() {
    let input = fs::read_to_string("tests/data/attvalues.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_005_bbc_html() {
    let input = fs::read_to_string("tests/data/bbc.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_006_cnn1_html() {
    let input = fs::read_to_string("tests/data/cnn1.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

// #[test]
// fn parse_007_digg_html() {
//     let input = fs::read_to_string("tests/data/digg.html").expect("cannot read file");
//     assert!(UxParser::parse(Rule::file, &input).is_ok());
// }

#[test]
fn parse_008_freebsd_html() {
    let input = fs::read_to_string("tests/data/freebsd.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_009_github_html() {
    let input = fs::read_to_string("tests/data/github.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_010_gnu_html() {
    let input = fs::read_to_string("tests/data/gnu.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_011_google_html() {
    let input = fs::read_to_string("tests/data/google.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_012_html4_html() {
    let input = fs::read_to_string("tests/data/html4.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_013_metafilter_html() {
    let input = fs::read_to_string("tests/data/metafilter.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_014_nbc_com_html() {
    let input = fs::read_to_string("tests/data/nbc.com.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_015_reddit_html() {
    let input = fs::read_to_string("tests/data/reddit.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_016_reddit2_html() {
    let input = fs::read_to_string("tests/data/reddit2.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_017_script1_html() {
    let input = fs::read_to_string("tests/data/script1.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_018_style1_html() {
    let input = fs::read_to_string("tests/data/style1.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_019_uglylink_html() {
    let input = fs::read_to_string("tests/data/uglylink.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

#[test]
fn parse_020_wikipedia_html() {
    let input = fs::read_to_string("tests/data/wikipedia.html").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}

// #[test]
// fn parse_021_youtube_html() {
//     let input = fs::read_to_string("tests/data/youtube.html").expect("cannot read file");
//     assert!(UxParser::parse(Rule::file, &input).is_ok());
// }

