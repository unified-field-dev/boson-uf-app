# boson-app Quality Gates

This crate uses cargo quality gates (`fmt` / `clippy` / `test` / `doc`) and
`cargo llvm-cov` as the source of truth for executable line coverage.

## Current Health Report

Last updated: 2026-03-17

Baseline metric source:
- `cargo test -p boson-app`
- `cargo llvm-cov -p boson-app --summary-only`

- Structure health (historical):
  - Overall grade: `unknown`
  - Structure grade: `D`
  - Architecture grade: `A`
  - Graph summary: `25` files, `1915` lines, `43` import edges



- Tests: `not captured`
- LLVM line coverage: `not captured`

## Targets

- Preserve or improve structure and architecture grades.
- Keep circular dependencies at zero and prevent unexpected coupling regressions.
- Raise LLVM line coverage over time with targeted module tests.
- Tighten public surface area and remove dead or unused paths where practical.

## Local Commands

### Quality CLI (recommended)

Generate this crate baseline end-to-end with the shared tool:

```bash
cargo run -p quality -- check --target boson-app
```

### Cargo quality gates (preferred)

Run in this order:
1. `scan(path="boson-app")`
2. `health()`
3. `cycles()`
4. `coupling()`
5. `architecture()`
6. `test_gaps(limit=20)`
7. `hottest(limit=10)`

### Proper test-coverage measurement

Use LLVM source-based coverage:

```bash
cargo llvm-cov -p boson-app --text --show-missing-lines
```

Optional summary export:

```bash
cargo llvm-cov -p boson-app --json --summary-only --output-path boson-app/coverage-summary.json
```

## CI Gate Policy

- CI should enforce `cargo test -p boson-app`.
- CI should capture LLVM coverage summary when `cargo-llvm-cov` is available.
- quality review checks should run as best-effort in CI (`scan`, `cycles`, `coupling`, `health`) when quality review CLI is available.
- Trend structure grade, architecture grade, and LLVM line coverage for this crate.
