;; ---------------------------------------------------------------------------
;; PDLL indentation rules
;;
;; PDLL has no dedicated "region" node — Pattern / Constraint / Rewrite bodies,
;; op attribute dicts, and rewrite-with blocks all use a plain `{ ... }`. One
;; generic rule per bracket pair covers every case.
;; ---------------------------------------------------------------------------

(_ "(" ")" @end) @indent
(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
