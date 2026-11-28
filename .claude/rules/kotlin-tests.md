---
paths:
  - "**/*.kt"
  - "**/*.kts"
---

## Rules

### Kotlin Tests

Tests written with a Kotlin test framework MUST be split by package, right under the root package: specifications into `spec`, and all other tests into `nonspec`, as in `com.example.spec` and `com.example.nonspec`.
What lies below those two packages MAY be laid out freely.
