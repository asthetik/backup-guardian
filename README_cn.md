## Rust文件复制程序

Translations available: [English](README.md), [中文](README_cn.md)

该程序将文件从源路径复制到目标路径。它有两个参数：源路径和目标路径。 该程序使用 Rust 版本 **1.67.1**。

### 用法

要使用该程序，请在终端中运行以下命令：

```shell
cargo run <source_path> <destination_path>
```

例子：

```shell
cargo run ./source/file.txt ./destination/
```

### 处理目标路径中的空格

该程序通过将目标路径用引号引起来支持带有空格的目标路径。例子：

```shell
cargo run ./source/file.txt "./destination with spaces/"
```

### 创建目标目录

如果目标目录不存在，程序将在复制文件之前创建它。

### 错误处理

如果源文件不存在，程序将打印错误信息并退出。

如果在复制过程中发生错误，程序将打印错误信息并退出。

### 未来的改进

- 添加对复制整个目录的支持。
- 添加在复制过程中保留文件元数据的选项。