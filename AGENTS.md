## Scope

This file applies to the entire repository and provides guidance for the assistant working on pull requests.

### Testing and Build Guidelines
- Run `./build.sh` when changing more than ~20 lines of code or when modifying multiple modules.
- Small changes under ~20 lines, such as documentation updates or minor tweaks, do not require running build or tests.
- If there are tests available, run `cargo test` only after large code refactors or feature additions.

### Commit Practices
- Use `git status` to confirm a clean repository before and after commits.
- Keep commit messages concise and descriptive.

