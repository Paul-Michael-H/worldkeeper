# WorldKeeper

A game built with Rust and the Bevy game engine.

## Overview

WorldKeeper is an ambitious game project that aims to create an engaging and immersive gaming experience using modern Rust development practices and the powerful Bevy engine.

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Git

### Setup

1. Clone the repository:
   ```bash
   git clone <your-repo-url>
   cd worldkeeper
   ```

2. Install dependencies and build:
   ```bash
   cargo build
   ```

3. Run the game:
   ```bash
   cargo run
   ```

### Development Mode

For faster compile times during development, use the dev feature:

```bash
cargo run --features dev
```

## Project Structure

```
worldkeeper/
├── src/
│   ├── main.rs              # Entry point
│   ├── systems/             # Game systems
│   ├── components/          # ECS components
│   ├── resources/           # Game resources
│   ├── plugins/             # Bevy plugins
│   └── utils/               # Utility functions
├── assets/                  # Game assets (sprites, sounds, etc.)
├── docs/                    # Documentation
├── GAME_DESIGN.md           # Game design document
├── DEVELOPMENT.md           # Development guidelines
└── README.md               # This file
```

## Building and Running

### Debug Build
```bash
cargo build
cargo run
```

### Release Build
```bash
cargo build --release
cargo run --release
```

### Running Tests
```bash
cargo test
```

### Formatting and Linting
```bash
cargo fmt
cargo clippy
```

## Game Features

*This section will be updated as features are implemented*

- [ ] Basic game loop
- [ ] Player character
- [ ] World generation
- [ ] Game mechanics (TBD)

## Documentation

- [Game Design Document](GAME_DESIGN.md) - Overall game design and mechanics
- [Development Guidelines](DEVELOPMENT.md) - Coding standards and best practices
- [Architecture Documentation](docs/architecture.md) - Technical architecture overview

## Contributing

1. Read the [Development Guidelines](DEVELOPMENT.md)
2. Create a feature branch
3. Make your changes
4. Run tests and formatting
5. Submit a pull request

## Resources

### Bevy Learning Resources
- [Bevy Book](https://bevyengine.org/learn/book/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Bevy Discord](https://discord.gg/bevy)

### Rust Learning Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## License

*Add your license information here*

## Changelog

### Version 0.1.0
- Initial project setup
- Added Bevy dependency
- Created project structure