use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};

use pandoc_ast::{Block, MetaValue, MutVisitor, Pandoc};

fn replace_includes_in_lines(lines: &mut dyn Iterator<Item = &str>, entry_dir: &Path) -> String {
    let mut output = String::new();

    lines.for_each(|line| {
        if line.starts_with("!include") {
            let last_space = line.rfind(' ').unwrap();

            let include = &line[..last_space];
            let mut start_line = 1;
            let mut end_line = usize::max_value() - 1;
            if let Some(attr_start) = include.find('`') {
                let attr_slice = &include[attr_start + 1..];
                if let Some(attr_end) = attr_slice.find('`') {
                    let attr_slice = &attr_slice[..attr_end];
                    attr_slice.split(',').for_each(|attr| {
                        let mut parts = attr.split('=');
                        let key = parts.next().unwrap().trim();
                        let value = parts.next().unwrap().trim();
                        match key {
                            "startLine" => {
                                start_line = value.parse::<usize>().unwrap();
                            }
                            "endLine" => {
                                end_line = value.parse::<usize>().unwrap();
                            }
                            _ => {
                                eprintln!("unknown attribute: {}", key);
                            }
                        }
                    });
                }
            }

            let path = line[last_space + 1..].trim();
            let path = entry_dir.join(path);
            let mut file = std::fs::File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            output.push_str(&replace_includes_in_lines(
                &mut contents
                    .lines()
                    .skip(start_line - 1)
                    .take(end_line - start_line + 1),
                entry_dir,
            ));
        } else {
            output.push_str(line);
            output.push('\n');
        }
    });

    output
}

fn replace_includes(input: &str, entry_dir: &Path) -> String {
    replace_includes_in_lines(&mut input.lines(), &entry_dir)
}

struct MyVisitor<'a> {
    entry_dir: &'a Path,
}

impl<'a> MutVisitor for MyVisitor<'a> {
    fn visit_block(&mut self, block: &mut Block) {
        if let Block::CodeBlock(ref attr, ref content) = block {
            // replace include in content
            *block = Block::CodeBlock(attr.clone(), replace_includes(content, &self.entry_dir))
        }
        self.walk_block(block);
    }
}

fn include_filter(mut pandoc: Pandoc) -> Pandoc {
    let mut entry_dir = PathBuf::from(".");
    if let Some(MetaValue::MetaString(entry)) = pandoc.meta.get("include-entry") {
        entry_dir = PathBuf::from(entry);
    }

    let mut visitor = MyVisitor {
        entry_dir: &entry_dir,
    };

    visitor.walk_pandoc(&mut pandoc);

    pandoc
}

fn main() {
    let mut json = String::new();
    stdin().read_to_string(&mut json).unwrap();
    let res = pandoc_ast::filter(json.clone(), include_filter);
    stdout().write(res.as_bytes()).unwrap();
}
