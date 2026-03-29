# Contributing

## Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [pnpm](https://pnpm.io/) v10+

## Setup

```bash
pnpm install
```

This automatically installs Husky git hooks via the `prepare` script.

---

## Git Hooks

Three hooks run automatically — no manual steps needed.

### pre-commit

Runs `lint-staged` before every commit. Only staged `.ts` and `.tsx` files are checked, keeping it fast.

- ESLint auto-fixes linting issues
- Prettier auto-formats code

If either fails, the commit is blocked until you fix the errors.

### commit-msg

Enforces [Conventional Commits](https://www.conventionalcommits.org/) on every commit message via `commitlint`.

**Format:** `type(scope): description`

**Allowed types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`

**Allowed scopes:** `auth`, `wallet`, `dashboard`, `marketplace`, `admin`, `donation`, `carbon`, `ui`, `layout`, `nav`, `config`, `deps`

Examples:
```
feat(wallet): add freighter connection flow
fix(auth): handle expired session tokens
docs(config): update environment variable reference
```

### pre-push

Runs `pnpm build` before every push. If the build fails, the push is blocked.

This ensures broken code never reaches the remote. Fix all build errors locally before pushing.

---

## Manual Commands

```bash
# Lint all files
pnpm lint

# Auto-fix lint issues
pnpm lint:fix

# Format all files
pnpm format

# Check formatting without writing
pnpm format:check

# Full build
pnpm build
```

---

## Bypassing Hooks (emergency only)

```bash
git commit --no-verify -m "chore: emergency fix"
git push --no-verify
```

Use `--no-verify` only when absolutely necessary. CI will still enforce all checks.
