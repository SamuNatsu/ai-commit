# AI Commit

A Command Line Utility for AI Generating Git Commit Message

## Install

### Use Cargo Install

```sh
cargo install --git https://github.com/SamuNatsu/ai-commit
```

### Use CLI (Linux/MacOS only)

```sh
curl -s https://installer.samunatsu.workers.dev/SamuNatsu/ai-commit | bash
```

## Environment Variables

Some environment variables must be provided, dotenv files also can be used to provide such variables.

|         Name         | Introduction                                      | Required |              Example              |                    Default                     |
| :------------------: | :------------------------------------------------ | :------: | :-------------------------------: | :--------------------------------------------: |
| `AI_COMMIT_ENDPOINT` | OpenAI API style endpoint url                     |   Yes    |    `https://api.deepseek.com/`    |                       -                        |
| `AI_COMMIT_API_KEY`  | OpenAI API style API key                          |   Yes    |       `sk-xxxxxxxxxxxxxxx`        |                       -                        |
|  `AI_COMMIT_MODEL`   | OpenAI API style model name                       |   Yes    |        `deepseek-reasoner`        |                       -                        |
|  `AI_COMMIT_FILTER`  | A RegExp file name filter for ignoring their diff |    No    | `package-lock\.json\| yarn\.lock` | See [here](./src/includes/default_filters.txt) |

## Usage

### TL; DR

```txt
A Command Line Utility for AI Generating Git Commit Message

Usage: ai-commit [OPTIONS]

Options:
  -v, --verbose                      Show verbose message
  -d, --dotenv <DOTENV>              Dotenv profile name
  -t, --commit-type <COMMIT_TYPE>    Force using the given commit type
  -s, --commit-scope <COMMIT_SCOPE>  Force using the given commit scope
  -p, --prompt <PROMPT>              Additional prompt message
  -h, --help                         Print help
  -V, --version                      Print version
```

## Dotenv profiles

You can use different dotenv file by passing the `-d|--dotenv` option.  
Command below will use `.env.openai` as profile:

```sh
ai-commit -d openai
```

## Additional message

You can add a custom prompt message for AI model, makes it better understands your demands.

```sh
ai-commit -p "This commit is for optimization"
```

## Sponsor

<a href="https://www.buymeacoffee.com/snrainiar"><img src="https://img.buymeacoffee.com/button-api/?text=Buy me a coffee&emoji=&slug=snrainiar&button_colour=FF5F5F&font_colour=ffffff&font_family=Cookie&outline_colour=000000&coffee_colour=FFDD00" /></a>
