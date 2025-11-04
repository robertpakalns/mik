use swc_ecma_ast::{CallExpr, Callee, Expr, ModuleItem, Stmt};

mod llvm;
mod parser;

fn main() {
    let source = r#"
        console.log("mik")
    "#;

    let module = parser::js_ts_parser::parse(source.to_string());

    let mut log_string = None;
    for item in module.body.iter() {
        if let ModuleItem::Stmt(stmt) = item {
            if let Stmt::Expr(expr_stmt) = stmt {
                if let Expr::Call(CallExpr { callee, args, .. }) = &*expr_stmt.expr {
                    if let Callee::Expr(expr) = callee {
                        if let Expr::Member(member) = &**expr {
                            if let Expr::Ident(obj) = &*member.obj {
                                if obj.sym == *"console" {
                                    log_string =
                                        parser::parse_console::extract_console(member, args);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let log_string = log_string.expect("Only supports console.log(\"...\") with a string literal");

    llvm::compile(log_string);
}
