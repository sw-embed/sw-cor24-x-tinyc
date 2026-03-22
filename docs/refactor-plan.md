# cc24 Architecture Refactor Plan

## Design Philosophy

The compiler is a **pipeline of configured, composable stages**. Each
stage is independently testable, replaceable, and extensible. Features
are not locations in code -- they are **handlers that plug into
dispatch chains**. Adding, reimplementing, or removing a feature should
not require modifying other features.

The architecture has these **top-down roles**:

1. **Configuration** -- what the compiler should do (data, no behavior)
2. **Building** -- construct the configured pipeline (wiring, no execution)
3. **Dispatch** -- route work to the right handler (delegation, no logic)
4. **Functional** -- do the actual work (pure transforms, stateless where possible)
5. **Testing** -- verify everything works (shared helpers, fixtures, fakes)

## Architectural Components

Every component below represents an **architectural role**, not a
grouping of existing code. Components are sparse by design -- many small
parts are better than few large ones.

### config/ -- Configuration (data only)

Defines what the compiler does. No behavior, no I/O.

```
config/
  crates/
    cc24-config/          -- CompilerConfig struct: source path, output path,
                             include dirs, target, optimization level, flags
    cc24-target/          -- TargetConfig: COR24-TB register addresses,
                             word size, stack init, ISR conventions
```

A CompilerConfig is a plain data struct. It does not know how to parse
CLI args or read files. It is constructed by the builder.

### builder/ -- Pipeline Construction (wiring only)

Builds the configured pipeline from CLI args, env vars, and defaults.
Registers handlers. Instantiates stages. Produces a ready-to-run pipeline.

```
builder/
  crates/
    cc24-cli-args/        -- parse CLI args into raw options
    cc24-pipeline-builder/-- build Pipeline from CompilerConfig:
                             register stmt/expr/op handlers,
                             configure preprocessor include paths,
                             set target, wire stages together
```

The builder is the only place that knows about all the handler crates.
It imports them and registers them. The dispatch component does not
know what handlers exist.

### dispatch/ -- Chain-of-Responsibility (delegation only)

Generic dispatch infrastructure. Knows nothing about C, assembly, or
COR24. Pure delegation pattern.

```
dispatch/
  crates/
    cc24-dispatch/        -- Handler<Input,State> trait, Chain<H> struct,
                             ChainBuilder, registration API
```

```rust
pub trait Handler<Input, State> {
    fn can_handle(&self, input: &Input) -> bool;
    fn handle(&self, input: &Input, state: &mut State);
}

pub struct Chain<H> {
    handlers: Vec<Box<H>>,
}
```

Statement dispatch, expression dispatch, operator dispatch, and
preprocessor directive dispatch all use the same Chain infrastructure
with different Input/State type parameters.

### core/ -- Shared Types and Traits (no behavior)

Data types and trait definitions. No implementations, no logic.

```
core/
  crates/
    cc24-span/            -- Span (unchanged)
    cc24-error/           -- CompileError (unchanged)
    cc24-token/           -- Token, TokenKind (unchanged)
    cc24-ast/             -- AST nodes: Program, Function, Stmt, Expr
    cc24-types/           -- Type enum (Char, Int, Ptr, Array, Void)
    cc24-traits/          -- Emitter trait, TypeQuery trait,
                             StmtHandler, ExprHandler, OpHandler
```

cc24-traits defines the interfaces. Implementations live elsewhere.

### codegen-state/ -- Codegen State (struct only)

The mutable state threaded through code generation. No methods.

```
codegen-state/
  crates/
    cc24-codegen-state/   -- CodegenState struct (pub fields):
                             output buffer, label counter, locals map,
                             local_types map, globals, break/continue
                             stacks, string literals, runtime flags
```

This crate has zero behavior. It is a plain data carrier.

### codegen-emit/ -- Low-Level Emission (mutating, target-specific)

Functions that write to the output buffer. Know about COR24 assembly
syntax but not about C language constructs.

```
codegen-emit/
  crates/
    cc24-emit-core/       -- emit(), new_label(), load_immediate()
    cc24-emit-data/       -- emit_data_section(), emit_start()
    cc24-emit-load-store/ -- gen_load_by_name(), gen_store_by_name(),
                             load/store for globals vs locals, char vs int
```

### codegen-query/ -- Read-Only Analysis (non-mutating)

Pure functions that inspect AST nodes and state without modifying anything.

```
codegen-query/
  crates/
    cc24-type-infer/      -- expr_type(): infer type of expression from state
    cc24-type-query/      -- is_char_ptr(), pointee_type(), element_size()
```

### codegen-handlers/ -- Feature Handlers (one per feature)

Each handler implements a trait from cc24-traits. Each handler is a
small crate with 1-3 functions. Adding a new C feature = adding a new
handler crate and registering it in the builder.

