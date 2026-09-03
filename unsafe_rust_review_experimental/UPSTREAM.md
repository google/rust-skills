# Upstream provenance

Repository: `google/zerocopy`

Revision: `2dad389b030e9268d6645ac0bf0626b867e96068`

| Upstream path | Destination path | Treatment |
| --- | --- | --- |
| `skills/unsafe-rust/SKILL.md` | `SKILL.md` | Skill name and repository-relative paths adapted for rust-skills |
| `skills/unsafe-rust/agents/` | `agents/` | Skill name and repository-relative paths adapted for rust-skills |
| `skills/unsafe-rust/assets/` | `assets/` | Skill name and repository-relative paths adapted for rust-skills |
| `skills/unsafe-rust/references/` | `references/` | Skill name and repository-relative paths adapted for rust-skills |
| `evals/unsafe-rust/` | `evals/unsafe-rust/` | Byte-for-byte copy |
| `maintainers/unsafe-rust-skill-design.md` | `maintainers/README.md` | Port header, local skill name, and repository-relative links adapted |

The preserved evaluation runs apply to the frozen upstream
`unsafe-rust` package revisions identified by their manifests. They do
not, by themselves, constitute evaluation results for the renamed
`unsafe-rust-review-experimental` port.

The installable runtime closure is limited to `SKILL.md`, `agents/`,
`assets/`, and `references/`. Evaluation tooling must stage those paths
into an isolated directory rather than mounting this complete source
bundle.
