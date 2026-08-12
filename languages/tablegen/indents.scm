;; ---------------------------------------------------------------------------
;; TableGen indentation rules
;;
;; TableGen's block-like constructs (class / multiclass / def / defset / let /
;; foreach / if bodies) all put the `{ ... }` inside helper nodes such as
;; `body`, `multiclass_body`, `block`, or `statement`. Matching generically on
;; the bracket pair is more robust than enumerating every node shape.
;; ---------------------------------------------------------------------------

(_ "(" ")" @end) @indent
(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent

;; `<>` indent is restricted to the constructs whose argument lists routinely
;; wrap across lines. Type-level `<>` (`bits<8>`, `list<int>`) and bang-op
;; type sugar (`!cast<i32>(x)`) are conventionally single-line and would only
;; suffer over-indentation under a generic `(_ "<" ">")` rule.
(template_parameters "<" ">" @end) @indent
(parent_class "<" ">" @end) @indent
(anonymous_record "<" ">" @end) @indent