```
codegen-handlers/
  crates/
    cc24-handle-return/       -- Stmt::Return handler
    cc24-handle-if/           -- Stmt::If handler
    cc24-handle-while/        -- Stmt::While handler
    cc24-handle-dowhile/      -- Stmt::DoWhile handler
    cc24-handle-for/          -- Stmt::For handler
    cc24-handle-break/        -- Stmt::Break handler
    cc24-handle-continue/     -- Stmt::Continue handler
    cc24-handle-local-decl/   -- Stmt::LocalDecl handler
    cc24-handle-asm/          -- Stmt::Asm handler
    cc24-handle-expr-stmt/    -- Stmt::Expr handler

    cc24-handle-intlit/       -- Expr::IntLit handler
    cc24-handle-stringlit/    -- Expr::StringLit handler
    cc24-handle-ident/        -- Expr::Ident (variable load + array decay)
    cc24-handle-assign/       -- Expr::Assign handler
    cc24-handle-call/         -- Expr::Call handler
    cc24-handle-addr-of/      -- Expr::AddrOf handler
    cc24-handle-deref/        -- Expr::Deref + DerefAssign handler
    cc24-handle-cast/         -- Expr::Cast handler
    cc24-handle-incdec/       -- PreInc, PreDec, PostInc, PostDec handler
    cc24-handle-unary/        -- UnaryOp (neg, bitnot, lognot)

    cc24-handle-add-sub/      -- BinOp::Add, Sub (with pointer arithmetic)
    cc24-handle-mul/          -- BinOp::Mul
    cc24-handle-div-mod/      -- BinOp::Div, Mod (software divide)
    cc24-handle-bitwise/      -- BinOp::BitAnd, BitOr, BitXor, Shl, Shr
    cc24-handle-compare/      -- BinOp::Eq, Ne, Lt, Gt, Le, Ge
    cc24-handle-logical/      -- BinOp::LogAnd, LogOr (short-circuit)
```

### codegen-runtime/ -- Runtime Library Templates

Assembly templates emitted when certain features are used.

```
codegen-runtime/
  crates/
    cc24-runtime-divmod/  -- __cc24_div, __cc24_mod assembly
    cc24-runtime-isr/     -- ISR prologue/epilogue templates
    cc24-runtime-start/   -- _start entry point template
```

### codegen/ -- Orchestrator (wiring only)

Thin orchestrator that owns the pipeline and drives it. Calls the builder,
then runs dispatch chains over the AST.

```
codegen/
  crates/
    cc24-codegen/         -- generate(): build state, walk AST,
                             dispatch each node, emit runtime, emit data
    cc24-func-codegen/    -- gen_function(): prologue, walk body, epilogue
    cc24-locals-collect/  -- pre-pass to collect local variable declarations
```

### preprocess/ -- Preprocessor

```
preprocess/
  crates/
    cc24-preprocess/
    cc24-preprocess-tests/
```

### lexer/ -- Tokenization

```
lexer/
  crates/
    cc24-lexer/
    cc24-lexer-tests/
```

### parser/ -- Parsing

```
parser/
  crates/
    cc24-parse-stream/
    cc24-parser/
    cc24-parser-tests/
```

### testing/ -- Test Infrastructure

Shared test helpers, fixtures, and validation. Every test crate depends
on this instead of duplicating compile/assert helpers.

```
testing/
  crates/
    cc24-test-compile/    -- compile(): source -> assembly string
    cc24-test-golden/     -- golden_test(): compare against .expected.s
    cc24-test-cor24/      -- cor24-run path lookup, assemble_with_cor24_run()
    cc24-test-as24/       -- as24 HTTP client, assert_assembles_http()
    cc24-test-fixtures/   -- fixture path resolution, fixture management
```

### cli/ -- Entry Point (thin)

```
cli/
  crates/
    cc24/                 -- main(): parse args -> build config -> build
                             pipeline -> read source -> run pipeline ->
                             write output
```

## Feature Lifecycle

Adding a new feature (e.g., switch/case):

1. Add `Stmt::Switch` to cc24-ast (core)
2. Add `Switch` token to cc24-token (core)
3. Add keyword to cc24-lexer (lexer)
4. Add parsing to cc24-parser (parser)
5. Create `cc24-handle-switch` crate (codegen-handlers)
6. Register handler in cc24-pipeline-builder (builder)
7. Create test in testing component
8. Create demo

Steps 1-4 touch existing crates minimally (add a variant, add a match arm).
Step 5 is a new crate -- no existing code modified.
Step 6 is one line: `chain.register(SwitchHandler);`

Removing a feature: delete the handler crate, remove registration.
Reimplementing: replace the handler crate, same interface.

## Phased Execution

### Phase 1: Infrastructure (config, builder, dispatch, testing)

Create the 5 architectural role components with minimal implementations.
Move existing compile/test helpers into testing component. Wire the
existing monolithic codegen through the new dispatch infrastructure.
All existing tests must still pass.

### Phase 2: Extract handlers from monolithic codegen

One handler at a time, extract from cc24-codegen into a handler crate.
Each extraction is a small commit. The monolithic codegen shrinks as
handlers are extracted. Eventually the monolithic crate is empty and
deleted.

### Phase 3: Extract query and emit

Move type inference and read-only queries into codegen-query.
Move emit functions into codegen-emit.
Move runtime templates into codegen-runtime.

### Phase 4: Extract parser and lexer

Move preprocess, lexer, parser into their own components.
Remove the frontend component.

## Invariants

- Every commit: all tests pass, zero sw-checklist failures
- Every handler crate: 1-3 functions, 1-2 modules
- Every component: 2-4 crates
- No circular dependencies between components
- Config has no dependencies on any other component
- Dispatch has no dependencies on handlers
- Handlers depend on: core, codegen-state, codegen-emit, codegen-query
- Builder depends on: config, dispatch, all handler crates
- CLI depends on: config, builder, and the pipeline result
