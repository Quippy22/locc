# locc (Line-of-code Counter)

A lightning-fast, simple CLI tool to count lines of code in your projects. It respects your `.gitignore` files automatically, so you don't waste time counting dependencies or build artifacts.

## Features
- **Smart Traversal:** Respects `.gitignore` and global gitignore settings.
- **Fast:** Sequential I/O using buffered reading for optimal speed.
- **Metrics:** Language breakdown with percentage summaries.
- **Simple:** No complex async or heavy dependencies.

## Usage
Basic usage from any directory:
```bash
locc
```
Or provide a specific path:
```bash
locc ~/Path/to/my-project
```

## License
MIT
