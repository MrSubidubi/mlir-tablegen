//! Completion-label classification for MLIR-family language servers.
//!
//! Labels arrive without request context or many original LSP fields, so these
//! rules use only the server id plus `Completion` fields exposed by Zed.

use crate::server::{MlirServer, PdllServer};
use zed_extension_api::{self as zed, lsp::Completion, lsp::CompletionKind};

/// Return a literal-span label, or `None` to keep Zed's default styling.
pub fn label_for(server_id: &str, completion: &Completion) -> Option<zed::CodeLabel> {
    let highlight = match server_id {
        MlirServer::SERVER_ID => mlir_completion_highlight(completion),
        PdllServer::SERVER_ID => pdll_completion_highlight(completion),
        _ => return None,
    }?;
    Some(code_label_for_literal(&completion.label, highlight))
}

// Track the server's hardcoded completion menus, not the larger grammar.
const MLIR_BUILTIN_TYPES: &[&str] = &[
    "memref", "tensor", "complex", "tuple", "vector", "bf16", "f16", "f32", "f64", "f80", "f128",
    "index", "none", "i<N>", "si<N>", "ui<N>",
];

// Attribute-value introducers that take a payload (dense<…>, affine_map<…>,
// sparse<…>) plus the bare `unit` literal. Upstream tree-sitter-mlir tags the
// payload forms `@constructor.builtin`; the extension's highlights.scm flattens
// that to `@constant.builtin` (Zed has no `@constructor.builtin`), so
// completion uses the same capture to stay visually consistent with the buffer.
const MLIR_BUILTIN_ATTR_LITERALS: &[&str] = &[
    "affine_map",
    "affine_set",
    "dense",
    "dense_resource",
    "sparse",
    "unit",
];

// `loc` is a keyword-proper in the grammar, not a literal introducer.
const MLIR_BUILTIN_ATTR_KEYWORDS: &[&str] = &["loc"];

// `true` / `false` are classified separately as booleans.

// PDLL core constraints, from PDLLServer.cpp `completeCoreConstraint`.
const PDLL_CORE_CONSTRAINTS: &[&str] = &["Op", "Attr", "Type", "Value", "ValueRange", "TypeRange"];

fn code_label_for_literal(label: &str, highlight: &'static str) -> zed::CodeLabel {
    let label = label.to_string();
    let filter_range = (0..label.len() as u32).into();
    zed::CodeLabel {
        // Literal spans do not index into `code`; keep it aligned for tools and
        // any future CodeRange-based labels.
        code: label.clone(),
        spans: vec![zed::CodeLabelSpan::literal(
            label,
            Some(highlight.to_string()),
        )],
        filter_range,
    }
}

/// Classify an MLIR completion item. Branch order is part of the policy.
fn mlir_completion_highlight(completion: &Completion) -> Option<&'static str> {
    let kind = completion.kind.as_ref()?;
    let detail = completion.detail.as_deref();
    let label = &completion.label;

    if matches!(kind, CompletionKind::Variable) {
        return Some("variable.special");
    }

    if matches!(kind, CompletionKind::Field) && label.starts_with('^') {
        return Some("label");
    }

    // Keep prefixed and unprefixed dialect names visually consistent.
    if matches!(kind, CompletionKind::Module) && detail == Some("dialect") {
        return Some("type");
    }

    // Aliases arrive as `Field` with an `alias:` detail. At a fresh type/attr
    // position the label keeps its `#`/`!` prefix, so attribute and type aliases
    // are distinguishable. But once the user types the `#`/`!` trigger (the
    // common autocomplete flow), the server strips the prefix
    // (completeDialectTypeOrAlias / completeDialectAttributeOrAlias) and the two
    // become indistinguishable per item. Default the prefixless form to "type"
    // — the extension's general "named entity" color (also dialects, type
    // aliases, user constraints) — so aliases stay colored instead of falling
    // back to the default foreground. Flip to "attribute" if that reads better.
    if matches!(kind, CompletionKind::Field) && detail.is_some_and(|d| d.starts_with("alias:")) {
        if label.starts_with('#') {
            return Some("attribute");
        }
        return Some("type");
    }

    if matches!(kind, CompletionKind::Keyword) {
        return Some("keyword");
    }

    if matches!(kind, CompletionKind::Field) && detail == Some("operation") {
        return Some("function");
    }

    // Server-side `Field` is overloaded; detail-free items need label tables.
    if matches!(kind, CompletionKind::Field) {
        if MLIR_BUILTIN_TYPES.contains(&label.as_str()) {
            return Some("type.builtin");
        }
        if label == "true" || label == "false" {
            return Some("boolean");
        }
        if MLIR_BUILTIN_ATTR_LITERALS.contains(&label.as_str()) {
            return Some("constant.builtin");
        }
        if MLIR_BUILTIN_ATTR_KEYWORDS.contains(&label.as_str()) {
            return Some("keyword");
        }
    }

    None
}

