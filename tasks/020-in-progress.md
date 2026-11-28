# Task 020: Pluginval Integration Testing

## Status: IN PROGRESS

## Description
Set up pluginval-based integration testing for the Periodic Instruments VST3 plugin. This covers the DAW integration tests moved from Task 001.

## Dependencies
- Task 001 (Foundation Application) must be `done` and closed
- Requires built VST3 bundle from Task 001

## Specifications Created
- `specs/task-020-pluginval/user-stories.md` — 8 user stories (US-020.1 through US-020.8)
- `specs/task-020-pluginval/acceptance-criteria.md` — 8 acceptance criteria groups (AC-020.1 through AC-020.8 + NF-AC)
- `specs/task-020-pluginval/test-scenarios.md` — 30+ test scenarios covering both platforms

## Implementation Summary

### Files Created/Modified

#### 1. GitHub Actions Workflow: `.github/workflows/pluginval.yml`
- **Triggers**: `push` and `pull_request` to `main` and `develop` branches
- **Jobs**: Two parallel jobs — `pluginval-windows` and `pluginval-linux`
- **Windows Runner**: `runs-on: [self-hosted, win]` (GitHub runner labeled "win")
- **Linux Runner**: `runs-on: [self-hosted, linux]` (internal/self-hosted runner labeled "linux")
- **Pluginval Version**: Pinned to `v0.3.8` via `PLUGINVAL_VERSION` environment variable
- **Caching**: Uses `actions/cache` with keys `pluginval-windows-${{ env.PLUGINVAL_VERSION }}` and `pluginval-linux-${{ env.PLUGINVAL_VERSION }}`
- **VST3 Bundle Path**: `target/release/Periodic Instruments.vst3`
- **Strictness Levels**: Runs both level 5 and level 7 sequentially per platform
- **Failure Handling**: 
  - Hard gate — no `continue-on-error: true`
  - Uploads pluginval logs as artifacts on failure (`pluginval-logs-{windows,linux}-level{5,7}`)
  - 15-minute timeout per job
- **Headless Execution**: 
  - Windows: Native headless (no GUI dependencies)
  - Linux: Uses `xvfb-run -a` for virtual display
- **Permissions**: Least privilege (`contents: read`)

#### 2. Local Test Script (Linux/macOS): `scripts/run-pluginval.sh`
- Accepts VST3 path, strictness level, and pluginval version as arguments
- Downloads and caches pluginval binary locally (`.cache/pluginval-<version>/linux/`)
- Uses `xvfb-run -a` for headless execution on Linux
- Returns pluginval exit code for CI/CD integration
- Usage examples included in script header

#### 3. Local Test Script (Windows): `scripts/run-pluginval.ps1`
- Accepts VST3 path, strictness level (5 or 7), and pluginval version as parameters
- Downloads and caches pluginval binary locally (`.cache/pluginval-<version>/windows/`)
- Native headless execution on Windows
- Returns pluginval exit code
- PowerShell parameter validation (`ValidateSet(5, 7)`)
- Usage examples in comment-based help

#### 4. Documentation Update: `README.md`
- Added "Pluginval Integration Testing" section under Building from Source
- Documents local test script usage for both platforms
- Describes CI integration details (workflow, runners, strictness levels, failure handling)
- Links to detailed specifications in `specs/task-020-pluginval/`

### Acceptance Criteria Coverage

| Criterion | Status | Implementation Details |
|-----------|--------|------------------------|
| AC-020.1: Workflow file with correct runners | ✅ Implemented | `.github/workflows/pluginval.yml` with `self-hosted, win` and `self-hosted, linux` |
| AC-020.2: Level 5 passes both platforms | ⏳ Pending CI run | Workflow runs level 5 first on both platforms |
| AC-020.3: Level 7 passes both platforms | ⏳ Pending CI run | Workflow runs level 7 after level 5 on both platforms |
| AC-020.4: Headless execution | ✅ Implemented | Windows native; Linux uses `xvfb-run -a` |
| AC-020.5: Block sizes & sample rates | ⏳ Verified by pluginval | Level 7 tests 64–4096 block sizes, 44.1k–192k sample rates |
| AC-020.6: Parameter automation & MIDI | ⏳ Verified by pluginval | Level 7 tests all 5 params + MIDI note/CC/pitch bend |
| AC-020.7: State save/restore | ⏳ Verified by pluginval | Level 7 tests getState/setState round-trip |
| AC-020.8: Hard CI gate | ✅ Implemented | No `continue-on-error`; failure uploads logs as artifacts |
| NF-AC-020.1: Version pinned | ✅ Implemented | `PLUGINVAL_VERSION: "v0.3.8"` |
| NF-AC-020.2: CI time < 10 min | ⏳ Pending measurement | 15-min timeout per job (conservative) |
| NF-AC-020.3: Cache key includes version+OS | ✅ Implemented | `pluginval-{windows,linux}-${{ env.PLUGINVAL_VERSION }}` |
| NF-AC-020.4: Least privilege permissions | ✅ Implemented | `permissions: contents: read` |
| NF-AC-020.5: Uses Task 001 artifact | ✅ Implemented | References `target/release/Periodic Instruments.vst3` |

### Next Steps
1. Push branch `task/020-pluginval-integration-tests` to origin
2. Trigger CI run to verify workflow executes correctly
3. Monitor pluginval output for any validation failures
4. If pluginval download URLs need adjustment (steinbergmedia/vst3sdk may not have releases), update workflow with correct URLs
5. Run local test scripts on development machines to verify plugin passes validation
6. QA verification per `specs/task-020-pluginval/test-scenarios.md`
7. PM acceptance review

### Notes
- **Pluginval Download URLs**: The workflow assumes pluginval binaries are available at `https://github.com/steinbergmedia/vst3sdk/releases/download/v{VERSION}/pluginval-{win,linux}.zip`. If these URLs are incorrect, they should be updated to the actual release location (possibly Steinberg's website or a dedicated pluginval repository).
- **VST3 Bundle**: The workflow expects the VST3 bundle at `target/release/Periodic Instruments.vst3` (built by Task 001). If Task 001 produces artifacts, consider adding `needs: build` and downloading the artifact instead.
- **Cargo Build**: Not tested in this environment (Rust toolchain not available). Verify `cargo build --release` produces the VST3 bundle at the expected path on a development machine.
- **Self-Hosted Runners**: Ensure GitHub runners labeled `win` and `linux` are configured and accessible.

## Labels
- integration
- testing
- pluginval
- ci
- in-progress
- running