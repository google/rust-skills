# Unsafe Rust Review Skill - Background & Attributions

This skill provides a mathematically rigorous, proof-obligation-based
methodology for auditing and reviewing unsafe Rust code.

## Authorship & Development

*   **Primary Development**: Developed by @joshlf working in collaboration
    with an AI agent.
*   **Extensions & Refinements**: @manishearth added additional rules,
    policy alignment, and operational guidance based on extensive experiences
    reviewing unsafe Rust code internally at Google.

## Theoretical & Practical Foundations

The rules and verification criteria within this skill are grounded in:

*   **Language & Standard Semantics**:
    *   [The Rust Reference](https://doc.rust-lang.org/reference/)
    *   The Rust standard library documentation
    *   [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
    *   [The Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/)
    *   Rust RFCs
    *   [Clippy documentation](https://rust-lang.github.io/rust-clippy/master/index.html)
*   **Real-world Findings**:
    *   Incorporates bug patterns and safety findings documented in the
        open-source
        [google/rust-crate-audits](https://github.com/google/rust-crate-audits)
        repository.

