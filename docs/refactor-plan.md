# cc24 Architecture Refactor Plan

## Problem

The codegen crate has 7 modules, each at 6-8 functions. Every new feature
(sizeof, switch/case, struct, etc.) forces function count violations or
hacks. The architecture is monolithic -- structs, impls, logic, and test
helpers are co-located. This makes it impossible to grow without constant
reshuffling.

## Current State

```
4 components, 19 crates, 51 modules, ~205 functions
```

The worst offender is `cc24-codegen` (7 modules, 45+ functions). The parser
is also near limits. Test helpers are duplicated across test crates.

## Design Principles

1. **Separate structs from impls** -- data types in one crate, behavior in others
2. **Separate traits from implementations** -- composable interfaces
3. **Separate read-only from mutating operations** -- different crates
4. **Chain-of-responsibility for dispatch** -- extensible operator/statement handlers
5. **Configuration separate from execution** -- builder pattern for setup
6. **Test helpers as their own crate** -- DI, fixtures, fakes

## Target Architecture

### Components (top-level directories)

```
components/
  core/           -- shared types, traits, errors (no behavior)
  config/         -- CLI args, paths, compiler configuration
  preprocess/     -- preprocessor (#define, #include)
  lexer/          -- tokenization
  parser/         -- parsing to AST
  codegen/        -- assembly generation (the big split)
  runtime/        -- runtime helpers (divmod, ISR templates)
  dispatch/       -- chain-of-responsibility handlers
  testing/        -- test helpers, fixtures, compile helper
  cli/            -- binary entry point (thin)
```

### Core Component (types only, no behavior)

```
core/
  crates/
    cc24-span/        -- Span type (unchanged)
    cc24-error/       -- CompileError (unchanged)
    cc24-token/       -- Token, TokenKind (unchanged)
    cc24-ast/         -- AST nodes (unchanged)
    cc24-types/       -- Type enum, type queries (size, element_type)
    cc24-traits/      -- Emitter, Handler, Visitor traits
```

### Codegen Split (biggest change)

Currently one crate with 7 crowded modules. Split into:

```
codegen/
  crates/
    cc24-codegen-state/   -- Codegen struct definition (fields only, no impls)
    cc24-codegen-emit/    -- emit(), new_label(), load_immediate(), start, data section
    cc24-codegen-func/    -- function prologue/epilogue, locals collection
    cc24-codegen-stmt/    -- statement handlers (each stmt type = 1 handler)
    cc24-codegen-expr/    -- expression handlers
    cc24-codegen-ops/     -- binary/unary operator handlers
    cc24-codegen/         -- orchestrator: generate() wires everything together
```

Each handler crate has free functions taking `&mut CodegenState`. No impl
blocks on CodegenState outside its own crate.

### Dispatch Component

Chain-of-responsibility for extensible dispatch:

```
dispatch/
  crates/
    cc24-dispatch/        -- Handler trait, Chain struct, registration
    cc24-stmt-handlers/   -- one handler per statement type
    cc24-expr-handlers/   -- one handler per expression type
    cc24-op-handlers/     -- one handler per operator
```

A handler is:
```rust
pub trait StmtHandler {
    fn can_handle(&self, stmt: &Stmt) -> bool;
    fn handle(&self, state: &mut CodegenState, stmt: &Stmt);
}
```

The chain tries handlers in order. New features add a handler without
modifying existing code (open/closed principle).

### Testing Component

```
testing/
  crates/
    cc24-test-helpers/    -- compile(), golden_test(), assert_assembles()
    cc24-test-fixtures/   -- fixture file management
```

Eliminates duplication between codegen-tests, codegen-validate, and
future test crates.

### Config Component

```
config/
  crates/
    cc24-config/          -- CompilerConfig struct (paths, flags, options)
    cc24-config-builder/  -- build config from CLI args, env, defaults
```

### Preprocess Component (extracted from frontend)

```
preprocess/
  crates/
    cc24-preprocess/      -- preprocessor logic (unchanged, just moved)
    cc24-preprocess-tests/
```

### Lexer Component (extracted from frontend)

```
lexer/
  crates/
    cc24-lexer/           -- lexer logic
    cc24-lexer-tests/
```

### Parser Component (extracted from frontend)

```
parser/
  crates/
    cc24-parse-stream/    -- TokenStream + attrs
    cc24-parser/          -- parsing logic
    cc24-parser-tests/
```

## Phased Execution

### Phase 1: Extract types and traits
- Create `cc24-types` crate (move Type enum from cc24-ast)
- Create `cc24-traits` crate (define Emitter, StmtHandler, ExprHandler traits)
- Update all downstream imports

### Phase 2: Split codegen state from behavior
- Create `cc24-codegen-state` with the Codegen struct (pub fields)
- Move emit helpers to `cc24-codegen-emit`
- Move func logic to `cc24-codegen-func`
- Move expression codegen to `cc24-codegen-expr`
- Move statement codegen to `cc24-codegen-stmt`
- Move operator codegen to `cc24-codegen-ops`
- Orchestrator in `cc24-codegen`

### Phase 3: Add dispatch
- Create `cc24-dispatch` with Handler trait and Chain
- Convert stmt/expr/op codegen to handlers
- Register handlers in orchestrator

### Phase 4: Extract components
- Move preprocess out of frontend to own component
- Move lexer out of frontend to own component
- Create testing component
- Create config component

### Phase 5: Separate read-only from mutating
- Read-only queries (expr_type, is_char_ptr) in their own crate
- Mutating operations (gen_*, emit_*) in their own crates

## Benefits

- Every crate stays well under limits (2-4 modules, 2-4 fns each)
- New features add handlers without modifying existing code
- Test helpers shared across all test crates
- Configuration is explicit and testable
- Each component is independently buildable and testable
