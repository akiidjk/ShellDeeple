# ShellDeeple 🔁


## Introduction

ShellDeeple is a Rust script designed to provide simple, direct access to the DeepL API for instant translation directly from the command line. Using Rust's powerful features to ensure high performance and secure data handling, ShellDeeple allows users to translate texts in various languages supported by DeepL with a simple command. Integrated into the terminal, this tool is ideal for developers, translators and professionals who need quick translations when working on multilingual projects. With minimal configuration and intuitive syntax, ShellDeeple makes instant translation more accessible, efficient and integrated into the daily workflow.

## Platform

- Linux
- Windows only
- MacOS (soon)

## Installation

1. Obtain a deeple key api (required)
2. Download the executable
    * **Only for Linux** If you want to use the software anywhere you need to copy or move the executable to the bin directory with this command `sudo mv sd /usr/local/bin/` or `sudo cp sd /usr/local/bin/` **'sd'** is the name of the executable, if you have another command with **sd** I recommend you rename the executable first. 
3. Run the command or the executable with `sudo sd -i -k {key of deeple (without the staples)}`

## Use and example

**! All the staples are only examples not to insert**!


### Example 1
The tool is very easy to use, the basic usage is

- `sd -t {text}`.

So this will simply translate the text into English using the autodetect of the source language.

### Example 2

You can specify the source language and the target language with

- `-s {source language}`
- `o {target language}`.

The languages are written in short form, and the list of all available languages and the explanation of all commands can be obtained with
`sd --help`
