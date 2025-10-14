use pgt_query::protobuf::String;

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

pub(super) fn emit_string(e: &mut EventEmitter, n: &String) {
    e.group_start(GroupKind::String);
    e.token(TokenKind::IDENT(n.sval.clone()));
    e.group_end();
}

pub(super) fn emit_string_literal(e: &mut EventEmitter, n: &String) {
    e.group_start(GroupKind::String);
    e.token(TokenKind::IDENT(format!("'{}'", n.sval.clone())));
    e.group_end();
}

pub(super) fn emit_string_identifier(e: &mut EventEmitter, n: &String) {
    e.group_start(GroupKind::String);
    e.token(TokenKind::IDENT(format!("\"{}\"", n.sval.clone())));
    e.group_end();
}