/// Classify a PDLL completion item. Branch order is part of the policy.
fn pdll_completion_highlight(completion: &Completion) -> Option<&'static str> {
    let kind = completion.kind.as_ref()?;
    let detail = completion.detail.as_deref();
    let label = &completion.label;

    if matches!(kind, CompletionKind::Folder | CompletionKind::File) {
        return Some("string");
    }

    // Core and inline-typed constraints are builtins; user constraints arrive as
    // `Interface`, not here. Inline-typed forms append a type argument
    // (`Attr<type>`), so match on the base name before `<`. An unrecognized name
    // is a future/unknown constraint kind — fall back to "type".
    if matches!(kind, CompletionKind::Class) && detail.is_some_and(|d| d.ends_with(" constraint")) {
        let base = label.split('<').next().unwrap_or(label);
        if PDLL_CORE_CONSTRAINTS.contains(&base) {
            return Some("type.builtin");
        }
        return Some("type");
    }

    if matches!(kind, CompletionKind::Class) && detail == Some("pattern metadata") {
        return Some("variable");
    }

    if matches!(kind, CompletionKind::Class) && detail.is_none() {
        return Some("type");
    }

    if matches!(kind, CompletionKind::Interface) {
        return Some("type");
    }

    if matches!(kind, CompletionKind::Field) && detail.is_some_and(starts_with_digit_colon) {
        return Some("property");
    }

    if matches!(kind, CompletionKind::Field) && detail == Some("optional") {
        return Some("property");
    }

    // PDLL drops empty details before Zed sees them, so operation names and
    // non-optional operation attributes are indistinguishable here.
    if matches!(kind, CompletionKind::Field) && detail.is_none() {
        return Some("constant");
    }

    None
}

