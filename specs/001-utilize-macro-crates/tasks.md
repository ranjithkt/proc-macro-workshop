# Tasks: Utilize Macro Crates

**Input**: Design documents from `/specs/001-utilize-macro-crates/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Tests are NOT explicitly requested. Existing trybuild tests will be used for verification.

**Organization**: Tasks are grouped by user story to enable independent implementation. Each story can be verified by running `cargo test` in the affected project(s).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Add Dependencies)

**Purpose**: Add required crate dependencies to each project's Cargo.toml

- [x] T001 [P] Add `proc-macro-error2 = "2"` to builder/Cargo.toml
- [x] T002 [P] Add `proc-macro-error2 = "2"` to debug/Cargo.toml
- [x] T003 [P] Add `proc-macro-error2 = "2"` to seq/Cargo.toml
- [x] T004 [P] Add `proc-macro-error2 = "2"` to sorted/Cargo.toml
- [x] T005 [P] Add `proc-macro-error2 = "2"` and `heck = "0.5"` to bitfield/impl/Cargo.toml
- [x] T006 Run `cargo check --workspace` to verify all dependencies resolve

**Checkpoint**: All dependencies added, workspace compiles

---

## Phase 2: Foundational (Baseline Measurement)

**Purpose**: Establish baseline metrics before refactoring

- [x] T007 Count logical statements in builder/src/lib.rs parsing code (document in PR) - **~14 statements**
- [x] T008 Count logical statements in debug/src/lib.rs parsing code (document in PR) - **~44 statements**
- [x] T009 Count logical statements in sorted/src/lib.rs error handling code (document in PR) - **~32 statements**
- [x] T010 Count logical statements in bitfield/impl/src/lib.rs parsing code (document in PR) - **~34 statements**
- [x] T011 Run all existing tests to confirm baseline: `cargo test --workspace` - **All 46 tests pass**

**Checkpoint**: Baseline established, all tests pass before refactoring

---

## Phase 3: User Story 1 - Simplified Attribute Parsing with Darling (Priority: P1) 🎯 MVP

**Goal**: Enhance darling usage to eliminate manual attribute parsing boilerplate

**Independent Test**: Run `cargo test` in debug/ and bitfield/ directories

### Implementation for User Story 1

#### Debug Project - FromMeta for Bound Parsing

- [x] T012 [US1] Replace `get_bound()` method with `#[darling(default)] bound: Option<String>` field in DebugInput struct in debug/src/lib.rs
- [x] T013 [US1] Change `#[darling(forward_attrs(debug))]` to `#[darling(attributes(debug))]` in DebugInput in debug/src/lib.rs
- [x] T014 [US1] Remove `attrs: Vec<Attribute>` field from DebugInput if no longer needed in debug/src/lib.rs
- [x] T015 [US1] Update `derive_debug_impl()` to use `input.bound` instead of `input.get_bound()` in debug/src/lib.rs
- [x] T016 [US1] Run `cd debug && cargo test` to verify debug project still passes - **All 8 tests pass**

#### Bitfield-Impl Project - FromField for #[bits]

- [~] T017 [US1] Create `BitfieldFieldInfo` struct - DEFERRED: `#[bits = N]` uses name-value syntax not supported by darling's `attributes()`. Current manual parsing is acceptable.
- [~] T018 [US1] Add `#[darling(default)] bits` field - DEFERRED: See T017
- [~] T019 [US1] Remove `get_bits_attribute()` function - DEFERRED: See T017
- [~] T020 [US1] Update `bitfield_impl()` to use FromField - DEFERRED: See T017
- [x] T021 [US1] Run `cd bitfield && cargo test` to verify bitfield project still passes - **All 12 tests pass**

**Checkpoint**: Darling enhancements complete, debug/ and bitfield/ tests pass

---

## Phase 4: User Story 2 - Enhanced Error Handling with Proc-Macro-Error2 (Priority: P2)

**Goal**: Replace manual `syn::Error` patterns with proc-macro-error2's `abort!` and `#[proc_macro_error]`

**Independent Test**: Run `cargo test` in each project after refactoring

### Implementation for User Story 2

#### Sorted Project (Full Migration)

