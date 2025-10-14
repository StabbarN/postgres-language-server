use pgt_query::protobuf::ResTarget;

use crate::TokenKind;
use crate::emitter::{EventEmitter, GroupKind};

use super::emit_node;

pub(super) fn emit_res_target(e: &mut EventEmitter, n: &ResTarget) {
    e.group_start(GroupKind::ResTarget);

    if !n.name.is_empty() {
        e.token(TokenKind::IDENT(n.name.clone()));
        for i in &n.indirection {
            if !matches!(i.node, Some(pgt_query::protobuf::node::Node::AIndices(_))) {
                e.token(TokenKind::DOT);
            }
            emit_node(i, e);
        }
        e.space();
        e.token(TokenKind::IDENT("=".to_string()));
        e.space();
    }
    if let Some(ref val) = n.val {
        emit_node(val, e);
    }

    e.group_end();
}
