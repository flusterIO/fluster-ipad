# Conundrum

> A purpose driven DSL with mdx like syntax and a modular toolkit for all things academic and note-taking related.


## Packages

Be aware... this is all still a work in progress. Conundrum powers [Fluster](https://flusterapp.com) and the Fluster blog, but these auxillary tools have so far been built with the goal of releasing Fluster in mind. Some of the additional features necessary to connect the dots are in various stages of completeness, but if you think the project and the goals are something you'd like to be a [part of](https://flusterapp.com/blog/by_path/conundrum_goals_and_license), I welcome all the help I can get ;).

### Conundrum

This is the language that powers [Fluster](https://flusterapp.com), and the primary input type that Conundrum uses. With Conundrum, you can take advantage of the more complex components supported by mdx (embedded videos, 3d-images, etc..) when the application rendering the content supports it, and then render to a simplified markdown or plain-text format when the application doesn't support rendering the complete AST.

The goal with Conundrum is to support every component a student or academic needs, and nothing more. While CSS and Javascript collectively manipulate hundreds of properties to build the modern web, Conundrum will offer _just_ what is needed to customize the input, and everything else will be handled by default with the ability to override these defaults through a set of _high-level_ properties accessible to those that never plan on becoming full-time software developers. With the included embedded documentation baked right into the language parser, each component should be discoverable and usable to anyone, **without** any previous coding background of any kind.

These are the components I'm planning for version 1:

- [x] Image
- [x] Tabs component
- [x] Card
- [x] Grid
- [x] Hr
  - There's actually an additional horizontal rule that accepts children for labeling sections and what-not that can be embedded using the `--- My children text ---` syntax.
- [x] Admonition
  - I honestly don't even know what an admonition is. The name comes from Jupyter, but it's the colored card that makes the content stand out. See the fluster website blog for an example.
- [x] Hint
  - A subtle text label that, you know... indicates a hint, although the text and color can be changed to something completely arbitrary.
- [x] Underline (Ul)
- [x] Highlight (Hl)
- [x] Emoji
  - Embed emojis at text scale using the `:smile:` syntax, or at any scale using the `<Emoji name="smile" large />` syntax.
- [x] Quote
  - Embed a styled quote. Not the block quote, but something that's styled to indicate a quote in a book or something similar.
- [x] EqRef
  - Reference equations by id (the `$$ {#myIdHere} ...$$` syntax), so you don't have to count the equation number.
- [x] Image
  - Embed an image that's completely customizable as a regular component, instead of using the very limited markdown syntax.
- [x] Color
  - Embed a single color, a color pair, a light/dark pair, or a pair of these pairs in a component for use in UI related workflows.
- [x] Container
  - An unstyled container that can be customized to fit the user's needs.
- [ ] Video
  - Play local videos or videos via a html src string.
- [ ] Youtube
  - Play videos directly from youtube.

As Conundrum is just a superset of markdown with mdx like syntax, you can use your markdown notes as is, and when it's time to export your notes to another application, Conundrum compiles the components away into customizable markdown blocks. That's actually how Fluster renders content in Swift, with Swift's very limited markdown support.

### Database

Powered by LanceDB, the database package acts as a unified tabular/vector store for everything a modern research or student might need. There's tables for things ranging from homework assignments to experimental results of recent research, and it's **structured**... designed for consumption by both humans _and_ AI.

The goals with the database package is to create a shared academic datalayer for users of Conundrum, while making integration with other applications as seamless as possible. I can imagine many users having a use case only for the CLI or the Rust library if they plan to use this for documentation or a blog, but the database is really the core component for user's that intend to use Conundrum as a sort of pseudo-framework; as something to build a knowledge base around.

#### On My Love/Hate Relationship With Vibe-Coding

As much as I hate vibe-coding as a fanciful profession, this platform would be the perfect ecosystem to vibe code upon. Let developers handled the data layer, and vibe-code away your own front-end. As the documentation comes together this will be increasingly straight-forward as AI becomes familiar with the architecture. As soon as I can afford an internet connection I'll move the Conundrum ecosystem to it's own monorepo, and begin to focus on this documentation.

### CLI

So far, this is very much a work in progress, but the goals are:

- [x] Render project
  - Renders a project based on the provided configuration. This powers the Fluster blog using Next.js, but pretty much any other framework that can consume json will work.
- [x] Watch project
  - [ ] Render project incrementally
- [ ] TUI with:
  - [ ] Search through your notes via:
    - [ ] Full-Text Search
    - [ ] Vector Search
    - [ ] Tag
    - [ ] Topic
    - [ ] Subject
    - [ ] Citation
- [ ] Render to stdout as markdown

### API's

Thanks to the awesome Rust FFI ecosystem, creating type-safe libraries in a variety of languages is very little additional work. So far, these libraries are in the works:

- [x] Rust, obviously
- [x] Typescript (Powers Fluster's webviews)
- [ ] Python (coming together with the database package)
- [ ] Go
- [ ] Lua, for those sweet vim plugins.

And planned for the future, aka langauges I can't write:
- [ ] Kotlin
- [ ] C#
