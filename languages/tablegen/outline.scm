;; ---------------------------------------------------------------------------
;; TableGen outline (symbol navigation)
;;
;; Anonymous, unset, and empty record names are skipped because they do not
;; have a readable label to surface in Zed's outline. Computed names keep their
;; source expression as a stable navigation label.
;; ---------------------------------------------------------------------------

(class_definition
  "class" @context
  . (identifier) @name) @item

((def_definition
  "def" @context
  . (object_name) @name) @item
  (#not-eq? @name "?")
  (#not-eq? @name "\"\""))

((defm_definition
  "defm" @context
  . (object_name) @name) @item
  (#not-eq? @name "?")
  (#not-eq? @name "\"\""))

(multiclass_definition
  "multiclass" @context
  . (identifier) @name) @item

(defset_definition
  "defset" @context
  . (type)
  . (identifier) @name) @item

;; These declarations are most useful at file scope. Scoped defvars are
;; implementation details and would add noise to the outline.
(source_file
  (deftype_definition
    "deftype" @context
    . (identifier) @name) @item)

(source_file
  (defvar_statement
    "defvar" @context
    . (identifier) @name) @item)
