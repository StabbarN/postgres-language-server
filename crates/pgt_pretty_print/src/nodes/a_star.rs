use pgt_query::protobuf::AStar;

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

pub(super) fn emit_a_star(e: &mut EventEmitter, _n: &AStar) {
    e.token(TokenKind::IDENT("*".to_string()))
}
