;; ---------------------------------------------------------------------------
;; TableGen language injections
;;
;; TableGen `[{ ... }]` code blocks are injected as C++ — but ONLY when the
;; enclosing field is unambiguously C++ source, not arbitrary text. Editors
;; should provide tree-sitter-cpp as a sibling parser.
;;
;; Per the LLVM TableGen ProgRef, `code` is an alias for `string`; on the
;; parser side they produce indistinguishable values. The two cases below
;; therefore key on different signals and cannot be collapsed:
;;   Case 1 — declared type is literally `code`, so any [{ ... }] is C++.
;;   Case 2 — declared type is `string` (or anything else), so we instead
;;            gate on a known C++-carrying ODS field name.
;; Counter-example deliberately NOT injected: `string assemblyFormat = "..."`
;; is the op assembly DSL, not C++.
;; ---------------------------------------------------------------------------

;; Case 1: field declared with `code` type — always C++.
;;   code extraClassDeclaration = [{ ... }];
(field_declaration
  (type) @_type
  (code_literal (code_chunk) @injection.content)
  (#eq? @_type "code")
  (#set! injection.language "cpp")
  (#set! injection.combined))

;; Case 2: field declaration (any type, in practice `string`) whose name is
;; a known C++-carrying ODS key.
;;   string builders = [{ ... }];
(field_declaration
  (identifier) @_name
  (code_literal (code_chunk) @injection.content)
  (#match? @_name
    "^(extraClassDeclaration|extraClassDefinition|extraTraitClassDeclaration|extraSharedClassDeclaration|builders|verify)$")
  (#set! injection.language "cpp")
  (#set! injection.combined))

;; Case 3: standalone let-assignment with a known C++-carrying ODS key.
;;   let builders = [{ ... }];
(let_assignment
  (identifier) @_name
  (code_literal (code_chunk) @injection.content)
  (#match? @_name
    "^(extraClassDeclaration|extraClassDefinition|extraTraitClassDeclaration|extraSharedClassDeclaration|builders|verify)$")
  (#set! injection.language "cpp")
  (#set! injection.combined))

;; Case 4: let-item inside a comma-separated `let ... in { }` block.
;;   let builders = [{ ... }],
;;       verify = [{ ... }] in { ... }
(let_item
  (identifier) @_name
  (code_literal (code_chunk) @injection.content)
  (#match? @_name
    "^(extraClassDeclaration|extraClassDefinition|extraTraitClassDeclaration|extraSharedClassDeclaration|builders|verify)$")
  (#set! injection.language "cpp")
  (#set! injection.combined))
