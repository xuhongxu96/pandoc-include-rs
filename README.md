# pandoc-include-rs

A pandoc filter to replace `!include src_file` in code blocks written in rust. 

Inspired by [pandoc-include](https://github.com/DCsunset/pandoc-include). Less features but much faster than it.

## Limitations

- Only support `!include` in CodeBlock
- Only support `startLine` and `endLine` attributes

## Usage

```
!include source_file_path
```

```
!include`startLine=1, endLine=10` source_file_path
```

Specify parent directory of the source files in `include-entry` metadata.
