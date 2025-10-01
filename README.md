# Files to prompt

A CLI tool to concatenate given directory into single file for use with LLMs.

### Options

1. `-e / --extension`: Only include files with given extension.

```bash
    ./files-to-prompt -e rs ts
```

2. `--include-hidden`: Include files and folders starting with `.`

3. `-i / --ignore pattern`: Specify one or more pattern to ignore.

```
    ./files-to-prompt -i node_modules
```

4. `-m / --markdown`: Output as markdown fenced code blocks.

5. `-o / --output`: Define output filename. Default `output.txt`

6. `-n / --line-number`: Include line numbers of the file.

--
This project is a rust rewrite of [files-to-prompt](https://github.com/simonw/files-to-prompt) for learning purpose.
