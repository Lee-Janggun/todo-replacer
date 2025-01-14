use ra_ap_syntax::ast::HasName;
use ra_ap_syntax::{ast, AstNode, SourceFile, SyntaxNode};
use std::collections::HashMap;
use std::{fs, io};

fn main() -> io::Result<()> {
    // Input arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: todo-replacer <rust_file> <csv_file>");
        std::process::exit(1);
    }

    let rust_file_path = &args[1];
    let csv_file_path = &args[2];

    // Read input files
    let rust_code = fs::read_to_string(rust_file_path)?;
    let csv_content = fs::read_to_string(csv_file_path)?;

    // Parse the CSV to get function replacements
    let replacements = parse_csv(&csv_content);

    // Parse the Rust file
    let parsed_file = SourceFile::parse(&rust_code, ra_ap_syntax::Edition::Edition2021);
    let tree = parsed_file.tree();
    let syntax_tree = tree.syntax();
    let updated_code = replace_function_bodies(syntax_tree, &replacements);

    // Output the modified Rust code
    fs::write(rust_file_path, updated_code)
}

/// Parse the CSV file to extract function replacements
fn parse_csv(csv_content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in csv_content.lines() {
        if let Some((name, body)) = line.split_once(',') {
            map.insert(
                name.to_string(),
                if body.is_empty() { "{ todo!() }" } else { body }.to_string(),
            );
        }
    }
    map
}

/// Replace function bodies in the parsed Rust syntax tree
fn replace_function_bodies(
    syntax_tree: &SyntaxNode,
    replacements: &HashMap<String, String>,
) -> String {
    let mut new_code = syntax_tree.text().to_string();

    // Sort the replacements according to their start position in code. This ensures replace rang
    // is done from back-to-front, which is necessary to avoid invalidating the ranges of subsequent
    // replacements.
    let mut replace_sorted = Vec::new();

    for fn_def in syntax_tree.descendants().filter_map(ast::Fn::cast) {
        let fn_name = fn_def.name().unwrap();
        if let Some(new_body) = replacements.get(&fn_name.text().to_string()) {
            let old_body_range = fn_def.body().unwrap().syntax().text_range();
            replace_sorted.push((
                usize::from(old_body_range.start()),
                usize::from(old_body_range.end()),
                new_body,
            ));
        }
    }

    replace_sorted.sort_by(|a, b| b.0.cmp(&a.0));
    replace_sorted
        .into_iter()
        .for_each(|(begin, end, new_body)| {
            new_code.replace_range(begin..end, new_body);
        });

    new_code
}
