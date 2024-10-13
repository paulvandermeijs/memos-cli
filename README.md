# Memos CLI

A command-line tool for [Memos](https://www.usememos.com/).

## Installation

Use Cargo to install the application.

```bash
cargo install memos-cli
```

## Usage

```bash
memo [OPTIONS] [COMMAND]
```

### Login

To enable all features you must first log in to your Memos server. Use the
following command to log into your server:

```bash
memo login
```

### Create a new memo

Once logged in you can create a new memo by running the command without any
options.

```bash
memo
```

This will open your default editor as defined in `VISUAL` or `EDITOR`.

> [!IMPORTANT]  
> Make sure you save the memo before closing the editor.

After closing the editor your memo will be saved to your server.

#### Visibility

By default your memo's visibility will be set to "Private". You can set the
visibility to "Workspace" using the following option:

```bash
memo --workspace
```

Or "Public" using this option:

```bash
memo --public
```

#### Reading input from `STDIN`

It is also possible to set the content for the memo from `STDIN`. To do this
just pipe your content into the command:

```bash
echo "Hello" | memo
```

#### Skipping the editor

When using content from `STDIN` you might not need to use the editor. To skip
editing use the following option:

```bash
memo --no-edit
```

### List memos

To display a list of all your memos use the following command:

```bash
memo list
```

Use `j` and `k` to navigate down and up the list. Press `q` to exit the list.

## Known issues

- When using Visual Studio Code as `VISUAL` it needs to wait for the file to be
  closed before returning. To make this happen you need to add the `--wait`
  option to the `code` command.
- The application uses an API client generated from the API specification. This
  may in some cases result in memos not being displayed due to missing
  definitions in the specification.
