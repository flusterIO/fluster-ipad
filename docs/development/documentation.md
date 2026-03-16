# Documentation Documentation

Any documents placed in the `docs/in_content_docs/` directory will be copied to the embedded folder in the `packages/rust/conundrum/src/embedded/in_content_docs/` directory.

Each input file is parsed for the following syntax:

````
```long-docs
This is my 'long docs only' content here.
```
````

This will of course over write any files that match the file name with out the `-full` or `-short` suffix in the output directory, and because there's not really a way to leave a comment, it can't be documented directly in the file.
