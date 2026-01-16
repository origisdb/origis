<div align="center">

# 🌿 Origis

**Git-like version control for your databases.**

[![CI](https://github.com/origisdb/origis/actions/workflows/ci.yml/badge.svg)](https://github.com/origisdb/origis/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Discord](https://img.shields.io/discord/1234567890?color=7289da&label=Discord&logo=discord&logoColor=white)](https://discord.gg/SwBDTRpwa7)

*Snapshots. Branches. History. For SQLite and PostgreSQL.*

[Getting Started](#-getting-started) •
[Features](#-features) •
[Documentation](#-documentation) •
[Contributing](#-contributing) •
[Discord](https://discord.gg/SwBDTRpwa7)

</div>

---

## 🎯 What is Origis?

Origis brings the power of version control to your databases. Just like Git tracks changes in your code, Origis tracks changes in your data.

```bash
# Initialize Origis in your project
$ origis init
✅ Origis initialized in ./.origis
📁 Detected databases: ./app.db

# Create a snapshot before a risky migration
$ origis snapshot -m "Before adding users table"
✅ Snapshot created: a1b2c3d

# Something went wrong? Restore instantly
$ origis restore a1b2c3d
✅ Database restored to snapshot a1b2c3d
```

**No more:**
- 🚫 `database_backup_final_FINAL_v2.sql`
- 🚫 Corrupted data with no way back
- 🚫 "It worked on my machine" data issues
- 🚫 Fear of running migrations

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 📸 **Snapshots** | Capture your database state at any point in time |
| 🌿 **Branches** | Experiment with data without affecting production |
| 🔄 **Diff** | See exactly what changed between two versions |
| ⏪ **Restore** | Roll back to any previous state instantly |
| 📋 **History** | Browse your complete database history |
| 🔌 **Multi-DB** | Works with SQLite today, PostgreSQL coming soon |

### Why Origis?

| | Origis | Manual Backups | Dolt | lakeFS |
|---|--------|----------------|------|--------|
| **Setup** | One binary, zero config | Scripts everywhere | Docker + infra | Kubernetes |
| **Learning curve** | Git-like commands | Varies | New SQL dialect | Object storage concepts |
| **Use your existing DB** | ✅ Yes | ✅ Yes | ❌ Dolt DB only | ❌ S3 only |
| **Target audience** | Developers | Everyone | Enterprise | Data teams |

---

## 🚀 Getting Started

### Installation

#### From source (recommended for now)

```bash
# Clone the repository
git clone https://github.com/origisdb/origis.git
cd origis

# Build with Cargo
cargo build --release

# The binary is in ./target/release/origis
```

#### Via Cargo (coming soon)

```bash
cargo install origis
```

### Quick Start

```bash
# 1. Navigate to your project with a SQLite database
cd my-project

# 2. Initialize Origis
origis init

# 3. Create your first snapshot
origis snapshot -m "Initial state"

# 4. Make changes to your database...

# 5. Create another snapshot
origis snapshot -m "Added users table"

# 6. View history
origis log

# 7. Restore if needed
origis restore <snapshot-id>
```

---

## 📖 Documentation

| Document | Description |
|----------|-------------|
| [User Stories](docs/user-stories-mvp.md) | MVP features and roadmap |
| [Contributing](CONTRIBUTING.md) | How to contribute to Origis |
| [Rust Conventions](docs/rust-conventions.md) | Code style and best practices |
| [Commit Conventions](docs/commit-conventions.md) | How to write commit messages |
| [Branch Conventions](docs/branch-conventions.md) | How to name branches |
| [Workflow](docs/workflow-linear.md) | Our development workflow |

---

## 🗺️ Roadmap

### v0.1.0 — Foundation *(current)*
- [x] Project setup
- [ ] `origis init`
- [ ] `origis snapshot`
- [ ] `origis log`
- [ ] `origis restore`

### v0.2.0 — Core Features
- [ ] `origis diff`
- [ ] `origis status`
- [ ] Ignore tables (`.origisignore`)

### v0.3.0 — Branching
- [ ] `origis branch`
- [ ] `origis checkout`
- [ ] `origis merge`
- [ ] PostgreSQL support

### v1.0.0 — Stable Release
- [ ] Production-ready
- [ ] Complete documentation
- [ ] Performance optimizations

---

## 🤝 Contributing

We welcome contributions from everyone! Origis is built by a team of passionate developers learning Rust together.

### Ways to contribute

- 🐛 **Report bugs** — Open an issue
- 💡 **Suggest features** — Start a discussion
- 📝 **Improve docs** — Every typo fix helps
- 🦀 **Write code** — Check our [good first issues](https://github.com/origisdb/origis/labels/good%20first%20issue)

### Development setup

```bash
# Clone the repo
git clone https://github.com/origisdb/origis.git
cd origis

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and test
cargo build
cargo test
cargo clippy
cargo fmt
```

See our [Contributing Guide](CONTRIBUTING.md) for more details.

---

## 💬 Community

- **Discord** — [Join our server](https://discord.gg/SwBDTRpwa7) for discussions and help
- **GitHub Issues** — Bug reports and feature requests
- **GitHub Discussions** — Questions and ideas

---

## 📄 License

Origis is open source software licensed under the [MIT License](LICENSE).

---

<div align="center">

**Made with 🦀 by the Origis Team**

*From the Latin "Origo" — the origin*

</div>
