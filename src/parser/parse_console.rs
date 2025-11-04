use swc_ecma_ast::{Expr, ExprOrSpread, Lit, MemberExpr};

pub fn extract_console(member: &MemberExpr, args: &Vec<ExprOrSpread>) -> Option<String> {
    if member
        .prop
        .as_ident()
        .map(|i| i.sym == *"log")
        .unwrap_or(false)
    {
        if let Some(ExprOrSpread { expr, .. }) = args.get(0) {
            if let Expr::Lit(Lit::Str(s)) = &**expr {
                return Some(s.value.to_string());
            }
        }
    }

    None
}
