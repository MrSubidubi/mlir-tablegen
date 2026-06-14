;; ---------------------------------------------------------------------------
;; Dialect-agnostic MLIR syntax highlighting
;;
;; Zed's tree-sitter highlighter uses last-match-wins: when multiple rules
;; overlap, the one that appears *later* in the file wins. The `(bare_id)
;; @keyword` fallback near the end must therefore come *before* any more
;; specific `bare_id` captures (e.g. inside `attribute_entry` or
;; `dense_resource_literal`) so those later rules override it.
;; ---------------------------------------------------------------------------

;; Comments
(comment) @comment

;; Keywords and Operation names
(func_operation name: _ @function)
(func_operation visibility: _ @attribute)
(func_operation specifier: (function_specifier) @keyword)
(func_operation "attributes" @attribute)
(module_operation name: _ @function)
(module_operation "attributes" @attribute)

(custom_op_name) @function
(generic_operation (string_literal) @function)

;; Types
(builtin_type) @type.builtin

;; Two-tone highlighting inside composite types (tensor, memref, vector):
(dimension_size) @number

;; Element types inside dim_list / vector_dim_list are named nodes — their
;; capture is deeper than (dim_list)/@number so they win for their own span.
[
  (float_type)
  (integer_type)
  (index_type)
  (none_type)
] @type.builtin

;; Composite types not always reachable through (builtin_type)
[
  (complex_type)
  (memref_type)
  (tensor_type)
  (vector_type)
  (tuple_type)
  (opaque_type)
] @type.builtin

;; 'x' separator inside dimension lists — render as delimiter rather than
;; inheriting @type.builtin from the (memref_type)/(tensor_type)/(vector_type)
;; block above. Must appear after that block under last-match-wins.
;; @cap binds to the "x" literal (parent-internal anonymous token); placing
;; it outside an alternation would capture the parent node instead.
(dim_list "x" @punctuation.delimiter)
(dimension_separator) @punctuation.delimiter
(vector_dim_list "x" @punctuation.delimiter)

[
  (type_alias)
  (dialect_type)
  (type_alias_def)
] @type

;; Numeric and Bool literals
[
  (integer_literal)
  (float_literal)
  (complex_literal)
] @number

(bool_literal) @boolean

;; Attributes and other constants
[
  (tensor_literal)
  (dense_resource_literal)
  (array_literal)
  (unit_literal)
  (uninitialized_literal)
] @constant.builtin

[
  (attribute_alias)
  (attribute_alias_def)
  (dialect_attribute)
] @attribute

(dictionary_attribute) @attribute

;; Builtin attribute and affine keywords
(affine_map "affine_map" @keyword)
(affine_set "affine_set" @keyword)
(affine_map ["symbol" "max" "min"] @keyword)
(affine_set ["symbol" "max" "min"] @keyword)
(strided_layout "strided" @keyword)
(strided_layout "offset" @keyword)
(distinct_attribute "distinct" @keyword)
["ceildiv" "floordiv" "mod"] @operator

;; Locations
(trailing_location "loc" @keyword)
(callsite_location ["callsite" "at"] @keyword)
(fused_location "fused" @keyword)
(location "to" @keyword)
(unknown_location) @constant.builtin

;; Strings
(string_literal) @string
(escape_sequence) @string.escape

;; Functions and symbols
;; func.func @name  →  @string.special.symbol  (function definition name)
;; module @name     →  @string.special.symbol  (module definition name)
;; @any_other_sym   →  @string.special.symbol  (symbol reference)
(func_operation sym_name: (symbol_ref_id) @string.special.symbol)
(module_operation sym_name: (symbol_ref_id) @string.special.symbol)
(symbol_ref_id) @string.special.symbol

;; SSA values — @variable.special gives a distinct color
(value_use) @variable.special
(func_arg_list (value_use) @variable.special)
(block_arg_list (value_use) @variable.special)
(op_result) @variable.special

;; Fallback keyword for ad-hoc tokens like `to`, `step`, `ins`, etc.
;; This rule must appear *before* any more specific `(bare_id)` capture
;; (e.g. `attribute_entry`, `dense_resource_literal`) so the later rule
;; wins under Zed's last-match-wins semantics.
(bare_id) @keyword

;; Recent parser versions treat some custom/dialect body words as anonymous
;; tokens so they no longer match the bare_id fallback above.
(custom_operation ["array" "sparse" "tensor" "vector"] @keyword)
(pretty_dialect_item_body
  ["array" "dense" "opaque" "sparse" "tensor" "vector"] @keyword)

;; Handle name inside dense_resource<...> — must come after the bare_id
;; catch-all so it wins under last-match-wins.
(dense_resource_literal (bare_id) @constant.builtin)

;; Dictionary attribute keys (e.g. `predicate` in `{predicate = 1}`)
(attribute_entry (bare_id) @attribute)

;; Brackets
[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
  "<"
  ">"
] @punctuation.bracket

;; Delimiters
[
  ":"
  ","
] @punctuation.delimiter

;; Variadic / dynamic markers
(variadic) @punctuation.special
"?" @punctuation.special

;; Operators
[
  "="
  "->"
  "::"
  "*"
] @operator

;; Unranked dimension marker `*` in memref<*xf32> / tensor<*xf32> — render as
;; a dimension placeholder (sibling of the `x` separator) rather than
;; inheriting @operator from the block above. `_dim_primitive` is hidden so the
;; `*` token is hoisted directly into (dim_list); vector_dim_list disallows it
;; by construction. Must appear after the operator block under last-match-wins.
(dim_list "*" @punctuation.special)

;; Block labels
(caret_id) @label
