# Tasks: Proc-Macro Workshop Implementations

**Input**: Design documents from `/specs/001-proc-macro-implementations/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Tests are PRE-WRITTEN by the workshop. Implementation follows test-driven approach: uncomment tests in `progress.rs` incrementally and implement until each passes.

**Organization**: Tasks are grouped by user story (each macro project). Projects can be implemented sequentially (recommended for learning) or in parallel.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files/projects, no dependencies)
- **[Story]**: Which user story/project this task belongs to (US1=builder, US2=debug, US3=seq, US4=sorted, US5=bitfield)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Configure dependencies for all macro crates

- [ ] T001 Add syn, quote, proc-macro2 dependencies to `builder/Cargo.toml`
- [ ] T002 [P] Add syn, quote, proc-macro2 dependencies to `debug/Cargo.toml`
- [ ] T003 [P] Add syn, quote, proc-macro2 dependencies to `seq/Cargo.toml`
- [ ] T004 [P] Add syn, quote, proc-macro2, visit-mut dependencies to `sorted/Cargo.toml`
- [ ] T005 [P] Add syn, quote, proc-macro2 dependencies to `bitfield/impl/Cargo.toml`
- [ ] T006 Add darling dependency to `builder/Cargo.toml` for attribute parsing
- [ ] T007 [P] Add darling dependency to `debug/Cargo.toml` for attribute parsing
- [ ] T008 [P] Add proc-macro-error2 dependency to `sorted/Cargo.toml` for multi-error handling
- [ ] T009 [P] Add darling and proc-macro-error2 dependencies to `bitfield/impl/Cargo.toml`

**Checkpoint**: All projects have required dependencies configured

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Verify workshop test harness works and understand test structure

**⚠️ CRITICAL**: Verify each project compiles before implementing

- [ ] T010 Verify `cargo check` passes in `builder/` with empty macro stub
- [ ] T011 [P] Verify `cargo check` passes in `debug/` with empty macro stub
- [ ] T012 [P] Verify `cargo check` passes in `seq/` with empty macro stub
- [ ] T013 [P] Verify `cargo check` passes in `sorted/` with empty macro stub
- [ ] T014 [P] Verify `cargo check` passes in `bitfield/` with empty macro stub

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Builder Macro (Priority: P1) 🎯 MVP

**Goal**: Implement `#[derive(Builder)]` that generates builder pattern boilerplate

**Independent Test**: `cd builder && cargo test` with all tests in `progress.rs` uncommented

### Implementation for Builder (9 tests)

- [ ] T015 [US1] Implement basic parsing in `builder/src/lib.rs` (test 01-parse.rs)
- [ ] T016 [US1] Generate Builder struct with Option-wrapped fields in `builder/src/lib.rs` (test 02-create-builder.rs)
- [ ] T017 [US1] Generate setter methods returning `&mut Self` in `builder/src/lib.rs` (test 03-call-setters.rs)
- [ ] T018 [US1] Implement `build()` method with required field validation in `builder/src/lib.rs` (test 04-call-build.rs)
- [ ] T019 [US1] Ensure method chaining works with `&mut self` pattern in `builder/src/lib.rs` (test 05-method-chaining.rs)
- [ ] T020 [US1] Handle `Option<T>` fields as optional (no error if unset) in `builder/src/lib.rs` (test 06-optional-field.rs)
- [ ] T021 [US1] Implement `#[builder(each = "...")]` for Vec fields using darling in `builder/src/lib.rs` (test 07-repeated-field.rs)
- [ ] T022 [US1] Add error handling for unrecognized attributes with proper spans in `builder/src/lib.rs` (test 08-unrecognized-attribute.rs)
- [ ] T023 [US1] Use fully-qualified paths (::std::option::Option) in `builder/src/lib.rs` (test 09-redefined-prelude-types.rs)
- [ ] T024 [US1] Uncomment all tests in `builder/tests/progress.rs` and verify all pass
- [ ] T025 [US1] Run `cargo clippy` in `builder/` and fix any warnings

**Checkpoint**: Builder macro complete — `cargo test` passes in `builder/`

---

