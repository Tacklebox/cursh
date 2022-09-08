# Cursh

* Note: At this point cursh is a low effort tool for an unusual use case quickly hacked together. Unless anyone finds it actually usefull it will probably stay that way.

## What is it?

If a program expects files as arguments and you want to use your system clipboard as the input instead, you can leave a placeholder in the command and cursh passes your clipboard contents as a temporary file to the command.

## Installation

#### Using Cargo

```sh
cargo install cursh
```

#### From Source (still using cargo)

```sh
git clone https://github.com/tacklebox/cursh.git
cd cursh
cargo install --path .
```

## Usage Example

```sh
cursh diff {{1}} {{2}}
```

Then copy some text, return to the terminal you ran cursh in, and hit enter. That text is now a temporary file in the position of the placeholder {{1}}.
Next, Copy what you would like to diff the previous texts against, return to the terminal and hit enter to execute the diff command and see the output.
