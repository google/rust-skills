# Evaluation material

[`unsafe-rust/`](unsafe-rust/) is a byte-for-byte import of
`google/zerocopy/evals/unsafe-rust/` at commit
`2dad389b030e9268d6645ac0bf0626b867e96068`.

Paths, package names, manifests, digests, and execution-environment
locations recorded inside preserved runs describe the original
Zerocopy checkout. They have intentionally not been rewritten.

These runs evaluated the upstream `unsafe-rust` package. Runs against
the renamed rust-skills port belong in a distinct
`unsafe-rust-review-experimental/` subtree and require their own
manifests and package digests.

Evaluators must not expose this directory to the agent under test.
Stage only the runtime allowlist with
[`../maintainers/stage-runtime.sh`](../maintainers/stage-runtime.sh).
