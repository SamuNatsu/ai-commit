# AI Commit

A Command Line Utility for AI Generating Git Commit Message

## Install

Use `cargo install` for automatically compiling and installing.

```sh
cargo install --git https://github.com/SamuNatsu/ai-commit
```

## Environment Variables

Some environment variables must be provided, a `.env` file also can be used to provide such variables.

|          Name           | Introduction                                      | Required |              Example              |                    Default                     |
| :---------------------: | :------------------------------------------------ | :------: | :-------------------------------: | :--------------------------------------------: |
|  `AI_COMMIT_ENDPOINT`   | OpenAI API style endpoint url                     |   Yes    |    `https://api.deepseek.com/`    |                       -                        |
|   `AI_COMMIT_API_KEY`   | OpenAI API style API key                          |   Yes    |       `sk-xxxxxxxxxxxxxxx`        |                       -                        |
|    `AI_COMMIT_MODEL`    | OpenAI API style model name                       |   Yes    |        `deepseek-reasoner`        |                       -                        |
| `AI_COMMIT_SHOW_REASON` | `1` for showing reasoning contents                |    No    |           `1` or other            |                       -                        |
|   `AI_COMMIT_FILTER`    | A RegExp file name filter for ignoring their diff |    No    | `package-lock\.json\| yarn\.lock` | See [here](./src/includes/default_filters.txt) |