## Phase 4: User Story 2 - CustomDebug Macro (Priority: P2)

**Goal**: Implement `#[derive(CustomDebug)]` with custom format strings and smart trait bound inference

**Independent Test**: `cd debug && cargo test` with all tests in `progress.rs` uncommented

### Implementation for CustomDebug (8 tests)

- [ ] T026 [US2] Implement basic parsing in `debug/src/lib.rs` (test 01-parse.rs)
- [ ] T027 [US2] Generate basic Debug impl with debug_struct in `debug/src/lib.rs` (test 02-impl-debug.rs)
- [ ] T028 [US2] Parse `#[debug = "format"]` attribute and apply to fields in `debug/src/lib.rs` (test 03-custom-format.rs)
- [ ] T029 [US2] Implement trait bound inference for generic type parameters in `debug/src/lib.rs` (test 04-type-parameter.rs)
- [ ] T030 [US2] Skip PhantomData fields from trait bound inference in `debug/src/lib.rs` (test 05-phantom-data.rs)
- [ ] T031 [US2] Handle associated type bounds correctly in `debug/src/lib.rs` (test 06-bound-trouble.rs)
- [ ] T032 [US2] Continue associated type handling in `debug/src/lib.rs` (test 07-associated-type.rs)
- [ ] T033 [US2] Implement `#[debug(bound = "...")]` escape hatch in `debug/src/lib.rs` (test 08-escape-hatch.rs)
- [ ] T034 [US2] Uncomment all tests in `debug/tests/progress.rs` and verify all pass
- [ ] T035 [US2] Run `cargo clippy` in `debug/` and fix any warnings

**Checkpoint**: CustomDebug macro complete — `cargo test` passes in `debug/`

---

## Phase 5: User Story 3 - Seq Macro (Priority: P3)

**Goal**: Implement `seq!` function-like macro for repetitive code generation with identifier pasting

**Independent Test**: `cd seq && cargo test` with all tests in `progress.rs` uncommented

### Implementation for Seq (9 tests)

- [ ] T036 [US3] Parse `N in 0..8` header syntax in `seq/src/lib.rs` (test 01-parse-header.rs)
- [ ] T037 [US3] Parse body tokens and return them in `seq/src/lib.rs` (test 02-parse-body.rs)
- [ ] T038 [US3] Handle error spans correctly for invalid input in `seq/src/lib.rs` (test 03-expand-four-errors.rs)
- [ ] T039 [US3] Implement `~N` identifier pasting in `seq/src/lib.rs` (test 04-paste-ident.rs)
- [ ] T040 [US3] Implement `#(...)*` repeat sections in `seq/src/lib.rs` (test 05-repeat-section.rs)
- [ ] T041 [US3] Support array initialization use case in `seq/src/lib.rs` (test 06-init-array.rs)
- [ ] T042 [US3] Support inclusive ranges `0..=N` in `seq/src/lib.rs` (test 07-inclusive-range.rs)
- [ ] T043 [US3] Ensure correct spans for pasted identifiers in `seq/src/lib.rs` (test 08-ident-span.rs)
- [ ] T044 [US3] Handle interaction with macro_rules! in `seq/src/lib.rs` (test 09-interaction-with-macrorules.rs)
- [ ] T045 [US3] Uncomment all tests in `seq/tests/progress.rs` and verify all pass
- [ ] T046 [US3] Run `cargo clippy` in `seq/` and fix any warnings

**Checkpoint**: Seq macro complete — `cargo test` passes in `seq/`

---

## Phase 6: User Story 4 - Sorted Macro (Priority: P4)

**Goal**: Implement `#[sorted]` attribute macro for compile-time ordering validation

**Independent Test**: `cd sorted && cargo test` with all tests in `progress.rs` uncommented

### Implementation for Sorted (8 tests)

