use pgt_query::protobuf::ObjectWithArgs;

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
    nodes::node_list::emit_comma_separated_list,
};

pub(super) fn emit_object_with_args(e: &mut EventEmitter, n: &ObjectWithArgs) {
    emit_object_with_args_impl(e, n, true);
}

/// Emit ObjectWithArgs without parentheses (for operators in operator classes)
pub(super) fn emit_object_name_only(e: &mut EventEmitter, n: &ObjectWithArgs) {
    emit_object_with_args_impl(e, n, false);
}

fn emit_object_with_args_impl(e: &mut EventEmitter, n: &ObjectWithArgs, with_parens: bool) {
    e.group_start(GroupKind::ObjectWithArgs);

    // Object name (qualified name)
    if !n.objname.is_empty() {
        super::node_list::emit_dot_separated_list(e, &n.objname);
    }

    if with_parens {
        // Function arguments (for DROP FUNCTION, etc.)
        if !n.objargs.is_empty() {
            e.token(TokenKind::L_PAREN);
            emit_comma_separated_list(e, &n.objargs, super::emit_node);
            e.token(TokenKind::R_PAREN);
        } else if !n.args_unspecified {
            // Empty parens if args are specified as empty
            e.token(TokenKind::L_PAREN);
            e.token(TokenKind::R_PAREN);
        }
    }

    e.group_end();
}
