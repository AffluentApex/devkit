# DevKit 🛠️

A comprehensive developer toolkit for project analysis and management.

Created with ❤️ by GlidedApex | December 18, 2024

## ✨ Features

- **Git Analysis**: Analyze repository statistics including commit count and branch information
- **Code Complexity**: Calculate complexity metrics for your codebase
- **Dependency Analysis**: Analyze project dependencies for Rust and Node.js projects
- **Project Templates**: Quickly scaffold new projects with best practices
- **Environment Check**: Verify your development environment setup

## 🚀 Installation

```bash
cargo install devkit
```

## 📖 Usage

```bash
# Show help and available commands
devkit --help

# Check your development environment
devkit check
# Example output:
# ✓ git (git version 2.45.2)
# ✓ rustc (rustc 1.83.0)
# ✓ python (Python 3.12.3)
# ✗ node not found

# Analyze code complexity
devkit complexity src
# Example output:
# Files analyzed: 5
# Total lines: 504
# Code lines: 427
# Comment lines: 17
# Blank lines: 60

# Analyze a git repository
devkit git .
# Example output:
# Total commits: 42
# Branch count: 3
# Latest commit by: JohnDoe
# Time: 2 hours ago

# Analyze project dependencies
devkit dependencies .
# Example output:
# Total dependencies: 12

# Create a new project
devkit new myproject rust
```

## 🌟 Example Use Cases

1. **Development Environment Setup**
   ```bash
   # Check if all required tools are installed
   devkit check
   ```

2. **Code Analysis**
   ```bash
   # Analyze your src directory
   devkit complexity src
   
   # Get detailed git statistics
   devkit git .
   ```

3. **Project Management**
   ```bash
   # Check project dependencies
   devkit dependencies .
   
   # Start a new Rust project
   devkit new myapp rust
   ```

4. **Analyzing External Repositories**
   ```bash
   # Step 1: Clone the repository
   git clone https://github.com/username/repository
   
   # Step 2: Navigate to the repository
   cd repository
   
   # Step 3: Run complete analysis
   devkit check                 # Check environment requirements
   devkit git .                 # Analyze git history and stats
   devkit complexity src       # Analyze code complexity
   devkit dependencies .       # Check project dependencies
   
   # Example for analyzing a specific repository:
   git clone https://github.com/AffluentApex/super-duper-doodle
   cd super-duper-doodle
   devkit complexity src      # Analyze code structure and complexity
   devkit git .              # Get repository statistics
   ```

## 💡 Tips and Tricks

1. **Analyzing External Repositories**
   - Always clone repositories before analysis
   - Use `devkit git .` to get repository insights
   - The complexity analyzer works with any programming language
   - Dependencies analysis works best with Rust and Node.js projects

2. **Best Practices**
   - Run `devkit check` before starting analysis
   - Use specific paths with complexity analysis for better insights
   - Keep your development tools updated for accurate environment checks

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---
Made with ❤️ and 🦀 Rust | © 2024 GlidedApex

> "Code is poetry in motion, and every developer is a poet." - GlidedApex
