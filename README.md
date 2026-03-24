# cram-zed

A [Zed](https://zed.dev) extension that adds support for [cram](https://bitheap.org/cram/) test files (`.t`).

Cram is a functional testing framework for command-line applications. Tests are written as snippets of shell session — commands prefixed with ` $` and their expected output — making them readable and easy to maintain.

## Features

- Syntax highlighting for cram test files
- Shell language injection in command lines
- Language-specific injection in output blocks via `< language:` specifiers

## Grammar

Uses [tree-sitter-cram](https://github.com/tjdevries/tree-sitter-cram) by TJ DeVries.

## Install

This extension is not published in Zed's extension marketplace currently. Clone this repo and [install as a dev extension](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally).
