# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Features

- feat: accept string path for config override
- feat: support glob patterns in conditional repository checks
- feat: preserve build cache when creating staged worktree
- feat: add fallback message when rsync is unavailable

### Miscellaneous Tasks

- chore: bump version to match tags (oops!)
- chore(docs): simplify git-cliff configuration

## 0.10.0 - 2026-08-23

### Documentation

- docs: add Claude Code repository guidance

### Features

- feat!: run preflight commands in a seperated worktree
- feat!: add model override option for AI calls

### Miscellaneous Tasks

- chore: ignore changelog in AI commit prompt

### Other (unconventional)

- ci: add Codeberg release publishing to workflow

## 0.9.0 - 2026-08-07

### Bug Fixes

- fix: make breaking changes and skip_preflight work

### Documentation

- docs: update changelog configuration to categorize CI and docs

### Features

- feat!: add conditional preflight support and subtitle test

### Testing

- tests: add test to yt plugin for subtitles

## 0.8.0 - 2026-08-04

### Features

- feat: add fetch command for webpage retrieval with markdown conversion
- feat: add linting system
- feat!: add preflight checks and remove git hooks
- feat: add post-command support for preflight checks
- feat(preflight): add cargo test command

### Miscellaneous Tasks

- chore: bump version

## 0.7.0 - 2026-08-03

### Features

- feat: add validation for empty git diff before generating commit message

### Miscellaneous Tasks

- chore: bump version

### Refactor

- refactor: remove error handling in trust functions and tests

### Testing

- test: add unit tests for get_public_ip
- test: add invalid config parsing test

## 0.6.6 - 2026-08-03

### Miscellaneous Tasks

- chore: bump version

## 0.6.5-beta6 - 2026-08-03

### Miscellaneous Tasks

- chore: install Github CLI in release workflow
- chore: bump version

## 0.6.5-beta5 - 2026-08-03

### Bug Fixes

- fix(ci): skip release asset uploads

### Miscellaneous Tasks

- chore: bump version

## 0.6.5-beta4 - 2026-08-03

### Bug Fixes

- fix(ci): hopefully fixed ci

### Miscellaneous Tasks

- chore: bumped version

## 0.6.5-beta3 - 2026-08-03

### Bug Fixes

- fix(ci): fixing release system (hopfully)

## 0.6.5-beta2 - 2026-08-03

### Miscellaneous Tasks

- chore: bump cli-tool version to 0.6.5-beta2 and adjust release workflow

## 0.6.5-beta1 - 2026-08-03

### Features

- feat: replace ConfigPath with Config command having Path and Show subcommands

### Miscellaneous Tasks

- chore: ensure release directory exists

### Other (unconventional)

- Bump version to beta

### Testing

- test: verify doctor command succeeds

## 0.6.4 - 2026-08-03

### Bug Fixes

- fix(ci): release system

### Other (unconventional)

- Revert "fix(ci): Hopefully this works now"

## 0.6.3 - 2026-08-03

### Bug Fixes

- fix(ci): Hopefully this works now

### Miscellaneous Tasks

- chore: bump version

## 0.6.2 - 2026-08-03

### Bug Fixes

- fix(ci): forgot to comment out a line

### Miscellaneous Tasks

- chore: bump version

## 0.6.1 - 2026-08-03

### Miscellaneous Tasks

- chore: bump version

### Other (unconventional)

- ci: made releases work

## 0.6.0 - 2026-08-03

### Bug Fixes

- fix: make tv config optional and handle missing configuration gracefully

### Features

- feat!: add doctor command for system health checks
- feat: add optional config path parameter to load_config

### Miscellaneous Tasks

- chore: bump version number

### Refactor

- refactor: moved config into lazylock
- refactor: replace string formatting with formatdoc for yaml config

### Testing

- test: add config tests and config_path command

## 0.5.2 - 2026-08-02

### Documentation

- docs: convert comments to doc comments for CLI help visibility

### Miscellaneous Tasks

