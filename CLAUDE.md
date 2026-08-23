# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Preflight

Run the repository-defined preflight through the CLI:

```bash
cargo run -- code run_preflight
```

This reads `tool.yaml` and runs its configured commands in order. Its final changelog step invokes `git-cliff` and stages `CHANGELOG.md`; use individual checks instead when regenerating and staging the changelog is not intended. Always run this preflight before making a commit.
