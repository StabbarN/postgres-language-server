use pgt_query::{NodeEnum, protobuf::DoStmt};

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

pub(super) fn emit_do_stmt(e: &mut EventEmitter, n: &DoStmt) {
    e.group_start(GroupKind::DoStmt);

    e.token(TokenKind::DO_KW);

    // First, emit LANGUAGE clause if present
    for arg in &n.args {
        if let Some(NodeEnum::DefElem(def_elem)) = &arg.node {
            if def_elem.defname == "language" {
                if let Some(lang_node) = &def_elem.arg {
                    if let Some(NodeEnum::String(s)) = &lang_node.node {
                        e.space();
                        e.token(TokenKind::IDENT("LANGUAGE".to_string()));
                        e.space();
                        e.token(TokenKind::IDENT(s.sval.clone()));
                    }
                }
            }
        }
    }

    // Then emit the code block
    for arg in &n.args {
        if let Some(NodeEnum::DefElem(def_elem)) = &arg.node {
            if def_elem.defname == "as" {
                // Emit the code as a dollar-quoted string
                if let Some(code_node) = &def_elem.arg {
                    if let Some(NodeEnum::String(s)) = &code_node.node {
                        e.space();
                        e.token(TokenKind::IDENT("$$".to_string()));
                        e.token(TokenKind::IDENT(s.sval.clone()));
                        e.token(TokenKind::IDENT("$$".to_string()));
                    }
                }
            }
        }
    }

    e.token(TokenKind::SEMICOLON);
    e.group_end();
}
