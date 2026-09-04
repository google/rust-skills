# Experimental unsafe Rust review skill bundle

This directory contains both the installable skill and the development
material needed to maintain and evaluate it.

## Runtime package

The installable runtime package is exactly:

- [`SKILL.md`](SKILL.md)
- [`agents/`](agents/)
- [`assets/`](assets/)
- [`references/`](references/)

Do not install or expose this directory recursively. In particular,
`evals/` and `maintainers/` contain evaluator-only and maintainer-only
material that must not be available to an evaluated agent. Use
[`maintainers/stage-runtime.sh`](maintainers/stage-runtime.sh) to create
an allowlisted runtime directory.

## Development material

- [`evals/unsafe-rust/`](evals/unsafe-rust/) is a byte-for-byte import
  of the upstream evaluation corpus. It retains the upstream package
  name because its frozen packages and historical runs evaluated that
  exact package identity.
- [`maintainers/README.md`](maintainers/README.md) is the path-adapted
  maintainer design guide for this port.
- [`UPSTREAM.md`](UPSTREAM.md) records the source revision, path
  mapping, and adaptation policy.

Historical paths, package names, manifests, digests, and run records
inside `evals/unsafe-rust/` are evidence about the original execution
environment. They are intentionally not rewritten. A run against this
renamed port requires a separate manifest and result set.
