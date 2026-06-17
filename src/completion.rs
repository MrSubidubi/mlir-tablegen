//! Completion-label classification for `label_for_completion`: each supported
//! server maps a `Completion` to a Zed highlight name, wrapped in a single
//! literal `CodeLabel` span.

use zed_extension_api::{self as zed, lsp::Completion, lsp::CompletionKind};

/// Build a highlighted [`zed::CodeLabel`] for a completion item, dispatched by
/// language-server ID. `None` falls back to Zed's default styling.
pub fn label_for(server_id: &str, completion: &Completion) -> Option<zed::CodeLabel> {
    let highlight = match server_id {
        "mlir-lsp-server" => mlir_completion_highlight(completion),
        "mlir-pdll-lsp-server" => pdll_completion_highlight(completion),
        _ => return None,
    }?;
    Some(code_label_for_literal(&completion.label, highlight))
}

// Builtin-type / attribute labels mirror `mlir-lsp-server`'s hardcoded
// completion menu (`completeType` / `completeAttribute` in MLIRServer.cpp),
// which is intentionally smaller than the grammar's surface — track the server,
// not tree-sitter-mlir.
const MLIR_BUILTIN_TYPES: &[&str] = &[
    "memref", "tensor", "complex", "tuple", "vector", "bf16", "f16", "f32", "f64", "f80", "f128",
    "index", "none", "i<N>", "si<N>", "ui<N>",
];

// `true` / `false` are excluded; they are colored as booleans below.
const MLIR_BUILTIN_ATTRIBUTE_KEYWORDS: &[&str] = &[
    "affine_set",
    "affine_map",
    "dense",
    "dense_resource",
    "loc",
    "sparse",
    "unit",
];

// PDLL core constraints, from PDLLServer.cpp `completeCoreConstraint`.
const PDLL_CORE_CONSTRAINTS: &[&str] = &["Op", "Attr", "Type", "Value", "ValueRange", "TypeRange"];

fn code_label_for_literal(label: &str, highlight: &'static str) -> zed::CodeLabel {
    let label = label.to_string();
    // Completion labels are ASCII bare-ids, so byte length == char length.
    let filter_range = (0..label.len() as u32).into();
    zed::CodeLabel {
        // `code` is unused by literal spans; kept for a future CodeRange span.
        code: label.clone(),
        spans: vec![zed::CodeLabelSpan::literal(
            label,
            Some(highlight.to_string()),
        )],
        filter_range,
    }
}

/// Classify an MLIR completion item; `None` keeps Zed's default styling.
/// Branches are ordered most-specific first.
fn mlir_completion_highlight(completion: &Completion) -> Option<&'static str> {
    let kind = completion.kind.as_ref()?;
    let detail = completion.detail.as_deref();
    let label = &completion.label;

    if matches!(kind, CompletionKind::Variable) {
        return Some("variable.special"); // SSA value
    }

    if matches!(kind, CompletionKind::Field) && label.starts_with('^') {
        return Some("label"); // block name
    }

    // Dialect name. Before the alias branch so `builtin`, `#builtin` and
    // `!builtin` share one color.
    if matches!(kind, CompletionKind::Module) && detail == Some("dialect") {
        return Some("type");
    }

    // Attribute / type alias, split by the `#` / `!` prefix.
    // When the user has already typed `#` / `!`, the server emits aliases
    // without the prefix (completeDialectAttributeOrAlias /
    // completeDialectTypeOrAlias); those fall through to `None` because the
    // prefix is the only way to tell attribute aliases from type aliases.
    if matches!(kind, CompletionKind::Field) && detail.is_some_and(|d| d.starts_with("alias:")) {
        if label.starts_with('#') {
            return Some("attribute");
        }
        if label.starts_with('!') {
            return Some("type");
        }
    }

    if matches!(kind, CompletionKind::Keyword) {
        return Some("keyword"); // expected token
    }

    if matches!(kind, CompletionKind::Field) && detail == Some("operation") {
        return Some("function"); // operation name
    }

    // Remaining `Field` items have no distinguishing detail: classify by label.
    if matches!(kind, CompletionKind::Field) {
        if MLIR_BUILTIN_TYPES.contains(&label.as_str()) {
            return Some("type.builtin");
        }
        if label == "true" || label == "false" {
            return Some("boolean");
        }
        if MLIR_BUILTIN_ATTRIBUTE_KEYWORDS.contains(&label.as_str()) {
            return Some("keyword");
        }
    }

    None
}

