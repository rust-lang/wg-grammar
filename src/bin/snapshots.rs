#![deny(rust_2018_idioms)]

use {
    std::{fmt::Debug, fs, process::exit},
    insta::assert_snapshot_matches,
    rust_grammar::parse,
    walkdir::WalkDir,
    regex::Regex,
    lazy_static::lazy_static,
};

fn to_debug_str(debug: &dyn Debug) -> String {
    format!("{:#?}", debug)
}

macro_rules! snapshot {
    ($production:ident, $src:expr) => {
        match $src.parse::<proc_macro2::TokenStream>() {
            Ok(tts) => parse::$production::parse_with(tts, |_, result| to_debug_str(&result)),
            // FIXME(eddyb) provide more information in this error case.
            Err(_) => to_debug_str(&Err::<(), _>(parse::ParseError::<()>::NoParse)),
        }
    };
}

macro_rules! dispatch {
    ($src:ident, $s:expr; $($prod:ident)*) => {
        match $s {
            $(stringify!($prod) => snapshot!($prod, $src),)*
            prod => panic!("Unexpected production {} tested", prod),
        }
    };
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\(\s+_,?\s+\)").unwrap();
}

fn test_snapshot(file: walkdir::DirEntry) {
    let path = file.path();
    let file_name = file.file_name().to_str().unwrap();
    let src = fs::read_to_string(path).unwrap();
    let production = &file_name[..file_name.find('.').unwrap_or(file_name.len())];
    let forest = dispatch! { src, production;
        // abi.lyg
        Abi
        // attr.lyg
        OuterAttr InnerAttr Attr AttrInput
        // expr.lyg
        Expr ExprKind UnaryOp BinaryOp BinaryAssignOp FieldName StructExprFieldsAndBase
        StructExprField StructExprFieldKind Label
        If Cond ElseExpr MatchArm ClosureArg
        // generics.lyg
        Generics GenericParam GenericParamKind ForAllBinder WhereClause WhereBound LifetimeBound
        TypeBound TypeTraitBound GenericArgs AngleBracketGenericArgsAndBindings GenericArg
        TypeBinding
        // item.lyg
        ModuleContents Item ItemKind UseTree UseTreePrefix ForeignItem ForeignItemKind TraitItem
        TraitItemKind ImplItem ImplItemKind FnHeader FnDecl FnArgs FnArg EnumVariant EnumVariantKind
        StructBody TupleField RecordField
        // macro.lyg
        MacroCall MacroInput ItemMacroCall ItemMacroInput
        // pat.lyg
        Pat PatRangeValue Binding SlicePatElem TuplePatField StructPatFieldsAndEllipsis
        StructPatField
        // path.lyg
        Path RelativePath PathSegment QSelf QPath
        // stmt.lyg
        Stmt Block
        // type.lyg
        Type FnSigInputs FnSigInput
        // vis.lyg
        Vis VisRestriction
    };
    let forest = forest.replace("Span..Span", "_");
    let forest = RE.replace_all(&forest, "(_)");
    assert_snapshot_matches!(file_name, forest);
}

fn spawn_panicking(stack_size: usize, f: impl FnOnce() + Send + 'static) -> Result<(), ()> {
    crossbeam::scope(|scope: &crossbeam::thread::Scope<'_>| {
        scope
            .builder()
            .stack_size(stack_size) // 32 MiB
            .spawn(|_| f())
            .unwrap()
            .join()
            .map_err(drop)
    })
    .unwrap()
}

fn main() {
    // Find all the testdata `.input` files.
    let files = WalkDir::new("testdata")
        .contents_first(true)
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "input"));

    // Parse and snapshot each file
    let snapshots = files
        // .par_bridge() // parallel will interleave output, unfortunately
        .map(|f| spawn_panicking(32 * 1024 * 1024, || test_snapshot(f))); // 32 MiB

    // Collect failures
    let failures: Vec<_> = snapshots.filter_map(Result::err).collect();

    if failures.len() == 0 {
        println!("All snapshots passed!");
    } else {
        exit(1);
    }
}
