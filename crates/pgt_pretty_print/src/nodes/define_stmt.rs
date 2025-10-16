use pgt_query::protobuf::{DefineStmt, Node, ObjectType};

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
    nodes::node_list::{emit_comma_separated_list, emit_dot_separated_list},
};

/// Emit collation definition (FROM clause)
fn emit_collation_definition(e: &mut EventEmitter, definition: &[Node]) {
    for def_node in definition {
        if let Some(pgt_query::NodeEnum::DefElem(def_elem)) = &def_node.node {
            if def_elem.defname == "from" {
                e.space();
                e.token(TokenKind::FROM_KW);
                e.space();
                // The arg is a List containing String nodes with the collation name
                if let Some(ref arg) = def_elem.arg {
                    if let Some(pgt_query::NodeEnum::List(list)) = &arg.node {
                        // Emit the strings in the list as dot-separated qualified name with quotes
                        for (i, item) in list.items.iter().enumerate() {
                            if i > 0 {
                                e.token(TokenKind::DOT);
                            }
                            if let Some(pgt_query::NodeEnum::String(s)) = &item.node {
                                super::emit_string_identifier(e, s);
                            } else {
                                super::emit_node(item, e);
                            }
                        }
                    } else {
                        super::emit_node(arg, e);
                    }
                }
            } else {
                // Other options use parenthesized syntax
                e.space();
                e.token(TokenKind::L_PAREN);
                super::emit_node(def_node, e);
                e.token(TokenKind::R_PAREN);
            }
        }
    }
}

pub(super) fn emit_define_stmt(e: &mut EventEmitter, n: &DefineStmt) {
    e.group_start(GroupKind::DefineStmt);

    e.token(TokenKind::CREATE_KW);

    if n.replace {
        e.space();
        e.token(TokenKind::OR_KW);
        e.space();
        e.token(TokenKind::REPLACE_KW);
    }

    e.space();

    let kind = ObjectType::try_from(n.kind).unwrap_or(ObjectType::Undefined);
    match kind {
        ObjectType::ObjectAggregate => e.token(TokenKind::AGGREGATE_KW),
        ObjectType::ObjectOperator => e.token(TokenKind::OPERATOR_KW),
        ObjectType::ObjectType => e.token(TokenKind::TYPE_KW),
        ObjectType::ObjectCollation => e.token(TokenKind::COLLATION_KW),
        ObjectType::ObjectTsdictionary => {
            e.token(TokenKind::IDENT("TEXT".to_string()));
            e.space();
            e.token(TokenKind::IDENT("SEARCH".to_string()));
            e.space();
            e.token(TokenKind::IDENT("DICTIONARY".to_string()));
        }
        ObjectType::ObjectTsconfiguration => {
            e.token(TokenKind::IDENT("TEXT".to_string()));
            e.space();
            e.token(TokenKind::IDENT("SEARCH".to_string()));
            e.space();
            e.token(TokenKind::IDENT("CONFIGURATION".to_string()));
        }
        ObjectType::ObjectTsparser => {
            e.token(TokenKind::IDENT("TEXT".to_string()));
            e.space();
            e.token(TokenKind::IDENT("SEARCH".to_string()));
            e.space();
            e.token(TokenKind::IDENT("PARSER".to_string()));
        }
        ObjectType::ObjectTstemplate => {
            e.token(TokenKind::IDENT("TEXT".to_string()));
            e.space();
            e.token(TokenKind::IDENT("SEARCH".to_string()));
            e.space();
            e.token(TokenKind::IDENT("TEMPLATE".to_string()));
        }
        _ => e.token(TokenKind::IDENT(format!("{:?}", kind))),
    }

    if n.if_not_exists {
        e.space();
        e.token(TokenKind::IF_KW);
        e.space();
        e.token(TokenKind::NOT_KW);
        e.space();
        e.token(TokenKind::EXISTS_KW);
    }

    if !n.defnames.is_empty() {
        e.space();
        emit_dot_separated_list(e, &n.defnames);
    }

    // TODO: Args (for operators/functions) - need parentheses
    if !n.args.is_empty() {
        e.space();
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.args, super::emit_node);
        e.token(TokenKind::R_PAREN);
    }

    // Definition options (WITH clause or parenthesized list)
    // Special case for COLLATION with FROM clause
    if kind == ObjectType::ObjectCollation && !n.definition.is_empty() {
        // For collations, emit FROM clause specially
        emit_collation_definition(e, &n.definition);
    } else if !n.definition.is_empty() {
        e.space();
        e.token(TokenKind::L_PAREN);
        emit_comma_separated_list(e, &n.definition, super::emit_node);
        e.token(TokenKind::R_PAREN);
    }

    e.token(TokenKind::SEMICOLON);
    e.group_end();
}
