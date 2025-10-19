#[test]
fn inspect_json_array_absent_returning() {
    let sql = "SELECT JSON_ARRAY(ABSENT ON NULL RETURNING jsonb);";
    let parsed = pgt_query::parse(sql).unwrap();
    let ast = parsed.into_root().unwrap();
    println!("AST: {:#?}", ast);
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
    println!("{}", output);
    panic!("stop");
}