- [ ] T022 [US2] Add `use proc_macro_error2::{abort, emit_error, proc_macro_error};` imports in sorted/src/lib.rs
- [ ] T023 [US2] Add `#[proc_macro_error]` attribute to `sorted()` entry function in sorted/src/lib.rs
- [ ] T024 [US2] Change `sorted_impl()` return type from `Result<TokenStream>` to `TokenStream` in sorted/src/lib.rs
- [ ] T025 [US2] Replace `Err(Error::new_spanned(...))` with `abort!(...)` in `sorted_impl()` in sorted/src/lib.rs
- [ ] T026 [US2] Replace `Err(Error::new(...))` with `abort!(...)` for variant sorting errors in sorted/src/lib.rs
- [ ] T027 [US2] Remove manual `to_compile_error()` handling in `sorted()` entry function in sorted/src/lib.rs
- [ ] T028 [US2] Add `#[proc_macro_error]` attribute to `check()` entry function in sorted/src/lib.rs
- [ ] T029 [US2] Change `check_impl()` return type and use `emit_error!` for accumulated errors in sorted/src/lib.rs
- [ ] T030 [US2] Simplify `SortedChecker` struct (remove `error: Option<Error>` field) in sorted/src/lib.rs
- [ ] T031 [US2] Run `cd sorted && cargo test` to verify sorted project still passes

#### Seq Project (Entry Point Only)

- [ ] T032 [P] [US2] Add `use proc_macro_error2::proc_macro_error;` import in seq/src/lib.rs
- [ ] T033 [US2] Add `#[proc_macro_error]` attribute to `seq()` entry function in seq/src/lib.rs
- [ ] T034 [US2] Run `cd seq && cargo test` to verify seq project still passes

#### Builder Project (Entry Point)

- [ ] T035 [P] [US2] Add `use proc_macro_error2::{abort, proc_macro_error};` imports in builder/src/lib.rs
- [ ] T036 [US2] Add `#[proc_macro_error]` attribute to `derive()` entry function in builder/src/lib.rs
- [ ] T037 [US2] Replace darling error handling with `abort!(e.span(), "{}", e)` pattern in builder/src/lib.rs
- [ ] T038 [US2] Run `cd builder && cargo test` to verify builder project still passes

#### Debug Project (Entry Point)

- [ ] T039 [P] [US2] Add `use proc_macro_error2::{abort, proc_macro_error};` imports in debug/src/lib.rs
- [ ] T040 [US2] Add `#[proc_macro_error]` attribute to `derive()` entry function in debug/src/lib.rs
- [ ] T041 [US2] Replace darling error handling with `abort!(e.span(), "{}", e)` pattern in debug/src/lib.rs
- [ ] T042 [US2] Run `cd debug && cargo test` to verify debug project still passes

#### Bitfield-Impl Project (Both Entry Points)

- [ ] T043 [P] [US2] Add `use proc_macro_error2::{abort, proc_macro_error};` imports in bitfield/impl/src/lib.rs
- [ ] T044 [US2] Add `#[proc_macro_error]` attribute to `bitfield()` entry function in bitfield/impl/src/lib.rs
- [ ] T045 [US2] Change `bitfield_impl()` return type from `Result<TokenStream>` to `TokenStream` in bitfield/impl/src/lib.rs
- [ ] T046 [US2] Replace `Err(Error::new_spanned(...))` with `abort!(...)` in `bitfield_impl()` in bitfield/impl/src/lib.rs
- [ ] T047 [US2] Add `#[proc_macro_error]` attribute to `derive_bitfield_specifier()` entry function in bitfield/impl/src/lib.rs
- [ ] T048 [US2] Change `derive_specifier_impl()` return type and replace errors with `abort!()` in bitfield/impl/src/lib.rs
- [ ] T049 [US2] Run `cd bitfield && cargo test` to verify bitfield project still passes

**Checkpoint**: All proc-macro-error2 migrations complete, all tests pass

---

## Phase 5: User Story 3 - Case Conversion with Heck (Priority: P3)

**Goal**: Use heck crate for case conversion where applicable (builder and bitfield-impl only)

**Independent Test**: Run `cargo test` in builder/ and bitfield/ directories

### Implementation for User Story 3

#### Builder Project (Activate Existing Dependency)

- [ ] T050 [US3] Add `use heck::ToUpperCamelCase;` import in builder/src/lib.rs
- [ ] T051 [US3] Verify builder struct naming uses heck for case normalization in builder/src/lib.rs
- [ ] T052 [US3] Run `cd builder && cargo test` to verify builder project still passes

#### Bitfield-Impl Project (Use New Dependency)

