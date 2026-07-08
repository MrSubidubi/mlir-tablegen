;; ---------------------------------------------------------------------------
;; Dialect-agnostic MLIR syntax highlighting
;;
;; Zed's tree-sitter highlighter uses last-match-wins: when multiple rules
;; overlap, the one that appears *later* in the file wins. The
;; `(custom_operation (bare_id) @keyword)` fallback near the end must therefore
;; come *before* any more specific `bare_id` captures (e.g. inside `attribute_entry` or
;; `dense_resource_literal`) so those later rules override it.
;; ---------------------------------------------------------------------------

;; Comments
(comment) @comment

;; Keywords and Operation names
(func_operation name: _ @function)
(func_operation visibility: _ @keyword)
(func_operation specifier: (function_specifier) @keyword)
(func_operation "attributes" @attribute)
;; llvm.func post-signature clause introducers.
(vscale_range "vscale_range" @keyword)
(comdat "comdat" @keyword)
(module_operation name: _ @function)
(module_operation "attributes" @attribute)

(custom_op_name) @function

(custom_operation ["array" "sparse" "tensor" "vector"] @keyword)
(custom_operation ["+" "-" "*" "/" "&" "|" "~"] @operator)
(custom_operation "?" @punctuation.special)
(custom_operation "loc" @keyword)
(custom_operation "module(" @keyword)
(custom_operation ">" @punctuation.bracket)

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
  (token_type)
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
(dim_list ["?" "*"] @punctuation.special)
(dialect_dim_list ["?" "*"] @punctuation.special)
(custom_body_dim_list "?" @punctuation.special)

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
  (array_literal)
  (unit_literal)
  (uninitialized_literal)
] @constant.builtin

;; Builtin attribute/literal introducers (dense, sparse, array, affine_map,
;; affine_set, dense_resource) are @constant.builtin — the Zed translation of
;; the VSCode grammar's constant.language anchor, not general syntax keywords
;; like loc or structural fields like offset. Zed's official capture list has
;; no @constructor.builtin, @constructor renders identical to @function in One
;; Dark (op names), and @property stays reserved for key-side channels, so
;; upstream's finer @constructor.builtin split deliberately flattens here.
(tensor_literal ["dense" "sparse"] @constant.builtin)
(array_literal "array" @constant.builtin)

[
  (attribute_alias)
  (attribute_alias_def)
  (builtin_attribute)
  (dialect_attribute)
] @attribute

(dictionary_attribute) @attribute

;; Builtin attribute and affine keywords
(affine_map "affine_map" @constant.builtin)
(affine_set "affine_set" @constant.builtin)
(affine_map (bare_id) @variable.parameter)
(affine_set (bare_id) @variable.parameter)
(affine_map ["symbol" "max" "min"] @keyword)
(affine_set ["symbol" "max" "min"] @keyword)
(affine_map
  ["dense" "sparse" "compressed" "singleton" "loose_compressed" "n_out_of_m"]
  @keyword)
(affine_set
  ["dense" "sparse" "compressed" "singleton" "loose_compressed" "n_out_of_m"]
  @keyword)
(affine_map ["+" "-" "*" "==" ">=" "<="] @operator)
(affine_set ["+" "-" "*" "==" ">=" "<="] @operator)
(properties ["<{" "}>"] @punctuation.bracket)
(strided_layout "strided" @keyword)
(strided_layout "offset" @keyword)
(strided_layout ["?" "*"] @punctuation.special)
(distinct_attribute "distinct" @keyword)
(dense_resource_literal "dense_resource" @constant.builtin)
["ceildiv" "floordiv" "mod"] @operator
(pretty_dialect_item_body
  ["array" "dense" "opaque" "sparse" "tensor" "vector"] @keyword)
(pretty_dialect_item_body (bare_id) @keyword)
(pretty_dialect_item_body ["?" "*"] @punctuation.special)

;; Locations
(trailing_location "loc" @keyword)
(callsite_location ["callsite" "at"] @keyword)
(fused_location "fused" @keyword)
(location "to" @keyword)
(unknown_location) @constant.builtin

;; Strings
(string_literal) @string
(generic_operation (string_literal) @function)
(escape_sequence) @string.escape

;; Symbols (@name) — all symbol references share one capture
(symbol_ref_id) @string.special.symbol

;; SSA values — @variable.special gives a distinct color
[(value_use) (op_result)] @variable.special

;; Fallback keyword for ad-hoc custom operation body tokens like `to`, `step`,
;; `ins`, and `outs`. Keep this scoped so attribute keys, affine dimensions,
;; and pretty dialect payload identifiers do not all become keywords.
(custom_operation (bare_id) @keyword)

;; Handle name inside dense_resource<...> — must come after the bare_id
;; catch-all so it wins under last-match-wins.
(dense_resource_literal (bare_id) @constant.builtin)

;; Dictionary attribute keys (e.g. `predicate` in `{predicate = 1}`)
(attribute_entry (bare_id) @attribute)
(attribute_entry (string_literal) @attribute)

;; External resource block delimiters
(external_resources ["{-#" "#-}"] @punctuation.bracket)

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

;; Operators
[
  "="
  "->"
  "::"
] @operator

;; Block labels
(caret_id) @label
