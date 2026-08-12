;; ---------------------------------------------------------------------------
;; PDLL syntax highlighting
;; ---------------------------------------------------------------------------

;; Comments
(comment) @comment

;; Strings and embedded C++ code block (`[{ ... }]`)
(string) @string
(escape_sequence) @string.escape
(code_block) @string.special

;; Numbers
(integer) @number

;; Include directive
(include_directive "#include" @keyword)

;; Top-level declaration keywords
[
  "Pattern"
  "Constraint"
  "Rewrite"
] @keyword

;; Statement / control keywords
[
  "let"
  "erase"
  "replace"
  "rewrite"
  "return"
  "with"
  "not"
] @keyword

;; Pattern metadata (Pattern Foo with benefit(1), recursion { ... })
(benefit_metadata "benefit" @keyword)
(recursion_metadata) @keyword

;; op<...> / attr<"..."> / type<"..."> expression keywords
(op_expr "op" @keyword)
(attr_expr "attr" @keyword)
(type_expr "type" @keyword)

;; Identifier fallback — keep BEFORE the specific identifier rules below.
;; Zed's tree-sitter highlighter uses last-match-wins: later captures for the
;; same identifier node override this fallback.
(identifier) @variable
(variable_def (identifier) @variable)
(call_expr (identifier) @function)

;; Built-in type constraints
(type_constraint
  ["Op" "Attr" "Type" "Value" "ValueRange" "TypeRange"] @type.builtin)

;; User-defined type constraints in type position
(type_constraint
  (identifier) @type)

;; Parameter names (includes contextual keywords aliased to identifier)
(argument (identifier) @variable.parameter)
(named_type_constraint (identifier) @variable.parameter)

;; Operation attribute keys
(op_attribute
  [(identifier) (string)] @attribute)

;; Declaration names
(pattern_decl . (identifier) @function)
(rewrite_decl . (identifier) @function)
(constraint_decl . (identifier) @type)

;; Negated native constraint calls: `not Constraint(...)`
(negated_call_expr
  "not" @keyword
  (identifier) @function)

;; Op names inside op<ns.name> / Op<ns.name>. The parser now wraps these in
;; `op_name` so dotted names do not get mistaken for member access.
(op_name (identifier) @constant)
(op_name (string) @constant)

;; Brackets
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<"
  ">"
] @punctuation.bracket

;; Delimiters
[
  ","
  ":"
  ";"
  "."
] @punctuation.delimiter

;; Operators
[
  "="
  "=>"
  "->"
] @operator

;; The `.` in op<ns.name> separates two halves of one name rather than acting
;; as member access, so it takes the surrounding @constant and the dotted name
;; renders as a single unit. Must come AFTER the delimiter list above.
(op_name "." @constant)
