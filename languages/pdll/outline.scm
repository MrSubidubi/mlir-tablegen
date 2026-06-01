;; ---------------------------------------------------------------------------
;; PDLL outline (symbol navigation)
;; Anonymous declarations (e.g. `Pattern { ... }` with no name) are skipped
;; because they have no stable label to surface.
;; Named inline Constraint / Rewrite declarations remain navigation targets.
;; ---------------------------------------------------------------------------

(pattern_decl
  "Pattern" @context
  . (identifier) @name) @item

(constraint_decl
  "Constraint" @context
  . (identifier) @name) @item

(rewrite_decl
  "Rewrite" @context
  . (identifier) @name) @item
