;; ---------------------------------------------------------------------------
;; PDLL outline (symbol navigation)
;; The LLVM PDLL parser distinguishes top-level declarations from inline
;; Constraint / Rewrite declarations, which are local lambda-like helpers.
;; Only named top-level declarations are surfaced here; anonymous Patterns and
;; inline Constraint / Rewrite declarations are skipped to keep the outline
;; focused on stable file-level symbols.
;; ---------------------------------------------------------------------------

(source_file
  (pattern_decl
    "Pattern" @context
    . (identifier) @name) @item)

(source_file
  (constraint_decl
    "Constraint" @context
    . (identifier) @name) @item)

(source_file
  (rewrite_decl
    "Rewrite" @context
    . (identifier) @name) @item)