- chore: bump version to 0.5.2

### Other (unconventional)

- ci: disable release building until further notice

### Refactor

- refactor: make the workflow valid
- refactor: simplify config structs with public fields and simplify dependencies

## 0.5.1 - 2026-08-02

### Miscellaneous Tasks

- chore: bump version to 0.5.1

## 0.5.0 - 2026-08-02

### Features

- feat: Made pre-commit run rustfmt instead of just checking
- feat: add AI-powered git commit command and clean up debug output
- feat: open generated commit message in git editor automatically

### Refactor

- refactor: restructure config loading with match expressions

### Testing

- test: Add AI generated tests to ai_calls.rs and tv.rs

## 0.4.0 - 2026-08-02

### Documentation

- docs(changelog): Update changelog
- docs(changelog): Update changelog
- docs(changelog): Update changelog
- docs(changelog): Update changelog

### Features

- feat: Can load TVDB keys
- feat: Add TVBD search function
- feat: reorganize tvbd search to be under the search subcommand
- feat: hopefully fix changelog generation

### Miscellaneous Tasks

- chore: add git hooks
- chore: bump version

### Refactor

- refactor: removed unnessary code
- refactor: seperate search into two functions, making it easier to reuse.

### Styling

- style: run rustfmt

## 0.3.0 - 2026-07-19

### Features

- feat!: switch to yaml based config

### Miscellaneous Tasks

- chore: bump version numbers

### Refactor

- refactor: run rustfmt

## 0.2.0 - 2026-07-19

### Bug Fixes

- fix: cliff.toml to ignore automated changelog commits
- fix(ci): Add git-cliff to PATH when restoring from cache
- fix(ci): Adding git-cliff to the path and installation conditions
- fix(ci): Remove caching that isn't working

### Documentation

- docs(changelog): Update changelog
- docs(changelog): Update changelog
- docs(changelog): Update changelog
- docs(changelog): Update changelog
- docs(changelog): Update changelog
- docs(changelog): Update changelog

### Features

- feat: get ip address tool
- feat: Youtube downloader to mp4

### Other (unconventional)

- ci: added caching for changlog construction

## 0.1.1 - 2026-07-18

### Bug Fixes

- fix: Made updating the changelog a "docs" type commit
- fix(ci): Install github cli before using it

### Documentation

- docs(changelog): Update changelog
- docs(changelog): Update changelog

### Features

- feat(ci): Add release publishing to github

## 0.1.0 - 2026-07-18

### Features

- feat: Weather command

### Miscellaneous Tasks

- chore: Remove unnessary information from cliff.toml

### Other (unconventional)

- Update changelog

## 0.0.6 - 2026-07-18

### Bug Fixes

- fix(ci): Release to use github link for install-action

### Other (unconventional)

- Update changelog

## 0.0.5 - 2026-07-17

### Bug Fixes

- fix(workflows): Use github url for git cliff installation

### Other (unconventional)

- Update .forgejo/workflows/release.yaml
- Add .forgejo/workflows/changelog.yaml
- Update changelog
- Merge branch 'main' of https://git.jonahmakowski.ca/jonahmakowski/cli-tool
- Update changelog
- ci: Add manual trigger for changelog update
- Update changelog

## 0.0.4 - 2026-07-17

### Features

- feat: add cliff.toml file

### Other (unconventional)

- Update .forgejo/workflows/release.yaml

## 0.0.3 - 2026-07-17

### Other (unconventional)

- Update .forgejo/workflows/release.yaml
- Implemented a "private mode"

## 0.0.2 - 2026-07-17

### Other (unconventional)

- Update .forgejo/workflows/release.yaml
- Update .forgejo/workflows/release.yaml

## 0.0.1 - 2026-07-17

### Other (unconventional)

- Initial commit
- Basic yt summary
- Made it into a simple cli tool
- Added workflows and renovate
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml
- Update .forgejo/workflows/build.yaml

<!-- generated by git-cliff -->
