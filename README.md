## Rust file copy program

Translations available: [English](README.md), [中文](README_cn.md)

  This program copies a file from a source path to a destination path. It takes two arguments: source path and destination path. The program uses Rust version **1.67.1**.

  ### Usage

  To use the program, run the following command in your terminal:

  ```shell
  cargo run <source_path> <destination_path>
  ```

  Example:

  ```shell
  cargo run ./source/file.txt ./destination/
  ```

  ### Handling spaces in destination path

  The program supports destination paths with spaces by wrapping the destination path in quotes. Example:

  ```shell
  cargo run ./source/file.txt "./destination with spaces/"
  ```

  ### Creating the destination directory

  If the destination directory does not exist, the program will create it before copying the file.

  ### Error handling

  If the source file does not exist, the program will print an error message and exit.

  If an error occurs during copying, the program will print an error message and exit.

  ### Future improvements

  - Add support for copying entire directories.
  - Add options for preserving file metadata during copying.