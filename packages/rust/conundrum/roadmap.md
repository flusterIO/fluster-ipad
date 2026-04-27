# Conundrum Goals & Roadmap

- Provide a general user friendly markup language capable of displaying more complex components usable in a note taking context (video/image components, embedded 3d models, interactive tables, etc.)
- Create a unified memory layer that can be appended to by nested languages in each cell's code block. This will likely be one of the last features to make it to production because of it's complexity and the security concerns, but would be an invaluable tool to researchers with a background in multiple languages.
- Create a **simple** programming layer, exposing Conundrum specific functionality in Conundrum code blocks. Things like a first-party AI module with structured generation support for new developers, some simple **high level** file utilities, 





### Missing Syntaxes

- [ ] The ability to link to timestamps in videos and audio files using the syntax `[My link](video:videoId@2:30)` or `[My link](audio:audioId@01:15:20)`
- [ ] The ability to place lists in a code block, making them interactive. This would be made possible by a required id on the code block, allowing the compiler to apply unique dom attributes to each list item at compilation time.
- [ ] A multi-column table syntax to collapse a table column and pass it's width on to the cell on either the right or the left by creating a cell with no white space and `|>|` or `|<|` to pass the cell's width to the right or left respectively.
