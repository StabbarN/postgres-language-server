use pgt_query::protobuf::Boolean;

use crate::{
    TokenKind,
    emitter::{EventEmitter, GroupKind},
};

pub(super) fn emit_boolean(e: &mut EventEmitter, n: &Boolean) {
    e.group_start(GroupKind::Boolean);
    // todo: user needs to be able to configure the case of boolean literals
    let val_str = if n.boolval { "TRUE" } else { "FALSE" };
    e.token(TokenKind::IDENT(val_str.to_string()));
    e.group_end();
}
