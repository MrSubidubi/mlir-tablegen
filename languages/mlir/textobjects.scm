;; ---------------------------------------------------------------------------
;; MLIR text objects (Vim mode)
;;
;; Regions and blocks are nested operation bodies rather than file sections, so
;; only functions and modules are captured; classifying every region as a class
;; would make `ac` select an arbitrary nesting level.
;;
;; Bodies use `_*` for consistency with the PDLL and TableGen queries, where
;; `(_)*` would stop at the anonymous statement separators.
;; ---------------------------------------------------------------------------

;; func.func and llvm.func: the operations with a modeled signature and body.
(func_operation) @function.around

;; Separate from `around` because the body is optional and may be empty.
(func_operation
  body: (region
    "{"
    _* @function.inside
    "}"))

;; Modules are the only MLIR construct that reads as a file-level section.
(module_operation) @class.around

(module_operation
  body: (region
    "{"
    _* @class.inside
    "}"))

;; `comment` is an extra, so a run is selectable wherever it appears.
(comment)+ @comment.around