- [ ] T053 [US3] Add `use heck::ToSnakeCase;` import in bitfield/impl/src/lib.rs
- [ ] T054 [US3] Update getter name generation to use `.to_snake_case()` in bitfield/impl/src/lib.rs
- [ ] T055 [US3] Update setter name generation to use `.to_snake_case()` in bitfield/impl/src/lib.rs
- [ ] T056 [US3] Run `cd bitfield && cargo test` to verify bitfield project still passes

**Checkpoint**: Heck integration complete where applicable

---

## Phase 6: Polish & Verification

**Purpose**: Final validation and documentation of code reduction

- [ ] T057 Run full test suite: `cargo test --workspace`
- [ ] T058 Run clippy on all projects: `cargo clippy --workspace --all-targets`
- [ ] T059 Count logical statements in refactored code (compare to T007-T010 baseline)
- [ ] T060 Document code reduction percentage in PR description (target: 30%+)
- [ ] T061 Verify all entry points have `#[proc_macro_error]` attribute (SC-003)
- [ ] T062 Verify no remaining `syn::Error::new(...).to_compile_error()` patterns (SC-004)
- [ ] T063 Run quickstart.md validation steps for each project

**Checkpoint**: All success criteria verified

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup - establishes baseline
- **User Story 1 (Phase 3)**: Depends on Phase 1 (needs darling available)
- **User Story 2 (Phase 4)**: Depends on Phase 1 (needs proc-macro-error2 available)
- **User Story 3 (Phase 5)**: Depends on Phase 1 (needs heck available)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Independent - affects debug and bitfield-impl only
- **User Story 2 (P2)**: Independent - affects all 5 projects
- **User Story 3 (P3)**: Independent - affects builder and bitfield-impl only

**Note**: User Stories 1, 2, and 3 can be worked on in parallel after Phase 1 since they modify different aspects of the code.

### Parallel Opportunities Within Stories

**User Story 2** has the most parallelization:
- T032, T035, T039, T043 can run in parallel (adding imports to different files)
- Sorted project tasks (T022-T031) are sequential within sorted/
- Each project's refactoring is independent of others

---

## Parallel Example: User Story 2 (Proc-Macro-Error2)

```bash
# After Phase 1 (dependencies added), launch these in parallel:

# Stream 1: Sorted project
Task T022-T031: Full sorted/ migration

# Stream 2: Seq project  
Task T032-T034: Seq entry point

# Stream 3: Builder project
Task T035-T038: Builder entry point

# Stream 4: Debug project
Task T039-T042: Debug entry point

# Stream 5: Bitfield-impl project
Task T043-T049: Bitfield entry points
```

---

## Implementation Strategy

### MVP First (User Story 2 - Most Impact)

For maximum immediate impact, prioritize User Story 2:

1. Complete Phase 1: Setup (add dependencies)
2. Complete Phase 2: Baseline measurement
3. Complete Phase 4: User Story 2 (proc-macro-error2 for ALL projects)
4. **STOP and VALIDATE**: `cargo test --workspace`
5. This alone achieves SC-003 and SC-004

### Incremental Delivery

1. Phase 1 + Phase 2 → Dependencies ready, baseline established
2. Add User Story 2 → All entry points use `#[proc_macro_error]` ✓
3. Add User Story 1 → Darling enhancements for debug/bitfield ✓
4. Add User Story 3 → Heck for case conversion ✓
5. Phase 6 → Final verification and documentation

### Single Developer Strategy

Recommended order for one developer:
1. T001-T011 (Setup + Baseline)
2. T022-T049 (US2 - All proc-macro-error2, project by project)
3. T012-T021 (US1 - Darling enhancements)
4. T050-T056 (US3 - Heck usage)
5. T057-T063 (Polish)

---

## Summary

| Phase | Tasks | Parallelizable |
|-------|-------|----------------|
| Setup | T001-T006 | T001-T005 (5 tasks) |
| Foundational | T007-T011 | T007-T010 (4 tasks) |
| US1 (Darling) | T012-T021 | None (sequential per project) |
| US2 (Error2) | T022-T049 | T032,T035,T039,T043 (imports) |
| US3 (Heck) | T050-T056 | None (2 projects) |
| Polish | T057-T063 | T057-T058 (2 tasks) |

**Total**: 63 tasks  
**Per Story**: US1=10, US2=28, US3=7  
**Parallel Opportunities**: 15 tasks can run in parallel with others

---

## Notes

- [P] tasks = different files, no dependencies on other incomplete tasks
- [USn] label maps task to specific user story for traceability
- Run `cargo test` after each project's changes to catch regressions early
- Commit after each completed task or logical group
- Semantic equivalence for error messages means spans must match, wording may differ

