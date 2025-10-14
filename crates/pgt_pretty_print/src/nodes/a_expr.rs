use pgt_query::protobuf::{AExpr, AExprKind};

use crate::emitter::{EventEmitter, GroupKind};

pub(super) fn emit_a_expr(e: &mut EventEmitter, n: &AExpr) {
    e.group_start(GroupKind::AExpr);

    assert_eq!(n.kind(), AExprKind::AexprOp);

    if let Some(ref lexpr) = n.lexpr {
        super::emit_node(lexpr, e);
    }

    if !n.name.is_empty() {
        e.space();
        for name in &n.name {
            super::emit_node(name, e);
        }
        e.space();
    }

    if let Some(ref rexpr) = n.rexpr {
        super::emit_node(rexpr, e);
    }

    e.group_end();
}
