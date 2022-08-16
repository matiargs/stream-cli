# Stream Cli
Simple cli tool to process thoughts and links and save them to a markdown file with a predefined format.

This tool it's a working prototype, i'm trying to find a simple way to save thoughts in a file without much hurdle.

This project was used to learn a bit about Rust. It's been a funny experience.

## Format

`2022-06-14 10:18:23 - <text or url>`

## Usage
This program need a env variable with a PATH to the markdown file where to save the thoughts and links. You can export directly into your `.bashrc` or `.zshrc` file with:
```bash
export STREAM_FILE_PATH=/home/user/path/to/file
```
or you can run the program with the variable:
```bash
STREAM_FILE_PATH=/home/user/path/to/file stream-cli -t "text"
```
This tools can process thoughts (text) or links (text). When processing links, it auto generate the correct markdown syntax `[text](url)`

To process thoughts:

`stream-cli -t "thought"`

To process links:

`stream-cli -u "link"`