;; Comments
(line_comment) @comment
(block_comment) @comment

;; Preprocessor
"#define"  @preproc
"#ifdef"   @preproc
"#ifndef"  @preproc
"#else"    @preproc
"#endif"   @preproc
(macro_name) @constant

"include" @keyword

;; Keywords
[
  "assert"
  "class"
  "multiclass"
  "field"
  "let"
  "def"
  "defm"
  "defset"
  "deftype"
  "defvar"
  "dump"
  "foreach"
  "if"
  "else"
  "then"
  "in"
] @keyword

(let_mode) @keyword

;; Identifier fallback — keep BEFORE the specific identifier rules below.
;; Zed's tree-sitter highlighter uses last-match-wins: later captures for the
;; same identifier node override this fallback.
(identifier) @variable

;; Broad heuristic — keep before context-specific and definition-name rules so
;; those later captures can override UPPER_CASE identifiers in semantic slots.
((identifier) @type
  (#match? @type "^_*[A-Z][A-Z0-9_]+$"))

;; Variable substitution (`$foo` / `$0` inside code_literal, `$bare` in DAG)
;; These are single-token nodes in the grammar, matching VS Code's `(\$\w+)\b`.
(variable_substitution) @variable.special
(variable_name) @variable.special

;; Template parameters — capture only the LHS name, not default-value refs.
(template_parameter (type) . (identifier) @variable.parameter)

;; Foreach iteration variable — capture only the first named child before `=`.
(foreach_statement . (identifier) @variable.parameter)

;; Member access: `Foo.bar`
(value_suffix_dot (identifier) @property)

;; Field declarations — capture only the declared field name, not RHS refs.
(field_declaration (type) . (identifier) @property)

;; Named arguments — capture only the argument name, not RHS refs.
(named_argument . (identifier) @property)

;; Let / defvar bindings — capture only LHS names, not RHS refs.
(let_assignment . (identifier) @variable)
(let_assignment . (let_mode) . (identifier) @variable)
(let_item . (identifier) @variable)
(let_item . (let_mode) . (identifier) @variable)
(defvar_statement . (identifier) @variable)

;; Types
(type) @type

[
  "bit"
  "bits"
  "int"
  "string"
  "dag"
  "code"
  "list"
] @type.builtin

;; Definition names — parser-aligned highlight groups.
;; `object_name` accepts paste expressions (`def NAME#"x"`), string literals
;; (`def "literal"`), and anonymous defs (`def : Parent`). Only the bare
;; identifier form is highlighted here; other forms are valid but uncolored.
(class_definition (identifier) @type)
(def_definition (object_name (identifier) @constant))
(multiclass_definition (identifier) @function)
(defm_definition (object_name (identifier) @type))
(defset_definition (identifier) @type)
(deftype_definition (identifier) @type)

;; Parent class references
(parent_class (identifier) @type)

;; Anonymous records
(anonymous_record (identifier) @type)

;; Bang operators / built-in functions.
;; `!cond` lives in its own grammar node (`cond_operator_call`) instead of the
;; flat `bang_operator` choice, so it needs an explicit anonymous-token rule
;; to receive the same highlight.
(bang_operator) @function
"!cond" @function

;; Operators
[
  "="
  "#"
  "-"
  "..."
] @operator

;; Punctuation
[ "{" "}" "[" "]" "(" ")" "<" ">" ] @punctuation.bracket

[
  "."
  ","
  ";"
  ":"
] @punctuation.delimiter

;; Literals
(string_literal) @string
(escape_sequence) @string.escape
(code_chunk) @string
(integer_literal) @number
(boolean_literal) @boolean
(unset_value) @constant.builtin

;; ─── MLIR dialect-flavor predicates ─────────────────────────────────────────
;; (Per spec §2.1: dialect identification is queries-side, not grammar-side.)

;; Common MLIR ODS base classes
(parent_class (identifier) @type.builtin
  (#match? @type.builtin "^(Op|Pattern|Pat|Intrinsic|Attr|AttrDef|TypeDef|Dialect|Interface|OpInterface|AttrInterface|TypeInterface|Constraint|Pred|Property)$"))

;; ODS def names that follow the "Op" / "Type" / "Attr" suffix convention
(def_definition (object_name (identifier) @type)
  (#match? @type "Op$|Type$|Attr$"))

;; Common ODS field names — Op / TypeDef / AttrDef / Dialect surface.
;; Split by rough category to keep the predicates readable.
(field_declaration (type) . (identifier) @property
  (#match? @property "^(arguments|results|regions|successors|parameters|traits|builders)$"))
(field_declaration (type) . (identifier) @property
  (#match? @property "^(summary|description|opName|mnemonic|cppNamespace|cppClassName|dependentDialects)$"))
(field_declaration (type) . (identifier) @property
  (#match? @property "^(assemblyFormat|extraClassDeclaration|extraClassDefinition)$"))
(field_declaration (type) . (identifier) @property
  (#match? @property "^(hasCustomAssemblyFormat|skipDefaultBuilders|hasVerifier|hasRegionVerifier|hasCanonicalizer|hasCanonicalizeMethod|hasFolder|hasOperandAccessFunctions)$"))
(field_declaration (type) . (identifier) @property
  (#match? @property "^(useCustomTypePrinterParser|useDefaultAttributePrinterParser|useDefaultTypePrinterParser)$"))
