# Files to Prompt

**Files to Prompt** is a command-line utility that scans a directory tree and concatenates files into a single output file.  
It’s especially useful for preparing codebases, datasets, or text collections to feed into **LLMs (Large Language Models)** for analysis, prompting, or fine-tuning.

This project is a Rust rewrite of [simonw/files-to-prompt](https://github.com/simonw/files-to-prompt), built for learning purposes and performance improvements.

---

## Features

- Traverse directories recursively and combine file contents into one file.
- Filter by file **extension** (e.g., only `.rs` and `.ts`).
- Exclude files and directories with **ignore patterns** (supports globs).
- Optionally **include hidden files/folders** (dotfiles).
- Export as plain text or as **Markdown fenced code blocks**.
- Add **line numbers** to file contents for reference.
- Output to a custom file (defaults to `output.txt`).

---

## Usage

```bash
./files-to-prompt [OPTIONS] <PATHS>...
```

### Options

1. **Filter by extensions**

   Only include files with specific extensions:

   ```bash
   ./files-to-prompt -e rs ts
   ```

   This will include only `.rs` and `.ts` files.

2. **Include hidden files**

   ```bash
   ./files-to-prompt --include-hidden
   ```

3. **Ignore patterns**

   Exclude files or directories by glob patterns:

   ```bash
   ./files-to-prompt -i node_modules -i target
   ```

4. **Markdown output**

   Wrap files in Markdown fenced code blocks:

   ```bash
   ./files-to-prompt -m
   ```

5. **Custom output file**

   ```bash
   ./files-to-prompt -o combined.md
   ```

   Default: `output.txt`

6. **Add line numbers**

   ```bash
   ./files-to-prompt -n
   ```

---

## Examples

Concatenate all Rust source files into a Markdown file with line numbers:

```bash
./files-to-prompt -e rs -m -n -o codebase.md ./src
```

Include all files, but ignore common build directories:

```bash
./files-to-prompt -i node_modules -i target -i .git ./project
```
---

## License

MIT License
