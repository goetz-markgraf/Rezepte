---
name: tdd
description: TDD approach for test-driven development with integration and unit test cycles. always read this skill
  when implementing code and when the user specificly asks from a TDD approach.
version: 1.0.0
---

# Test Implementation

When writing code, follow BDD dual-loop TDD. Every feature increment starts from a failing integration
test and is driven inward through unit-level red-green-refactor cycles.

## Outer loop (integration)

1. Red (integration) — Write one integration/acceptance test that describes the
   next observable behavior from the outside in. Run it. Confirm it fails for the
   reason you expect. Do not proceed until the failure message matches your intent.
2. Inner loop (unit) — repeat until the integration test can pass:

- Red — Write the smallest unit test that expresses the next missing piece of
  implementation the integration test needs.
- Green — Write the minimum production code to make that unit test pass.
  Run it in isolation and confirm. No speculative code.
- Refactor — Clean up the code you just wrote (duplication, naming, structure)
  while all unit tests stay green. Only touch code covered by passing tests.

3. Green (integration) — When enough unit-level pieces exist, re-run the
   integration test. If it still fails, diagnose which piece is missing and drop back
   into the inner loop. Do not add code without a failing test driving it.
4. Refactor (integration) — With the integration test green, refactor across
   module boundaries if needed. All tests — unit and integration — must stay green.
5. Repeat from step 1 with the next slice of behavior until the task is complete.

## Discipline rules

- Never skip the red step. If you cannot articulate why a test fails, you do not yet
  understand the requirement.
- One logical change per cycle. If you are changing more than one behavior at a
  time, split it.
- Run only the relevant test after each green step, then the full suite before each
  commit-worthy checkpoint.
- If a refactor breaks a test, revert the refactor — do not fix forward.
- Treat a surprise failure (wrong message, wrong location) as information: re-read
  it, adjust your understanding, then proceed.
