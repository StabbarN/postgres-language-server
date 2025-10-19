use std::fs;

#[test]
fn debug_sqljson_first_difference() {
    let path = "crates/pgt_pretty_print/tests/data/multi/sqljson_60.sql";
    let content = fs::read_to_string(path).unwrap();
    let split_result = pgt_statement_splitter::split(&content);
    for range in &split_result.ranges {
        let statement = &content[usize::from(range.start())..usize::from(range.end())];
        let trimmed = statement.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parsed = pgt_query::parse(trimmed).unwrap();
        let mut ast = parsed.into_root().unwrap();

        let mut emitter = pgt_pretty_print::emitter::EventEmitter::new();
        pgt_pretty_print::nodes::emit_node_enum(&ast, &mut emitter);

        let mut output = String::new();
        let mut renderer = pgt_pretty_print::renderer::Renderer::new(
            &mut output,
            pgt_pretty_print::renderer::RenderConfig {
                max_line_length: 60,
                indent_size: 2,
                indent_style: pgt_pretty_print::renderer::IndentStyle::Spaces,
            },
        );
        renderer.render(emitter.events).unwrap();

        let parsed_output = pgt_query::parse(&output).unwrap();
        let mut parsed_ast = parsed_output.into_root().unwrap();

        clear_location(&mut parsed_ast);
        clear_location(&mut ast);

        if ast != parsed_ast {
            println!("Original: {}", trimmed);
            println!("Formatted: {}", output);
            panic!("Mismatch detected");
        }
    }
}

fn clear_location(node: &mut pgt_query::NodeEnum) {
    unsafe {
        node.iter_mut().for_each(|n| match n {
            pgt_query::NodeMut::ColumnRef(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::ParamRef(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::AExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::JoinExpr(n) => {
                (*n).rtindex = 0;
            }
            pgt_query::NodeMut::TypeCast(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::CollateClause(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::FuncCall(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::JsonParseExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::JsonValueExpr(n) => {
                if let Some(format) = (*n).format.as_mut() {
                    format.location = 0;
                }
            }
            pgt_query::NodeMut::JsonScalarExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::JsonSerializeExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::JsonObjectConstructor(n) => {
                (*n).location = 0;
                if let Some(output) = (*n).output.as_mut() {
                    if let Some(returning) = output.returning.as_mut() {
                        if let Some(format) = returning.format.as_mut() {
                            format.location = 0;
                        }
                    }
                }
            }
            pgt_query::NodeMut::JsonArrayConstructor(n) => {
                (*n).location = 0;
                if let Some(output) = (*n).output.as_mut() {
                    if let Some(returning) = output.returning.as_mut() {
                        if let Some(format) = returning.format.as_mut() {
                            format.location = 0;
                        }
                    }
                }
            }
            pgt_query::NodeMut::JsonArrayQueryConstructor(n) => {
                (*n).location = 0;
                if let Some(format) = (*n).format.as_mut() {
                    format.location = 0;
                }
                if let Some(output) = (*n).output.as_mut() {
                    if let Some(returning) = output.returning.as_mut() {
                        if let Some(format) = returning.format.as_mut() {
                            format.location = 0;
                        }
                    }
                }
            }
            pgt_query::NodeMut::AArrayExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::ResTarget(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::SortBy(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::WindowDef(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::TypeName(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::PartitionSpec(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::PartitionElem(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::SqlvalueFunction(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::ColumnDef(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::DefElem(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::XmlSerialize(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::AConst(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::RangeVar(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::RoleSpec(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::RangeTableFunc(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::RangeTableFuncCol(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::RowExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::BoolExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::GroupingFunc(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::GroupingSet(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::CommonTableExpr(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::SubLink(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::NullTest(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::Constraint(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::CaseWhen(n) => {
                (*n).location = 0;
            }
            pgt_query::NodeMut::CaseExpr(n) => {
                (*n).location = 0;
            }
            _ => {}
        });
    }
}
