# Contributing to Conway's Game of Life

Thank you for your interest in contributing to this project! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find that you don't need to create one. When creating a bug report, please include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Screenshots if applicable
- Your environment (OS, Rust version, etc.)

### Suggesting Features

Feature suggestions are welcome! Please:

- Use a clear and descriptive title
- Provide a detailed description of the suggested feature
- Explain why this feature would be useful
- Consider if this fits the project's scope

### Pull Requests

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Add or update tests as necessary
5. Ensure the test suite passes
6. Make sure your code follows the project's style guidelines
7. Write a clear commit message following our commit convention
8. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- System dependencies for egui (varies by platform)

### Building

```bash
# Clone your fork
git clone https://github.com/shallvtalk/game_of_life.git
cd game_of_life

# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test
```

## Code Style

### Formatting

This project uses `rustfmt` for code formatting. Before submitting any code:

```bash
cargo fmt
```

### Linting

We use `clippy` for additional linting:

```bash
cargo clippy -- -D warnings
```

### Project Structure

- `src/main.rs` - Application entry point and main structure
- `src/game.rs` - Core game logic and Conway's Game of Life implementation
- `src/ui.rs` - User interface components and interactions
- `src/patterns.rs` - Preset pattern definitions

## Commit Convention

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `build`: Changes that affect the build system or external dependencies
- `ci`: Changes to CI configuration files and scripts
- `chore`: Other changes that don't modify src or test files

### Scopes

- `ui`: User interface changes
- `game`: Game logic changes
- `patterns`: Pattern-related changes
- `docs`: Documentation changes
- `config`: Configuration changes
- `deps`: Dependency changes
- `ci`: CI/CD changes

### Examples

```
feat(ui): add zoom functionality for game grid
fix(game): correct cell neighbor calculation
docs(readme): update installation instructions
refactor(patterns): organize patterns by category
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

- Write unit tests for new functions
- Add integration tests for new features
- Ensure tests are clear and focused
- Use descriptive test names

## Documentation

- Document public APIs with doc comments
- Update README.md if adding user-facing features
- Add inline comments for complex logic
- Keep documentation current with code changes

## Release Process

Releases are automated through GitHub Actions when tags are pushed:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit changes: `chore: bump version to X.Y.Z`
4. Create and push tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
5. GitHub Actions will create the release and build binaries

## Getting Help

If you need help with contributing:

- Check existing issues and discussions
- Create a new issue with the `question` label
- Reach out to maintainers

Thank you for contributing!