/// Classify a PDLL completion item; `None` keeps Zed's default styling.
/// Branches are ordered most-specific first.
fn pdll_completion_highlight(completion: &Completion) -> Option<&'static str> {
    let kind = completion.kind.as_ref()?;
    let detail = completion.detail.as_deref();
    let label = &completion.label;

    if matches!(kind, CompletionKind::Folder | CompletionKind::File) {
        return Some("string"); // include path
    }

    // Constraint: core and inline-typed forms are builtins, others user types.
    if matches!(kind, CompletionKind::Class) && detail.is_some_and(|d| d.ends_with(" constraint")) {
        if label.starts_with("Attr<")
            || label.starts_with("Value<")
            || label.starts_with("ValueRange<")
        {
            return Some("type.builtin");
        }
        if PDLL_CORE_CONSTRAINTS.contains(&label.as_str()) {
            return Some("type.builtin");
        }
        return Some("type");
    }

    if matches!(kind, CompletionKind::Class) && detail == Some("pattern metadata") {
        return Some("variable"); // benefit / recursion
    }

    if matches!(kind, CompletionKind::Class) && detail.is_none() {
        return Some("type"); // dialect name
    }

    if matches!(kind, CompletionKind::Interface) {
        return Some("type"); // user constraint
    }

    if matches!(kind, CompletionKind::Field) && detail.is_some_and(starts_with_digit_colon) {
        return Some("property"); // tuple / result member, detail like "0: Value"
    }

    if matches!(kind, CompletionKind::Field) && detail == Some("optional") {
        return Some("property"); // optional operation attribute
    }

    // Operation name → "constant" (matches the grammar's `op_name`). A
    // non-optional operation attribute sends an empty detail the server drops,
    // so it is indistinguishable and lands here too.
    if matches!(kind, CompletionKind::Field) && detail.is_none() {
        return Some("constant");
    }

    if matches!(kind, CompletionKind::Keyword) {
        return Some("keyword");
    }

    None
}

/// Whether `s` starts with one or more digits then `':'` — the shape of PDLL
/// member details like "0: Value" or "10: ValueRange".
fn starts_with_digit_colon(s: &str) -> bool {
    let rest = s.trim_start_matches(|c: char| c.is_ascii_digit());
    rest.len() < s.len() && rest.starts_with(':')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn completion(label: &str, kind: CompletionKind, detail: Option<&str>) -> Completion {
        Completion {
            label: label.to_string(),
            label_details: None,
            detail: detail.map(|s| s.to_string()),
            kind: Some(kind),
            insert_text_format: None,
        }
    }

    // --- dispatch ---

    #[test]
    fn unknown_server_is_none() {
        // `CodeLabel` has no `PartialEq`, so assert on `is_none()`.
        let c = completion("%0", CompletionKind::Variable, None);
        assert!(label_for("tblgen-lsp-server", &c).is_none());
    }

    #[test]
    fn known_server_builds_label() {
        let c = completion("%0", CompletionKind::Variable, None);
        let label = label_for("mlir-lsp-server", &c).expect("should classify");
        assert_eq!(label.code, "%0");
    }

    // --- MLIR tests ---

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

        // Prefixed dialects share the unprefixed color.
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
    fn mlir_keyword_token() {
        let c = completion("affine_set", CompletionKind::Keyword, Some("optional"));
        assert_eq!(mlir_completion_highlight(&c), Some("keyword"));

        // Keyword rule ignores detail.
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
    fn mlir_builtin_attribute_keyword() {
        for name in &["dense", "affine_map", "affine_set", "loc", "sparse", "unit"] {
            let c = completion(name, CompletionKind::Field, None);
            assert_eq!(
                mlir_completion_highlight(&c),
                Some("keyword"),
                "builtin attribute keyword {name} should map to keyword",
            );
        }
    }

    #[test]
    fn mlir_unknown_field() {
        let c = completion("unknown_thing", CompletionKind::Field, None);
        assert_eq!(mlir_completion_highlight(&c), None);
    }

    #[test]
    fn mlir_dialect_before_prefix() {
        // Dialect classification must win over the alias-prefix branch.
        let c = completion("#builtin", CompletionKind::Module, Some("dialect"));
        assert_eq!(
            mlir_completion_highlight(&c),
            Some("type"),
            "dialect must precede alias prefix"
        );
    }

    // --- PDLL tests ---

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
        let c = completion("Attr<type>", CompletionKind::Class, Some("Attr constraint"));
        assert_eq!(pdll_completion_highlight(&c), Some("type.builtin"));

        let c = completion(
            "Value<type>",
            CompletionKind::Class,
            Some("Value constraint"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("type.builtin"));

        let c = completion(
            "ValueRange<type>",
            CompletionKind::Class,
            Some("ValueRange constraint"),
        );
        assert_eq!(pdll_completion_highlight(&c), Some("type.builtin"));
    }

    #[test]
    fn pdll_other_constraint() {
        // Non-core, non-inline constraint → "type".
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
        // A two-digit member index must still classify as a member.
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

        // A non-optional operation attribute (empty detail dropped by the
        // server) is indistinguishable and lands in the same branch.
        let c = completion("some_attr", CompletionKind::Field, None);
        assert_eq!(pdll_completion_highlight(&c), Some("constant"));
    }

    #[test]
    fn pdll_keyword() {
        let c = completion("some_keyword", CompletionKind::Keyword, None);
        assert_eq!(pdll_completion_highlight(&c), Some("keyword"));
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
