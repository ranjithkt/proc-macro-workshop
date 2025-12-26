# Tasks: Proc-Macro Tutorial Documentation

**Input**: Design documents from `/specs/002-proc-macro-tutorial/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Not applicable (documentation project - validation via manual review and code example compilation)

**Organization**: Tasks grouped by user story (chapter) to enable independent writing and review

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1-US6)
- Include exact file paths in descriptions

## Path Conventions

Documentation structure from plan.md:
```
docs/proc-macro-tutorial/
├── README.md
├── 01-tokens.md
├── 02-syn.md
├── 03-quote.md
├── 04-darling.md
├── 05-heck.md
├── 06-pipeline.md
└── examples/
    ├── 01-token-debug/
    ├── 02-parse-struct/
    ├── 03-generate-impl/
    ├── 04-darling-attrs/
    └── 05-case-convert/
```

---

## Phase 1: Setup (Directory Structure)

**Purpose**: Create documentation directory and example project scaffolding

- [ ] T001 Create documentation directory structure at `docs/proc-macro-tutorial/`
- [ ] T002 [P] Create example project scaffold `docs/proc-macro-tutorial/examples/01-token-debug/` with Cargo.toml
- [ ] T003 [P] Create example project scaffold `docs/proc-macro-tutorial/examples/02-parse-struct/` with Cargo.toml
- [ ] T004 [P] Create example project scaffold `docs/proc-macro-tutorial/examples/03-generate-impl/` with Cargo.toml
- [ ] T005 [P] Create example project scaffold `docs/proc-macro-tutorial/examples/04-darling-attrs/` with Cargo.toml
- [ ] T006 [P] Create example project scaffold `docs/proc-macro-tutorial/examples/05-case-convert/` with Cargo.toml

**Checkpoint**: Directory structure ready for content

---

## Phase 2: Foundational (Index & Cross-Cutting)

**Purpose**: Create the tutorial index and establish common elements

- [ ] T007 Write tutorial index with chapter links in `docs/proc-macro-tutorial/README.md`
- [ ] T008 Add "The Crate Ecosystem" overview table to `docs/proc-macro-tutorial/README.md`
- [ ] T009 Add "How to Read This Tutorial" section to `docs/proc-macro-tutorial/README.md`
- [ ] T010 Add "Running the Examples" instructions to `docs/proc-macro-tutorial/README.md`

**Checkpoint**: Index complete - chapter writing can begin

---

## Phase 3: User Story 1 - Understanding Tokens (Priority: P1) 🎯 MVP

**Goal**: Teach TokenStream and proc-macro2 fundamentals

**Independent Test**: Reader can explain TokenTree variants and debug tokens with eprintln!

### Chapter Content

- [ ] T011 [US1] Write "What You'll Learn" section in `docs/proc-macro-tutorial/01-tokens.md`
- [ ] T012 [US1] Write "Macros See Tokens, Not Text" introduction in `docs/proc-macro-tutorial/01-tokens.md`
- [ ] T013 [US1] Write TokenStream and TokenTree explanation in `docs/proc-macro-tutorial/01-tokens.md`
- [ ] T014 [US1] Create TokenStream structure Mermaid diagram (FR-010) in `docs/proc-macro-tutorial/01-tokens.md`
- [ ] T015 [US1] Write "Why proc-macro2 Exists" section in `docs/proc-macro-tutorial/01-tokens.md`
- [ ] T016 [US1] Write "Key Takeaways" conclusion in `docs/proc-macro-tutorial/01-tokens.md`

### Example Code

- [ ] T017 [US1] Implement token debug macro in `docs/proc-macro-tutorial/examples/01-token-debug/src/lib.rs`
- [ ] T018 [US1] Add usage example with eprintln output in `docs/proc-macro-tutorial/examples/01-token-debug/examples/demo.rs`
- [ ] T019 [US1] Verify example compiles: `cd docs/proc-macro-tutorial/examples/01-token-debug && cargo check`

**Checkpoint**: Chapter 1 complete and testable independently

---

## Phase 4: User Story 2 - Parsing with Syn (Priority: P1)

**Goal**: Teach syn's structured parsing of Rust syntax

**Independent Test**: Reader can parse a struct and iterate its fields

### Chapter Content

- [ ] T020 [US2] Write "What You'll Learn" section in `docs/proc-macro-tutorial/02-syn.md`
- [ ] T021 [US2] Write "Tokens Are Too Low-Level" problem statement in `docs/proc-macro-tutorial/02-syn.md`
- [ ] T022 [US2] Write DeriveInput explanation with code examples in `docs/proc-macro-tutorial/02-syn.md`
- [ ] T023 [US2] Write Data enum and Fields explanation in `docs/proc-macro-tutorial/02-syn.md`
- [ ] T024 [US2] Create syn type hierarchy Mermaid diagram (FR-011) in `docs/proc-macro-tutorial/02-syn.md`
- [ ] T025 [US2] Write attribute parsing section in `docs/proc-macro-tutorial/02-syn.md`
- [ ] T026 [US2] Write feature flags guidance in `docs/proc-macro-tutorial/02-syn.md`
- [ ] T027 [US2] Write "Key Takeaways" conclusion in `docs/proc-macro-tutorial/02-syn.md`

### Example Code

- [ ] T028 [US2] Implement struct parser macro in `docs/proc-macro-tutorial/examples/02-parse-struct/src/lib.rs`
- [ ] T029 [US2] Add usage example with eprintln output in `docs/proc-macro-tutorial/examples/02-parse-struct/examples/demo.rs`
- [ ] T030 [US2] Verify example compiles: `cd docs/proc-macro-tutorial/examples/02-parse-struct && cargo check`

**Checkpoint**: Chapter 2 complete and testable independently

---

## Phase 5: User Story 3 - Code Generation with Quote (Priority: P1)

**Goal**: Teach quote's quasi-quoting for code generation

**Independent Test**: Reader can generate an impl block with field iteration

### Chapter Content

- [ ] T031 [US3] Write "What You'll Learn" section in `docs/proc-macro-tutorial/03-quote.md`
- [ ] T032 [US3] Write "Building TokenStreams Manually" problem statement in `docs/proc-macro-tutorial/03-quote.md`
- [ ] T033 [US3] Write basic quote! usage with examples in `docs/proc-macro-tutorial/03-quote.md`
- [ ] T034 [US3] Write variable interpolation (#var) explanation in `docs/proc-macro-tutorial/03-quote.md`
- [ ] T035 [US3] Write repetition syntax (#(...)* ) explanation in `docs/proc-macro-tutorial/03-quote.md`
- [ ] T036 [US3] Write quote_spanned! for error locations in `docs/proc-macro-tutorial/03-quote.md`
- [ ] T037 [US3] Write format_ident! usage in `docs/proc-macro-tutorial/03-quote.md`
- [ ] T038 [US3] Write "Key Takeaways" conclusion in `docs/proc-macro-tutorial/03-quote.md`

### Example Code

- [ ] T039 [US3] Implement Debug derive macro in `docs/proc-macro-tutorial/examples/03-generate-impl/src/lib.rs`
- [ ] T040 [US3] Add usage example with cargo expand output in `docs/proc-macro-tutorial/examples/03-generate-impl/examples/demo.rs`
- [ ] T041 [US3] Verify example compiles: `cd docs/proc-macro-tutorial/examples/03-generate-impl && cargo check`

**Checkpoint**: Chapter 3 complete - core pipeline (parse→generate) now documented

---

## Phase 6: User Story 4 - Ergonomic Attributes with Darling (Priority: P2)

**Goal**: Teach declarative attribute parsing with darling

**Independent Test**: Reader can refactor manual parsing to use darling traits

### Chapter Content

- [ ] T042 [US4] Write "What You'll Learn" section in `docs/proc-macro-tutorial/04-darling.md`
- [ ] T043 [US4] Write manual attribute parsing "before" example in `docs/proc-macro-tutorial/04-darling.md`
- [ ] T044 [US4] Create before/after comparison diagram (FR-013) in `docs/proc-macro-tutorial/04-darling.md`
- [ ] T045 [US4] Write FromDeriveInput explanation in `docs/proc-macro-tutorial/04-darling.md`
- [ ] T046 [US4] Write FromField explanation in `docs/proc-macro-tutorial/04-darling.md`
- [ ] T047 [US4] Write darling attributes cheat sheet in `docs/proc-macro-tutorial/04-darling.md`
- [ ] T048 [US4] Write error handling magic section in `docs/proc-macro-tutorial/04-darling.md`
- [ ] T049 [US4] Write "Key Takeaways" conclusion in `docs/proc-macro-tutorial/04-darling.md`

### Example Code

- [ ] T050 [US4] Implement manual parsing version in `docs/proc-macro-tutorial/examples/04-darling-attrs/src/manual.rs`
- [ ] T051 [US4] Implement darling version in `docs/proc-macro-tutorial/examples/04-darling-attrs/src/lib.rs`
- [ ] T052 [US4] Add usage example in `docs/proc-macro-tutorial/examples/04-darling-attrs/examples/demo.rs`
- [ ] T053 [US4] Verify example compiles: `cd docs/proc-macro-tutorial/examples/04-darling-attrs && cargo check`

**Checkpoint**: Chapter 4 complete and testable independently

---

## Phase 7: User Story 5 - Case Conversion with Heck (Priority: P3)

**Goal**: Teach heck's case conversion utilities

**Independent Test**: Reader can use heck to transform identifiers

### Chapter Content

- [ ] T054 [US5] Write "What You'll Learn" section in `docs/proc-macro-tutorial/05-heck.md`
- [ ] T055 [US5] Write case conversion problem statement in `docs/proc-macro-tutorial/05-heck.md`
- [ ] T056 [US5] Write case conversion traits table in `docs/proc-macro-tutorial/05-heck.md`
- [ ] T057 [US5] Write usage examples with format_ident! in `docs/proc-macro-tutorial/05-heck.md`
- [ ] T058 [US5] Write common patterns section in `docs/proc-macro-tutorial/05-heck.md`
- [ ] T059 [US5] Write "Key Takeaways" conclusion in `docs/proc-macro-tutorial/05-heck.md`

### Example Code

- [ ] T060 [US5] Implement case conversion macro in `docs/proc-macro-tutorial/examples/05-case-convert/src/lib.rs`
- [ ] T061 [US5] Add usage example in `docs/proc-macro-tutorial/examples/05-case-convert/examples/demo.rs`
- [ ] T062 [US5] Verify example compiles: `cd docs/proc-macro-tutorial/examples/05-case-convert && cargo check`

**Checkpoint**: Chapter 5 complete and testable independently

---

## Phase 8: User Story 6 - Complete Pipeline (Priority: P2)

**Goal**: Synthesize all crates into unified mental model

**Independent Test**: Reader can trace a macro through all pipeline stages

### Chapter Content

- [ ] T063 [US6] Write "What You'll Learn" section in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T064 [US6] Write "The Big Picture" introduction in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T065 [US6] Create macro pipeline Mermaid diagram (FR-012) in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T066 [US6] Write annotated Builder macro walkthrough in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T067 [US6] Write common patterns section in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T068 [US6] Write error handling patterns in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T069 [US6] Write testing macros section in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T070 [US6] Write reference checklist in `docs/proc-macro-tutorial/06-pipeline.md`
- [ ] T071 [US6] Write "Key Takeaways" conclusion in `docs/proc-macro-tutorial/06-pipeline.md`

**Checkpoint**: Chapter 6 complete - full tutorial now available

---

## Phase 9: Polish & Validation

**Purpose**: Cross-cutting quality improvements and validation

- [ ] T072 [P] Add cross-references between chapters in all `docs/proc-macro-tutorial/*.md`
- [ ] T073 [P] Add "Learn More" external links to all chapters
- [ ] T074 Verify all Mermaid diagrams render in GitHub preview
- [ ] T075 Verify all code examples compile: `for d in docs/proc-macro-tutorial/examples/*/; do (cd "$d" && cargo check); done`
- [ ] T076 Review tone for conversational style per FR-015
- [ ] T077 Verify each chapter readable in <20 minutes (SC-002)
- [ ] T078 Final proofreading pass on all chapters

**Checkpoint**: Tutorial complete and validated

---

## Dependencies & Execution Order

### Phase Dependencies

```mermaid
graph TD
    P1[Phase 1: Setup] --> P2[Phase 2: Index]
    P2 --> P3[Phase 3: US1 Tokens]
    P3 --> P4[Phase 4: US2 Syn]
    P4 --> P5[Phase 5: US3 Quote]
    P5 --> P6[Phase 6: US4 Darling]
    P5 --> P7[Phase 7: US5 Heck]
    P6 --> P8[Phase 8: US6 Pipeline]
    P7 --> P8
    P8 --> P9[Phase 9: Polish]
