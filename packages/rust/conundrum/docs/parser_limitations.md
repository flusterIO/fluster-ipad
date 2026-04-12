# Parser Limitations

1. Inline mdx content cannot have access to global scope variables the same way block level markdown can due to the way nested parsers are handled, requiring that their children be rendered ahead of time.
