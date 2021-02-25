package main

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var header = `use pest::Parser;
use std::fs;

use ux_compiler::{Rule, UxParser};
`
var template = `
#[test]
fn %s() {
    let input = fs::read_to_string("%s").expect("cannot read file");
    assert!(UxParser::parse(Rule::file, &input).is_ok());
}
`

func main() {
	var items = []string{header}
	directory := "../tests/data"
	var idx = 0
	if err := filepath.Walk(directory, func(path string, f os.FileInfo, err error) error {
		if !f.IsDir() && filepath.Ext(path) == ".html" {
			path = strings.Replace(path, "../", "", 1)
			re, _ := regexp.Compile(`[A-Z.-]`)
			newfname := re.ReplaceAllStringFunc(f.Name(), func(s string) string {
				if s != "." && s != "-" {
					return "_" + strings.ToLower(s)
				}
				return "_"
			})
			if strings.HasPrefix(newfname, "_") {
				newfname = newfname[1:]
			}
			idx++
			procname := fmt.Sprintf("parse_%.3d_%s", idx, newfname)
			record := fmt.Sprintf(template, procname, path)
			items = append(items, record)
		}
		return nil
	}); err != nil {
		fmt.Println(err.Error())
	}
	fmt.Println(strings.Join(items, ""))
}