/// Match PDLL member details such as `"0: Value"` or `"10: ValueRange"`.
fn starts_with_digit_colon(s: &str) -> bool {
    let rest = s.trim_start_matches(|c: char| c.is_ascii_digit());
    rest.len() < s.len() && rest.starts_with(':')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::TablegenServer;

    fn completion(label: &str, kind: CompletionKind, detail: Option<&str>) -> Completion {
        Completion {
            label: label.to_string(),
            label_details: None,
            detail: detail.map(|s| s.to_string()),
            kind: Some(kind),
            insert_text_format: None,
        }
    }

    #[test]
    fn unknown_server_is_none() {
        let c = completion("%0", CompletionKind::Variable, None);
        assert!(label_for(TablegenServer::SERVER_ID, &c).is_none());
    }

    #[test]
    fn known_server_builds_label() {
        let c = completion("%0", CompletionKind::Variable, None);
        let label = label_for(MlirServer::SERVER_ID, &c).expect("should classify");
        assert_eq!(label.code, "%0");
    }

    #[test]
    fn mlir_ssa_value() {
        let c = completion("%0", CompletionKind::Variable, None);
        assert_eq!(mlir_completion_highlight(&c), Some("variable.special"));

        let c = completion("%arg0", CompletionKind::Variable, None);
        assert_eq!(mlir_completion_highlight(&c), Some("variable.special"));
    }

    #[test]
    fn mlir_block_label() {
        let c = completion("^bb0", CompletionKind::Field, None);
        assert_eq!(mlir_completion_highlight(&c), Some("label"));
    }

    #[test]
    fn mlir_dialect_name() {
        let c = completion("builtin", CompletionKind::Module, Some("dialect"));
        assert_eq!(mlir_completion_highlight(&c), Some("type"));

        let c = completion("#builtin", CompletionKind::Module, Some("dialect"));
        assert_eq!(mlir_completion_highlight(&c), Some("type"));

        let c = completion("!builtin", CompletionKind::Module, Some("dialect"));
        assert_eq!(mlir_completion_highlight(&c), Some("type"));
    }

    #[test]
    fn mlir_alias() {
        let c = completion("#map", CompletionKind::Field, Some("alias: ..."));
        assert_eq!(mlir_completion_highlight(&c), Some("attribute"));

        let c = completion("!shape", CompletionKind::Field, Some("alias: ..."));
        assert_eq!(mlir_completion_highlight(&c), Some("type"));
    }

    #[test]
    fn mlir_alias_prefix_stripped() {
        // When completion fires after the user types `!`/`#`, the server strips
        // the prefix (completeDialectTypeOrAlias / completeDialectAttributeOrAlias).
        // attr and type aliases become indistinguishable; both default to "type"
        // so they still get colored instead of falling back to None.
        let c = completion("my_type_alias", CompletionKind::Field, Some("alias: i32"));
        assert_eq!(mlir_completion_highlight(&c), Some("type"));

        let c = completion(
            "my_attr_alias",
            CompletionKind::Field,
            Some("alias: 42 : i32"),
        );
        assert_eq!(mlir_completion_highlight(&c), Some("type"));
    }

    #[test]
    fn mlir_keyword_token() {
        let c = completion("affine_set", CompletionKind::Keyword, Some("optional"));
        assert_eq!(mlir_completion_highlight(&c), Some("keyword"));

        let c = completion("dense", CompletionKind::Keyword, None);
        assert_eq!(mlir_completion_highlight(&c), Some("keyword"));
    }

    #[test]
    fn mlir_operation_name() {
        let c = completion("func", CompletionKind::Field, Some("operation"));
        assert_eq!(mlir_completion_highlight(&c), Some("function"));
    }

    #[test]
    fn mlir_builtin_type() {
        for name in &["memref", "tensor", "i<N>", "f32"] {
            let c = completion(name, CompletionKind::Field, None);
            assert_eq!(
                mlir_completion_highlight(&c),
                Some("type.builtin"),
                "builtin type {name} should map to type.builtin",
            );
        }
    }

    #[test]
    fn mlir_boolean_literal() {
        let c = completion("true", CompletionKind::Field, None);
        assert_eq!(mlir_completion_highlight(&c), Some("boolean"));

        let c = completion("false", CompletionKind::Field, None);
        assert_eq!(mlir_completion_highlight(&c), Some("boolean"));
    }

    #[test]
    fn mlir_builtin_attribute_constant() {
        // Payload-taking introducers (dense, sparse, affine_map, ...) plus the
        // bare `unit` literal render as @constant.builtin in the grammar
        // (languages/mlir/highlights.scm; upstream tags the payload forms
        // @constructor.builtin, which Zed flattens to @constant.builtin), so
        // completion matches the buffer.
        for name in &[
            "affine_map",
            "affine_set",
            "dense",
            "dense_resource",
            "sparse",
            "unit",
        ] {
            let c = completion(name, CompletionKind::Field, None);
            assert_eq!(
                mlir_completion_highlight(&c),
                Some("constant.builtin"),
                "builtin attribute introducer {name} should map to constant.builtin",
            );
        }
    }

    #[test]
    fn mlir_builtin_attribute_keyword() {
        // `loc` is the one builtin-attribute name the grammar renders as
        // @keyword (trailing_location / location), not @constant.builtin.
        let c = completion("loc", CompletionKind::Field, None);
        assert_eq!(mlir_completion_highlight(&c), Some("keyword"));
    }

    #[test]
    fn mlir_unknown_field() {
        let c = completion("unknown_thing", CompletionKind::Field, None);
        assert_eq!(mlir_completion_highlight(&c), None);
    }

    #[test]
    fn mlir_dialect_before_prefix() {
        let c = completion("#builtin", CompletionKind::Module, Some("dialect"));
        assert_eq!(
            mlir_completion_highlight(&c),
            Some("type"),
            "dialect must precede alias prefix"
        );
    }

    #[test]
    fn pdll_include_path() {
        let c = completion("file.pdll", CompletionKind::File, None);
        assert_eq!(pdll_completion_highlight(&c), Some("string"));

        let c = completion("dir/", CompletionKind::Folder, None);
        assert_eq!(pdll_completion_highlight(&c), Some("string"));
    }

    #[test]
    fn pdll_core_constraint() {
        for name in &["Op", "Attr", "Type", "Value", "ValueRange", "TypeRange"] {
            let c = completion(name, CompletionKind::Class, Some("Attr constraint"));
            assert_eq!(
                pdll_completion_highlight(&c),
                Some("type.builtin"),
                "core constraint {name} should map to type.builtin",
            );
        }
    }

    #[test]
    fn pdll_inline_typed_constraint() {
        // Server sends detail "Attr<type> constraint" for the inline form
        // (template argument appended to the constraint name before the suffix).
        let c = completion(
            "Attr<type>",
            CompletionKind::Class,
            Some("Attr<type> constraint"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("type.builtin"));

        let c = completion(
            "Value<type>",
            CompletionKind::Class,
            Some("Value<type> constraint"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("type.builtin"));

        let c = completion(
            "ValueRange<type>",
            CompletionKind::Class,
            Some("ValueRange<type> constraint"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("type.builtin"));
    }

    #[test]
    fn pdll_other_constraint() {
        let c = completion("MyConstraint", CompletionKind::Class, Some("My constraint"));
        assert_eq!(pdll_completion_highlight(&c), Some("type"));
    }

    #[test]
    fn pdll_pattern_metadata() {
        let c = completion("benefit", CompletionKind::Class, Some("pattern metadata"));
        assert_eq!(pdll_completion_highlight(&c), Some("variable"));

        let c = completion("recursion", CompletionKind::Class, Some("pattern metadata"));
        assert_eq!(pdll_completion_highlight(&c), Some("variable"));
    }

    #[test]
    fn pdll_dialect_name() {
        let c = completion("arith", CompletionKind::Class, None);
        assert_eq!(pdll_completion_highlight(&c), Some("type"));

        let c = completion("builtin", CompletionKind::Class, None);
        assert_eq!(pdll_completion_highlight(&c), Some("type"));
    }

    #[test]
    fn pdll_user_constraint() {
        let c = completion(
            "MyConstraint",
            CompletionKind::Interface,
            Some("(Input: Type) -> RetType"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("type"));
    }

    #[test]
    fn pdll_tuple_member() {
        let c = completion("0 (field #0)", CompletionKind::Field, Some("0: Value"));
        assert_eq!(pdll_completion_highlight(&c), Some("property"));
    }

    #[test]
    fn pdll_result_member() {
        let c = completion(
            "result_name (field #0)",
            CompletionKind::Field,
            Some("0: ValueRange"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("property"));
    }

    #[test]
    fn pdll_multi_digit_member() {
        let c = completion(
            "10 (field #10)",
            CompletionKind::Field,
            Some("10: ValueRange"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("property"));
    }

    #[test]
    fn pdll_optional_attribute() {
        let c = completion("attr_name", CompletionKind::Field, Some("optional"));
        assert_eq!(pdll_completion_highlight(&c), Some("property"));
    }

    #[test]
    fn pdll_operation_name() {
        let c = completion("AddOp", CompletionKind::Field, None);
        assert_eq!(pdll_completion_highlight(&c), Some("constant"));

        let c = completion("some_attr", CompletionKind::Field, None);
        assert_eq!(pdll_completion_highlight(&c), Some("constant"));
    }

    #[test]
    fn pdll_keyword_kind_unclassified() {
        // PDLLServer never emits CompletionItemKind::Keyword, so it is left to
        // Zed's default styling rather than force-colored.
        let c = completion("some_keyword", CompletionKind::Keyword, None);
        assert_eq!(pdll_completion_highlight(&c), None);
    }

    #[test]
    fn code_label_for_literal_basic() {
        let label = code_label_for_literal("abc", "function");
        assert_eq!(label.code, "abc");
        assert_eq!(label.spans.len(), 1);
        assert_eq!(label.filter_range.start, 0);
        assert_eq!(label.filter_range.end, 3);
    }
}
