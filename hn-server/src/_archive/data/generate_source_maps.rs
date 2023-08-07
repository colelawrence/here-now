// ! Experimental matcher for source map creation for declaration file

use std::collections::BTreeMap;

use rayon::prelude::*;

use crate::{data::source_lines, prelude::get_crate_path};

fn rust_query() -> String {
    // (attribute_item (attribute (identifier) arguments: (token_tree (string_literal))))
    let protocol = r#"(attribute_item (attribute
      (identifier) @_identifier (#match? @_identifier "^protocol_type+")
        arguments: (token_tree . (string_literal) @protocol)
    ))"#;
    // works on playground...
    // let protocol = r#"(attribute_item (meta_item
    //   (identifier) @identifier (#match? @identifier "^protocol_type+")
    //     arguments: (meta_arguments . (string_literal) @protocol)
    // ))"#;
    format!(
        "
(
  {protocol}
  (enum_item
    name: (type_identifier) @item_enum)
)
(
  {protocol}
  (enum_item
    name: (type_identifier) @item_enum
    body: (enum_variant_list (enum_variant
        name: (identifier) @item_enum_variant)))
)
(
  {protocol}
  (enum_item
    name: (type_identifier) @item_enum
    body: (enum_variant_list (enum_variant
        name: (identifier) @item_enum_variant
      body: (field_declaration_list (field_declaration
            name: (field_identifier) @item_enum_variant_field)))))
)
(
  {protocol}
  (struct_item
    name: (type_identifier) @item_struct)
)
(
  {protocol}
  (struct_item
    name: (type_identifier) @item_struct
    body: (field_declaration_list (field_declaration
            name: (field_identifier) @item_struct_field)))
)
(
  {protocol}
  (struct_item
    name: (type_identifier) @item_struct)
)
"
    )
}

fn typescript_query() -> String {
    format!(
        "
(export_statement
  declaration: (type_alias_declaration
      name: (type_identifier) @item))
(export_statement 
    declaration: (ambient_declaration (function_signature
        name: (identifier) @item)))
; type DECLARE_SERVICE =
(internal_module (identifier) @item_enum body: (statement_block
    (type_alias_declaration (type_identifier) @item_enum_variant)))
; DECLARE_SERVICE: + fields
(internal_module (identifier) @item_enum body: (statement_block
    (type_alias_declaration (type_identifier) @item_enum_variant (object_type (property_signature
        (property_identifier) @item_enum_variant
        (type_annotation (object_type
            (property_signature
                (property_identifier) @item_enum_variant_field)
        ))
    )))
))
; function DECLARE_SERVICE
(internal_module body: (statement_block
    (function_signature (identifier) @item_enum_variant)
))
; Everything else
(type_identifier) @type
  "
    )
}

