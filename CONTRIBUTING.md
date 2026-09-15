# Persona Exporter - Contributing

## Branches
### Major branches
`develop` - dev branch which contains the latest (potentially breaking) code.

`unstable` - Focused on product testing and bug fixing.

`release-candidate` - A branch for full product testing. 
Code merged into it is considered polished and 
ready for widespread use, but still requires feedback.

`main` - Release branch. The contains a stable and polished product ready for use.
### Your branches

Create your branches using the 'purpose/functionality' format.

**Purpose levels:**
- **feature** for new features
- **bugfix** for bug fixes
- **docs** for documentation
- **refactoring** for refactoring (improving and optimizing) existing code

**Example:** "*feature/my-branch*", "*docs/update-system-module*"

## Pull Request
**Before send pull request, check your code:**

check code
```bash
cargo check --release
```

check formatting
```bash
cargo clippy -- -D warnings
cargo fmt --all -- --check
```

check documentation
```bash
cargo test
cargo test --doc
```

Now that all tests have passed successfully, you can submit 
a pull request specifically to the `develop` branch


