use swc_experimental_ecma_parser::{EsSyntax, Parser, StringSource, Syntax};
use swc_experimental_ecma_semantic::resolver::resolver;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn main() {
    let source = include_str!("../files/typescript.js");
    let syntax = Syntax::Es(EsSyntax::default());
    let input = StringSource::new(source);

    let parser = Parser::new(syntax, input, None);
    let ret = parser.parse_program().unwrap();

    let semantic = resolver(ret.root, &ret.ast);
    let _legacy = swc_experimental_ecma_ast_compat::AstCompat::new(&ret.ast, &semantic)
        .compat_program(ret.root);
}