```

### User Story Dependencies

| Story | Depends On | Can Start After |
|-------|------------|-----------------|
| US1 (Tokens) | Index | Phase 2 complete |
| US2 (Syn) | US1 | Chapter 1 complete |
| US3 (Quote) | US2 | Chapter 2 complete |
| US4 (Darling) | US3 | Chapter 3 complete |
| US5 (Heck) | US3 | Chapter 3 complete |
| US6 (Pipeline) | US4, US5 | Chapters 4 & 5 complete |

### Parallel Opportunities

**Within Phase 1 (Setup)**:
- T002-T006 can all run in parallel (different directories)

**Within Each Chapter Phase**:
- Content writing and example code can proceed in parallel
- Different sections marked [P] can run in parallel

**Chapters 4 & 5**:
- US4 (Darling) and US5 (Heck) can be written in parallel after US3 complete

---

## Parallel Example: Phase 1

```bash
# All example scaffolds can be created simultaneously:
T002: "Create example project scaffold docs/proc-macro-tutorial/examples/01-token-debug/"
T003: "Create example project scaffold docs/proc-macro-tutorial/examples/02-parse-struct/"
T004: "Create example project scaffold docs/proc-macro-tutorial/examples/03-generate-impl/"
T005: "Create example project scaffold docs/proc-macro-tutorial/examples/04-darling-attrs/"
T006: "Create example project scaffold docs/proc-macro-tutorial/examples/05-case-convert/"
```

---

## Implementation Strategy

### MVP First (Chapters 1-3 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Index
3. Complete Phase 3: US1 (Tokens)
4. Complete Phase 4: US2 (Syn)
5. Complete Phase 5: US3 (Quote)
6. **STOP and VALIDATE**: Core pipeline documented, tutorial usable
7. Deploy/share if ready

### Incremental Delivery

1. Setup + Index → Navigation ready
2. Add Chapter 1 (Tokens) → Foundation documented
3. Add Chapter 2 (Syn) → Parsing documented
4. Add Chapter 3 (Quote) → Generation documented (MVP complete!)
5. Add Chapter 4 (Darling) → Advanced parsing
6. Add Chapter 5 (Heck) → Utility crate
7. Add Chapter 6 (Pipeline) → Synthesis complete
8. Polish → Tutorial finalized

---

## Summary

| Metric | Count |
|--------|-------|
| **Total Tasks** | 78 |
| **Setup Tasks** | 6 |
| **Foundational Tasks** | 4 |
| **US1 Tasks** | 9 |
| **US2 Tasks** | 11 |
| **US3 Tasks** | 11 |
| **US4 Tasks** | 12 |
| **US5 Tasks** | 9 |
| **US6 Tasks** | 9 |
| **Polish Tasks** | 7 |
| **Parallel Opportunities** | 15+ tasks |
| **Required Diagrams** | 4 (FR-010, FR-011, FR-012, FR-013) |

---

## Notes

- [P] tasks = different files, no dependencies
- [US*] label maps task to specific user story for traceability
- Each chapter can be validated independently
- Code examples must compile before chapter is complete
- Mermaid diagrams must render in GitHub
- Commit after each task or logical group

