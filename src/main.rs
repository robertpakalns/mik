use swc_common::{
    FileName, SourceMap,
    errors::{ColorConfig, Handler},
    sync::Lrc,
};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

fn main() {
    let source = r#"
        console.log("mik")
    "#;

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let fm = cm.new_source_file(FileName::Custom("input.ts".into()).into(), source);

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            decorators: false,
            dts: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: false,
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for err in parser.take_errors() {
        err.into_diagnostic(&handler).emit();
    }

    let module = parser.parse_module().map_err(|err| {
        let _ = err.into_diagnostic(&handler);
    });

    println!("{module:#?}");
}
