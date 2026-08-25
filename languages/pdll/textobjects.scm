;; ---------------------------------------------------------------------------
;; PDLL text objects (Vim mode)
;;
;; Anchored to `source_file` like `outline.scm`: the grammar aliases inline
;; Constraint / Rewrite declarations to the same node types as the top-level
;; ones, so an unanchored pattern would select a nested helper instead.
;;
;; Bodies use `_*`, not `(_)*` — statement separators are anonymous direct
;; children, so a `(_)*` run stops at the first `;`.
;;
;; @class.* is unused: Pattern, Constraint, and Rewrite are all callable
;; declarations, and PDLL has no class-like construct.
;; ---------------------------------------------------------------------------

;; Named or anonymous — unlike the outline, a motion over an unnamed
;; `Pattern { ... }` is still useful.
(source_file
  (pattern_decl) @function.around)

(source_file
  (constraint_decl) @function.around)

(source_file
  (rewrite_decl) @function.around)

;; Compound bodies only; `=> expr;`, `[{ ... }];`, and `;` have no block.
(source_file
  (pattern_decl
    "{"
    _* @function.inside
    "}"))

(source_file
  (constraint_decl
    "{"
    _* @function.inside
    "}"))

(source_file
  (rewrite_decl
    "{"
    _* @function.inside
    "}"))

;; `comment` is an extra, so a run is selectable wherever it appears.
(comment)+ @comment.around