#[test]
#[ignore]
fn generate_sourcemap() {
    use std::fmt::Write;
    let protocol_file_dir = get_crate_path().join("protocols/protocol/is");

    let rust_file = "../../../src/data.rs";
    let decl_file = "driver.v0.gen.d.ts";

    let mut sm = sourcemap::SourceMapBuilder::new(None);

    eprintln!("Rust");
    type SourceThing = smallvec::SmallVec<[ustr::Ustr; 8]>;
    type SourceLoc = (u32, (usize, usize), (usize, usize));

    // rs
    let origins: BTreeMap<SourceThing, SourceLoc> = {
        let source_idx = sm.add_source(rust_file);
        let rs_lang = tree_sitter_rust::language();
        let rust_bytes = std::fs::read(protocol_file_dir.join(rust_file)).unwrap();
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(rs_lang).unwrap();
        let tree = parser.parse(&rust_bytes, None).unwrap();
        let qstr = rust_query();
        let query = match tree_sitter::Query::new(rs_lang, &qstr) {
            Ok(query) => query,
            Err(err) => panic!(
                "Failed to load Rust query ({err:#?}): {qstr}\n\nTRYING TO MATCH:\n\n{}",
                tree.root_node().to_sexp()
            ),
        };

        let cap_names = query.capture_names();
        let line_index = source_lines::SourceLineNumberIndex::new(rust_bytes.as_slice());
        let mut cur = tree_sitter::QueryCursor::new();
        let mut matches = cur.matches(&query, tree.root_node(), rust_bytes.as_slice());
        let origins = matches
            .map(|mat| {
                mat.captures
                    .iter()
                    .map(|cap| (cap.index, cap.node.byte_range()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let origins = origins
            .into_par_iter()
            .filter_map(|caps| -> Option<(SourceThing, SourceLoc)> {
                use std::fmt::Write;
                let mut buf = SourceThing::new();
                let mut some_bytes = None;
                for (cap_index, byte_range) in caps {
                    let cap_name = &cap_names[cap_index as usize];
                    if cap_name.starts_with('_') {
                        continue;
                    }
                    // we can assume the last because specificity seems to always be in order
                    some_bytes = Some(byte_range.clone());
                    let text = String::from_utf8_lossy(&rust_bytes[byte_range]);
                    let text = text.trim_matches('"');
                    buf.push(ustr::ustr(&cap_name));
                    buf.push(ustr::ustr(&text));
                    // eprintln!("{text:?} @{cap_name} found at {from:?} - {to:?}")
                }
                let byte_range = some_bytes?;
                let from = line_index.get_ln_col(byte_range.start);
                let to = line_index.get_ln_col(byte_range.end);

                Some((buf, (source_idx, from, to)))
            })
            .collect::<BTreeMap<SourceThing, SourceLoc>>();

        for (k, v) in origins.iter() {
            eprintln!("{k:?} - {v:?}");
        }
        // eprintln!("{origins:#?}");
        origins
    };

    /// we will need to sort this before actually doing the insertion.
    let mut tokens = Vec::with_capacity(128);
    eprintln!("TypeScript");
    // ts
    {
        let mut matched = 0;
        let mut not_found = 0;
        let mut skipped = 0;
        let protocol = ustr::ustr("protocol");
        let fallback_protocol_names = [ustr::ustr("global")];
        let protocol_name = ustr::ustr("driver");
        let query_bases = {
            let mut base = SourceThing::new();
            base.push(protocol);
            base.push(protocol_name);
            let mut bases = smallvec::SmallVec::<[SourceThing; 3]>::new();
            bases.push(base);
            for name in fallback_protocol_names {
                let mut base = SourceThing::new();
                base.push(protocol);
                base.push(name);
                bases.push(base);
            }
            bases
        };

        let ts_lang = tree_sitter_typescript::language_typescript();
        let driver_bytes = std::fs::read(protocol_file_dir.join(decl_file)).unwrap();
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(ts_lang).unwrap();
        let tree = parser.parse(&driver_bytes, None).unwrap();
        let query = tree_sitter::Query::new(ts_lang, &typescript_query()).unwrap();

        let cap_names = query.capture_names();
        let line_index = source_lines::SourceLineNumberIndex::new(driver_bytes.as_slice());
        let mut cur = tree_sitter::QueryCursor::new();
        let mut matches = cur.matches(&query, tree.root_node(), driver_bytes.as_slice());
        'match_loop: for mat in matches {
            let mut debug = String::new();
            let mut query_tails = smallvec::SmallVec::<[SourceThing; 2]>::new();
            query_tails.push(SourceThing::new());
            let mut from = (0, 0);
            let mut to = (0, 0);
            for cap in mat.captures {
                let cap_name = &cap_names[cap.index as usize];
                let byte_range = cap.node.byte_range();
                from = line_index.get_ln_col(byte_range.start);
                to = line_index.get_ln_col(byte_range.end);
                let text = String::from_utf8_lossy(&driver_bytes[byte_range]);
                if cap_name == "type" && &text == "R" {
                    skipped += 1;
                    continue 'match_loop;
                }

                writeln!(
                    &mut debug,
                    "{text:?} @{cap_name} found at {from:?} - {to:?}"
                );
                let cap_name = ustr::ustr(&cap_name);
                let text = ustr::ustr(&text);
                // garbage...
                /// if it's type, then we need to check both enums and structs
                let cap_name_type = ustr::ustr("type");
                let cap_name_item = ustr::ustr("item");
                if cap_name == cap_name_type || cap_name == cap_name_item {
                    let mut alt2 = query_tails.clone();
                    query_tails.iter_mut().for_each(|tail| {
                        let cap_name = ustr::ustr("item_enum");
                        if tail.ends_with(&[cap_name, text]) {
                            return;
                        }
                        tail.push(cap_name);
                        tail.push(text);
                    });
                    alt2.iter_mut().for_each(|tail| {
                        let cap_name = ustr::ustr("item_struct");
                        if tail.ends_with(&[cap_name, text]) {
                            return;
                        }
                        tail.push(cap_name);
                        tail.push(text);
                    });
                    query_tails.extend(alt2);
                } else {
                    query_tails.iter_mut().for_each(|tail| {
                        if tail.ends_with(&[cap_name, text]) {
                            return;
                        }
                        tail.push(cap_name);
                        tail.push(text);
                    });
                }
            }

            for query_base in query_bases.iter() {
                for query_tail in query_tails.iter() {
                    let mut query = query_base.clone();
                    query.extend(query_tail.iter().copied());
                    if let Some((src, (rs_from_ln, rs_from_col), (rs_to_ln, rs_to_col))) =
                        origins.get(&query)
                    {
                        // eprintln!("{rs_from_ln} -> {rs_to_ln} 👌");
                        tokens.push((
                            from.0 as u32,
                            from.1 as u32,
                            *rs_from_ln as u32,
                            *rs_from_col as u32,
                            *src,
                        ));
                        tokens.push((
                            to.0 as u32,
                            to.1 as u32,
                            *rs_to_ln as u32,
                            *rs_to_col as u32,
                            *src,
                        ));
                        matched += 1;
                        continue 'match_loop;
                    }
                }
            }

            not_found += 1;

            eprintln!("\n{debug}⛔️ {query_tails:?} with bases: {query_bases:?}");
            // eprintln!("{mat:?}");
        }

        let total = not_found + matched;
        eprintln!(
            "{matched} / {total} = {}% (skipped = {skipped})",
            matched * 100 / total
        );
    }

    // // data.rs: 43 18 5
    // // driver.v0.gen.ts: 170 12 5
    // sm.add_raw(42, 17, 169, 11, Some(data), None);
    // sm.add_raw(42, 22, 169, 16, Some(data), None);

    // eprintln!("Done src-len={}", driver_bytes.len());
    // let mut sm = sourcemap::SourceMapBuilder::new(None);
    // let data = sm.add_source("../../src/data.rs");
    // // data.rs: 43 18 5
    // // driver.v0.gen.ts: 170 12 5
    // sm.add_raw(42, 17, 169, 11, Some(data), None);
    // sm.add_raw(42, 22, 169, 16, Some(data), None);

    // need to be sorted before inserting into map
    tokens.sort();
    for a in tokens {
        sm.add_raw(a.0, a.1, a.2, a.3, Some(a.4), None);
    }

    let map_file_name = format!("{}.map", decl_file);
    let mut map_file = std::fs::File::create(protocol_file_dir.join(map_file_name)).unwrap();
    // Check using https://evanw.github.io/source-map-visualization
    sm.into_sourcemap()
        .to_writer(&mut map_file)
        .expect("wrote to map");
}
