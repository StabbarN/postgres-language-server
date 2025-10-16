use crate::{TokenKind, emitter::EventEmitter, nodes::node_list::emit_comma_separated_list};
use pgt_query::protobuf::WindowDef;

// WindowDef is not a NodeEnum type, so we don't use pub(super)
// It's a helper structure used within FuncCall and SelectStmt
pub fn emit_window_def(e: &mut EventEmitter, n: &WindowDef) {
    // WindowDef is a helper structure, so we don't use group_start/group_end
    // It's emitted within the parent's group (FuncCall or SelectStmt)

    // If refname is set, this is a reference to a named window
    if !n.refname.is_empty() {
        e.token(TokenKind::IDENT(n.refname.clone()));
        return;
    }

    e.token(TokenKind::L_PAREN);

    let mut needs_space = false;

    // PARTITION BY clause
    if !n.partition_clause.is_empty() {
        e.token(TokenKind::PARTITION_KW);
        e.space();
        e.token(TokenKind::BY_KW);
        e.space();
        emit_comma_separated_list(e, &n.partition_clause, |node, emitter| {
            super::emit_node(node, emitter)
        });
        needs_space = true;
    }

    // ORDER BY clause
    if !n.order_clause.is_empty() {
        if needs_space {
            e.space();
        }
        e.token(TokenKind::ORDER_KW);
        e.space();
        e.token(TokenKind::BY_KW);
        e.space();
        emit_comma_separated_list(e, &n.order_clause, |node, emitter| {
            super::emit_node(node, emitter)
        });
    }

    // Frame clause (ROWS/RANGE/GROUPS)
    // frame_options is a bitmap that encodes the frame clause
    // This is complex - implementing basic support
    // TODO: Full frame clause implementation with start_offset and end_offset
    // For now, we skip frame clause emission if frame_options != 0
    // The default frame options (1058 = RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW)
    // are implicit and don't need to be emitted

    e.token(TokenKind::R_PAREN);
}