- [ ] T047 [US4] Parse enum and return unchanged in `sorted/src/lib.rs` (test 01-parse-enum.rs)
- [ ] T048 [US4] Reject non-enum items with appropriate error in `sorted/src/lib.rs` (test 02-not-enum.rs)
- [ ] T049 [US4] Detect out-of-order variants and emit precise errors in `sorted/src/lib.rs` (test 03-out-of-order.rs)
- [ ] T050 [US4] Handle variants with associated data in `sorted/src/lib.rs` (test 04-variants-with-data.rs)
- [ ] T051 [US4] Implement `#[sorted::check]` using VisitMut pattern in `sorted/src/lib.rs` (test 05-match-expr.rs)
- [ ] T052 [US4] Extract variant names from path patterns (Error::Io) in `sorted/src/lib.rs` (test 06-pattern-path.rs)
- [ ] T053 [US4] Handle unrecognized patterns with clear errors in `sorted/src/lib.rs` (test 07-unrecognized-pattern.rs)
- [ ] T054 [US4] Handle underscore wildcard pattern correctly in `sorted/src/lib.rs` (test 08-underscore.rs)
- [ ] T055 [US4] Uncomment all tests in `sorted/tests/progress.rs` and verify all pass
- [ ] T056 [US4] Run `cargo clippy` in `sorted/` and fix any warnings

**Checkpoint**: Sorted macro complete — `cargo test` passes in `sorted/`

---

## Phase 7: User Story 5 - Bitfield Macro (Priority: P5)

**Goal**: Implement `#[bitfield]` attribute macro and `BitfieldSpecifier` derive for packed bit structures

**Independent Test**: `cd bitfield && cargo test` with all tests in `progress.rs` uncommented

### Implementation for Bitfield (12 tests)

- [ ] T057 [US5] Create Specifier trait and B1-B64 types in `bitfield/src/lib.rs` (test 01-specifier-types.rs)
- [ ] T058 [US5] Implement struct storage transformation (byte array) in `bitfield/impl/src/lib.rs` (test 02-storage.rs)
- [ ] T059 [US5] Generate get_* and set_* accessor methods in `bitfield/impl/src/lib.rs` (test 03-accessors.rs)
- [ ] T060 [US5] Validate total bits is multiple of 8 at compile time in `bitfield/impl/src/lib.rs` (test 04-multiple-of-8bits.rs)
- [ ] T061 [US5] Return correct integer types (u8/u16/u32/u64) from accessors in `bitfield/impl/src/lib.rs` (test 05-accessor-signatures.rs)
- [ ] T062 [US5] Implement BitfieldSpecifier derive for enums in `bitfield/impl/src/lib.rs` (test 06-enums.rs)
- [ ] T063 [US5] Handle enums without explicit discriminants in `bitfield/impl/src/lib.rs` (test 07-optional-discriminant.rs)
- [ ] T064 [US5] Validate enum variant count is power of two in `bitfield/impl/src/lib.rs` (test 08-non-power-of-two.rs)
- [ ] T065 [US5] Validate discriminant values fit in bit width in `bitfield/impl/src/lib.rs` (test 09-variant-out-of-range.rs)
- [ ] T066 [US5] Implement `#[bits = N]` attribute for explicit width in `bitfield/impl/src/lib.rs` (test 10-bits-attribute.rs)
- [ ] T067 [US5] Validate `#[bits = N]` matches actual specifier width in `bitfield/impl/src/lib.rs` (test 11-bits-attribute-wrong.rs)
- [ ] T068 [US5] Handle edge cases in bit accessors (spanning bytes) in `bitfield/impl/src/lib.rs` (test 12-accessors-edge.rs)
- [ ] T069 [US5] Uncomment all tests in `bitfield/tests/progress.rs` and verify all pass
- [ ] T070 [US5] Run `cargo clippy` in `bitfield/` and fix any warnings

**Checkpoint**: Bitfield macro complete — `cargo test` passes in `bitfield/`

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final validation across all projects

- [ ] T071 Run `cargo test --workspace` from repository root
- [ ] T072 [P] Run `cargo clippy --all-targets` in each project directory
- [ ] T073 [P] Run `cargo doc` in each project and verify no warnings
- [ ] T074 [P] Add doc comments to public macro entry points in all `src/lib.rs` files
- [ ] T075 Verify `cargo expand` output is readable for sample inputs in `main.rs`
- [ ] T076 Final review: ensure all `.stderr` files match expected error output

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup (T001-T009)
    │
    ▼
