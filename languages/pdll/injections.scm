;; ---------------------------------------------------------------------------
;; PDLL language injections
;;
;; PDLL's `[{ ... }]` blocks inside Constraint / Rewrite / native expressions
;; carry C++ source that mlir-pdll emits verbatim, so their highlighting is
;; delegated to the C++ grammar. Editors should provide tree-sitter-cpp as a
;; sibling parser.
;; ---------------------------------------------------------------------------

;; Inject only the inner `code_block_content` node so the `[{` and `}]`
;; delimiters stay under PDLL highlighting.
((code_block
  (code_block_content) @injection.content)
  (#set! injection.language "cpp"))
