
## Conundrum: The Data Layer

This crate is part of the [Fluster](https://flusterapp.com) and Conundrum ecosystem. Conundrum
is a Rust powered note taking DSL built on top of mdx syntax with zero
javascript dependencies, and Fluster is the Apple (iPad and Mac) front-end
for said language. In short: It's markdown, but better.

This language came to be over the course of my own academic pursuits, and as
the language grew, so too did the tooling around it. For this reason I'm
releasing a modular academic toolkit: A combination of pieces that come
together to form a complete knowledge management system that can be
independently replaced or expanded upon as the user requires.

I imagine most 'vibe-coders' will want to keep the data layer as their
primary source of truth, as that's kind of the heart of the 'framework'
aspect of Conundrum, but Conundrum is entirely usable on it's own via the
CLI, the Rust library, or any of the SDK's in active development.

The ecosystem, when used in it's entirety as a note taking framework (it
also powers the Fluster blog) includes a completely local LanceDB vector
store, an MCP/rspc server with a growing list of AI accessible tools ready
to query, modify and create new content as the user requests, a CLI and of
course, Fluster (paid).

### Vibe-Coders

If you're here looking to vibe-code your personal application on top of
Conundrum, checkout the [models](crate::vector::models) module, as that
module describes the structure of the data layer that your front-end will be
able to pull from. Everything from the results of the latest cutting edge
experiment to the properties of your personal pet (yea, like your dog) have
a structured representation in this database, all structured in a way which
encourages both graph and vector similarity retrieval methods. The criteria
to qualify for an independent model in the database was simple: _Are you
important to the life of a modern academic or student?_ If the answer is
yes, AI is given an explicit, structured model to describe this entity.

If you're just starting your vibe-coded Conundrum project, take a look at
the `conundrum_server_rs` crate as that sort of connects the dots, binding
the data layer described above with the front-end of your design. The
[parameters](crate::vector::parameters) module describes the arguments AI,
and your application must provide to the server to retrieve the relevant
information, and as the Conundrum ecosystem becomes more well established,
AI should become better trained on this documentation and be
able to reliably generate Conundrum based projects in just a handful of
prompts, if you're starting from a boilerplate template (work in progress).

Current version: 0.0.1

License: [Open-Source, but with a purpose](https://flusterapp.com/license)