Phase 2: Foundational (T010-T014)
    │
    ▼
┌───┴───┬───────┬───────┬───────┐
▼       ▼       ▼       ▼       ▼
Phase 3 Phase 4 Phase 5 Phase 6 Phase 7
(US1)   (US2)   (US3)   (US4)   (US5)
Builder Debug   Seq     Sorted  Bitfield
    │       │       │       │       │
    └───────┴───────┴───────┴───────┘
                    │
                    ▼
            Phase 8: Polish (T071-T076)
```

### User Story Dependencies

- **US1 (Builder)**: Foundation only — no dependencies on other stories (RECOMMENDED FIRST)
- **US2 (Debug)**: Foundation only — builds on concepts from US1 but code-independent
- **US3 (Seq)**: Foundation only — completely different approach (function-like macro)
- **US4 (Sorted)**: Foundation only — different skill (validation vs generation)
- **US5 (Bitfield)**: Foundation only — most complex, benefits from all prior experience

### Recommended Learning Path

```
Builder (P1) → Debug (P2) → Seq (P3) → Sorted (P4) → Bitfield (P5)
```

This order matches workshop recommendations: each project introduces new concepts that build on previous experience.

### Parallel Opportunities

**Setup Phase (T001-T009):**
```bash
# All dependency additions can run in parallel:
T001, T002, T003, T004, T005  # Core deps (different Cargo.toml files)
T006, T007, T008, T009        # Additional deps (different files)
```

**User Stories (once Foundational complete):**
```bash
# All 5 projects can be worked in parallel if desired:
Phase 3: Builder (T015-T025)
Phase 4: Debug (T026-T035)
Phase 5: Seq (T036-T046)
Phase 6: Sorted (T047-T056)
Phase 7: Bitfield (T057-T070)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (all deps configured)
2. Complete Phase 2: Foundational (verify builds)
3. Complete Phase 3: Builder (US1)
4. **STOP and VALIDATE**: `cargo test` in `builder/`
5. You now have a working derive macro!

### Incremental Delivery

| Milestone | Stories Complete | Verification |
|-----------|-----------------|--------------|
| MVP | US1 (Builder) | `cd builder && cargo test` |
| +Generics | US1 + US2 (Debug) | Both pass independently |
| +Custom Syntax | US1-3 | All 3 projects pass |
| +Validation | US1-4 | All 4 projects pass |
| Complete | US1-5 | `cargo test --workspace` |

### Test-Driven Workflow (Per Test File)

For each test file (e.g., `01-parse.rs`):

1. **Read** the test file comments for implementation hints
2. **Uncomment** that test in `progress.rs`
3. **Run** `cargo test` — verify it fails
4. **Implement** the feature in `src/lib.rs`
5. **Run** `cargo test` — verify it passes
6. **Commit** the working implementation
7. **Proceed** to next test file

---

## Task Summary

| Phase | Tasks | Parallel | Description |
|-------|-------|----------|-------------|
| 1. Setup | T001-T009 | 8/9 | Add dependencies to all Cargo.toml |
| 2. Foundational | T010-T014 | 4/5 | Verify all projects compile |
| 3. US1 Builder | T015-T025 | 0/11 | Implement builder (9 tests) |
| 4. US2 Debug | T026-T035 | 0/10 | Implement debug (8 tests) |
| 5. US3 Seq | T036-T046 | 0/11 | Implement seq (9 tests) |
| 6. US4 Sorted | T047-T056 | 0/10 | Implement sorted (8 tests) |
| 7. US5 Bitfield | T057-T070 | 0/14 | Implement bitfield (12 tests) |
| 8. Polish | T071-T076 | 3/6 | Final validation |

**Total Tasks**: 76  
**Parallel Opportunities**: 15 tasks can run in parallel within their phase  
**User Stories**: Each is independently testable

---

## Notes

- Tasks within a user story are SEQUENTIAL (each test builds on previous)
- User stories themselves are PARALLEL-capable (different directories)
- Test files contain detailed implementation hints — read them!
- Commit after each passing test to maintain working state
- Use `cargo expand` to debug generated code
- Use `eprintln!` in macros during development (remove before final)

