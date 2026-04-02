# Release Checklist — graph-rs

Personal checklist for tagging a new version and pushing docs to GitHub Pages.

---

## 1 · Code quality

- [ ] `cargo fmt --all -- --check` — zero diff
- [ ] `cargo clippy --all -- -D warnings` — zero errors
- [ ] `cargo test --all` — zero failures
- [ ] `cargo doc --workspace --no-deps` — zero warnings

---

## 2 · Docs smoke-check

- [ ] `cargo doc --workspace --no-deps --open` — skim each crate's root page
- [ ] Public items have doc comments (no orphan `pub fn` with no `///`)
- [ ] Code examples in doc comments compile (`cargo test --doc --workspace`)

---

## 3 · Version bump

- [ ] Bump `version` in `Cargo.toml` (workspace root `[workspace.package]`)
- [ ] `cargo check --workspace` — version propagates cleanly to all crates

---

## 4 · README

- [ ] Workspace layout table reflects any new or renamed crates
- [ ] Algorithm complexity table is up to date
- [ ] Badge URLs are correct (CI badge points at `ci.yml`; docs badge points at
  `https://chaganti-reddy.github.io/graph-rs`)

---

## 5 · Commit & tag

```bash
git add CHANGELOG.md Cargo.toml Cargo.lock README.md
git commit -m "chore: release vX.Y.Z"
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin main --tags
```

---

## 6 · CI / GitHub Pages

- [ ] Push triggers the CI pipeline — watch the Actions tab
- [ ] `check` job passes (fmt → clippy → test → doc build)
- [ ] `deploy` job runs and succeeds (only fires on `main`)
- [ ] Docs are live at `https://chaganti-reddy.github.io/graph-rs`
  (may take ~60 s after the deploy job completes)

---

## 7 · GitHub release

- [ ] Create a GitHub release tagged `vX.Y.Z`
- [ ] Paste the relevant `CHANGELOG` as the release body
- [ ] Set as latest release

---

## Semver policy

| Change                                  | Version bump        |
| --------------------------------------- | ------------------- |
| New public function / type              | Minor (0.x.0)       |
| Rename / remove public API              | Major (x.0.0)       |
| Bug fix, no API change                  | Patch (0.0.x)       |
| New algorithm in existing crate         | Minor               |
| Breaking trait change in `graph-core` | Major on all crates |

Because `graph-core`'s `Graph` trait is the foundation of the entire workspace,
any breaking change there requires a coordinated major bump of all eight crates.
Keep the trait stable.
