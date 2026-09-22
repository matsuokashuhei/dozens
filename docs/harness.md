# Quality harness

This document defines the code metrics for the Rust API and the tool that
enforces each one.

## Thresholds

| Metric | Threshold | Enforced by |
|---|---|---|
| Cyclomatic complexity | 10, reported only | SonarQube metric |
| Cognitive complexity | 10 | SonarQube rule S3776 |
| Propagation cost | 0.35 | codelore `check` |
| Unused dependencies | 0 | cargo-machete |
| Coverage | 85 overall, 90 new code | cargo-llvm-cov and SonarQube |
| Compiler and lint warnings | 0 | `-Dwarnings` |

## Where each metric lives

Rust settings live in two files. `apps/api/Cargo.toml` sets
`warnings = "deny"`. `apps/api/rustfmt.toml` pins the format edition.
Clippy uses its default lint set. CI passes `-Dwarnings` so warnings fail the job.

`sonar-project.properties` points the scanner at the crate and the LCOV
report. `infra/sonar/docker-compose.yml` runs a
local SonarQube. `infra/sonar/quality-gate.json` holds the gate
conditions and the rule parameters. `infra/sonar/provision.sh` creates
the project if it is missing, replaces the gate conditions, assigns the
gate, copies the Rust profile to `Dozenz Rust`, makes it the default,
and sets the `rust:S3776` threshold to 10.

The Rust analyzer reports cyclomatic complexity as a metric. It has no
cyclomatic rule, so the gate cannot fail on that number. The cognitive
complexity rule S3776 carries the threshold. The gate ignores a coverage
condition until a coverage report for the project exists, so the strict
coverage thresholds apply only after the first report is imported.

`.github/workflows/quality.yml` runs three jobs. The `rust` job checks
format, lints, tests, coverage, and unused dependencies. The `sonar` job
scans with the SonarQube Rust analyzer and waits for the quality gate.
The `codelore` job checks out full history and runs `codelore check`
against `.codelore-thresholds.toml`.

Run cargo commands in `apps/api`, or pass `--manifest-path apps/api/Cargo.toml`.

## Local commands

Run cargo through the project hook, which sends it to the api dev
container. Type the command as usual.

```
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -Dwarnings
cargo test --all-features
```

The other checks need their own tools. The api dev image includes
`cargo-modules` for a local module-cycle inspection. That command is not
a CI check.

```
cargo install cargo-machete cargo-llvm-cov --locked
cargo llvm-cov --all-features --lcov --output-path lcov.info
cargo machete
cargo modules dependencies --lib --acyclic
codelore check --repo .
```

For SonarQube, start the server and apply the gate.

```
docker compose -f infra/sonar/docker-compose.yml up -d
SONAR_HOST_URL=http://localhost:9000 SONAR_TOKEN=<token> sh infra/sonar/provision.sh
```

## Sources

- Cognitive Complexity, SonarSource: https://www.sonarsource.com/resources/cognitive-complexity/
- SonarQube Rust analyzer: https://docs.sonarsource.com/sonarqube-server/analyzing-source-code/languages/rust
- Clippy in CI: https://doc.rust-lang.org/clippy/continuous_integration/github_actions.html
- OO design quality metrics, Robert C. Martin: http://objectmentor.com/resources/articles/oodmetrc.pdf
- A metrics suite for object oriented design, Chidamber and Kemerer: https://www.cs.kent.edu/~jmaletic/cs63901/lectures/Chidamber94.pdf
- cargo-modules: https://github.com/regexident/cargo-modules
- cargo-machete: https://github.com/bnjbvr/cargo-machete
- codelore: https://github.com/emrecdr/codelore
- Code coverage best practices, Google: https://testing.googleblog.com/2020/08/code-coverage-best-practices.html
