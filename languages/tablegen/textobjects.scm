;; ---------------------------------------------------------------------------
;; TableGen text objects (Vim mode)
;;
;; class / multiclass are the section-shaped containers and own the class text
;; object; the records they contain own the function one, which keeps the two
;; on different nesting levels inside a multiclass.
;;
;; `defset`, `deftype`, and `defvar` are uncaptured — they bind a set, a type,
;; or a value rather than defining a record.
;;
;; Bodies use `_*`, not `(_)*` — a `(_)*` run stops at the anonymous
;; punctuation and would yield partial `inside` spans.
;; ---------------------------------------------------------------------------

(class_definition) @class.around

;; Separate from `around` because `body` is also satisfied by a bare `;`.
(class_definition
  (body
    "{"
    _* @class.inside
    "}"))

;; A multiclass carries its items directly rather than through `body`.
(multiclass_definition) @class.around

(multiclass_definition
  "{"
  _* @class.inside
  "}")

;; Named records. The predicates match `outline.scm`: `?` is the unset value
;; and `""` an empty computed name, so neither identifies a record.
((def_definition
  "def"
  . (object_name) @_name) @function.around
  (#not-eq? @_name "?")
  (#not-eq? @_name "\"\""))

((def_definition
  "def"
  . (object_name) @_name
  (body
    "{"
    _* @function.inside
    "}"))
  (#not-eq? @_name "?")
  (#not-eq? @_name "\"\""))

;; `defm` ends at `;` and has no body, so it contributes `around` only.
((defm_definition
  "defm"
  . (object_name) @_name) @function.around
  (#not-eq? @_name "?")
  (#not-eq? @_name "\"\""))

;; Line comments group into a block; `/* ... */` is already one node.
(line_comment)+ @comment.around

(block_comment) @comment.around
