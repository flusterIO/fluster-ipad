//! # Conundrum
//!
//! ### A modular knowledge toolkit for the vibe-coding era.
//!
//! > **Conundrum is a modular academic toolkit built in the pursuit of
//! > quantized gravity, rebuilt for the vibe-coding era.**
//!
//! Conundrum began as part of [Fluster](https://flusterapp.com), an application born from an attempt to build better tools for doing academic research.
//!
//! It has since evolved into something considerably larger:
//!
//! **a local, programmable second brain designed to make human knowledge
//! structured enough for both people and AI to use.**
//!
//! Conundrum combines a deliberately constrained knowledge language, a
//! graph-oriented local data layer, vector search, AI tooling, MCP, SDKs, a
//! CLI, and a full React application into one modular ecosystem.
//!
//! The goal isn't simply to make a better note-taking application.
//!
//! The goal is to make your knowledge **programmable, portable, searchable, and
//! understandable to machines.**
//!
//! ---
//!
//! # Why Conundrum?
//!
//! Traditional note-taking applications primarily think in terms of documents.
//!
//! Conundrum thinks in terms of **knowledge**.
//!
//! Your notes are still documents. Markdown is still useful. You can still
//! write equations, explanations, references, experiments, and everything else
//! you'd expect from an academic notebook.
//!
//! But underneath those documents is a structured system of entities and
//! relationships.
//!
//! A paper can relate to a concept.
//!
//! A concept can relate to an experiment.
//!
//! An experiment can produce a result.
//!
//! A result can support or contradict a hypothesis.
//!
//! A hypothesis can depend on several equations.
//!
//! A person can be associated with a project.
//!
//! A project can have its own notes.
//!
//! And all of those things can be searched semantically and navigated as a
//! graph.
//!
//! This matters because AI models have a fundamental limitation:
//!
//! **context is finite.**
//!
//! Instead of continually giving an AI your entire knowledge base, Conundrum
//! gives it a way to **navigate your knowledge.**
//!
//! ---
//!
//! # A local second brain
//!
//! Conundrum is designed to keep your knowledge local.
//!
//! The database remains on your machine, with AI being an external service only
//! when you choose to use one.
//!
//! The underlying data layer uses **LanceDB** for local vector storage and
//! retrieval while maintaining a strongly structured, graph-oriented
//! representation of your information.
//!
//! This means your knowledge isn't merely:
//!
//! ```text
//! notes/
//! ├── physics.md
//! ├── mathematics.md
//! ├── research.md
//! └── ideas.md
//! ```
//!
//! It can instead become something closer to:
//!
//! ```text
//!                   ┌─────────────┐
//!                   │   Person    │
//!                   └──────┬──────┘
//!                          │
//!                        studies
//!                          │
//!                          ▼
//!                   ┌─────────────┐
//!                   │   Concept   │
//!                   └──────┬──────┘
//!                          │
//!                     depends_on
//!                          │
//!                          ▼
//!                   ┌─────────────┐
//!                   │   Equation  │
//!                   └──────┬──────┘
//!                          │
//!                      used_in
//!                          │
//!                          ▼
//!                   ┌─────────────┐
//!                   │ Experiment  │
//!                   └──────┬──────┘
//!                          │
//!                       produces
//!                          │
//!                          ▼
//!                   ┌─────────────┐
//!                   │   Result    │
//!                   └─────────────┘
//! ```
//!
//! The exact graph can be much richer than this.
//!
//! That's intentional.
//!
//! ---
//!
//! # Why a graph?
//!
//! Imagine asking:
//!
//! > "What do I know about this experiment?"
//!
//! A traditional search system might find documents containing the experiment's
//! name.
//!
//! A vector database might find documents semantically similar to the question.
//!
//! A graph can additionally tell the system:
//!
//! - Which experiment you're talking about
//! - What hypothesis it was testing
//! - Which concepts it depended on
//! - Which equations were involved
//! - Which results it produced
//! - Which papers informed it
//! - Which other experiments reference it
//! - What notes you've made about it over time
//!
//! Conundrum combines these approaches.
//!
//! **Vector similarity finds relevant things.**
//!
//! **The graph explains how those things relate.**
//!
//! This allows an AI system to retrieve much more information without requiring
//! enormous context windows.
//!
//! ---
//!
//! # Conundrum, the language
//!
//! At the center of the ecosystem is **Conundrum**, a language built on top of
//! MDX.
//!
//! Most MDX documents containing components with compatible names and
//! properties can be made to compile in Conundrum, but Conundrum deliberately
//! removes the dependency on JavaScript.
//!
//! There is **no JavaScript runtime required to understand Conundrum.**
//!
//! The language is compiled completely by Rust and supports WebAssembly
//! targets.
//!
//! That means the same language can participate in both native and
//! browser-based applications while remaining integrated with the rest of the
//! Conundrum ecosystem.
//!
//! ---
//!
//! # Markdown's simplicity, MDX's expressiveness, without JavaScript
//!
//! Markdown is wonderful because it is simple.
//!
//! That simplicity is also its limitation.
//!
//! Markdown can express:
//!
//! ```markdown
//! # A heading
//!
//! This is a paragraph.
//!
//! **This is important.**
//! ```
//!
//! But it doesn't provide much semantic information beyond the formatting
//! itself.
//!
//! MDX adds expressive components, but its relationship with JavaScript makes
//! it difficult to treat the document purely as portable structured
//! information.
//!
//! HTML provides enormous expressive power, but that power comes with a
//! problem.
//!
//! There are often dozens—or hundreds—of ways to communicate essentially the
//! same idea.
//!
//! Conundrum deliberately occupies the middle ground.
//!
//! It aims to be:
//!
//! **more expressive than Markdown, but more deliberate than HTML.**
//!
//! ---
//!
//! # A deliberately constrained vocabulary
//!
//! Conundrum is intentionally opinionated.
//!
//! There shouldn't be ten different ways to express the same semantic
//! component.
//!
//! For example, an admonition shouldn't be expressible through hundreds of
//! arbitrary combinations of HTML, CSS, JavaScript, and component properties.
//!
//! Conundrum wants a deliberate vocabulary.
//!
//! Why?
//!
//! Because the structure is useful to AI.
//!
//! Consider:
//!
//! ```tsx
//! <Admonition type="research">
//!     This result is still speculative.
//! </Admonition>
//! ```
//!
//! The `research` classification isn't merely a styling instruction.
//!
//! It is information.
//!
//! An AI processing the document can understand that this isn't simply a
//! visually highlighted paragraph.
//!
//! It is a particular **kind of knowledge**.
//!
//! Likewise, properties such as:
//!
//! ```text
//! error
//! warning
//! research
//! definition
//! example
//! proof
//! question
//! ```
//!
//! can communicate semantic intent while also controlling presentation.
//!
//! The same information can then survive through different output formats.
//!
//! This is a major part of Conundrum's philosophy:
//!
//! > **The syntax should communicate useful information to both humans and
//! > machines.**
//!
//! ---
//!
//! # One concept, one canonical representation
//!
//! Conundrum will intentionally avoid providing multiple competing ways to
//! express the same semantic component.
//!
//! This might sound restrictive.
//!
//! It is actually one of the most important parts of the language.
//!
//! If there are one hundred ways to create an "admonition," an AI has to reason
//! about one hundred syntaxes that all mean roughly the same thing.
//!
//! If there is one canonical representation, the structure becomes much easier
//! to interpret.
//!
//! That produces a useful property:
//!
//! **semantic density per token increases.**
//!
//! The document contains less accidental complexity and more deliberate
//! information.
//!
//! In an era where AI systems increasingly operate under token and context
//! constraints, this matters.
//!
//! ---
//!
//! # Compile your knowledge to different targets
//!
//! Conundrum isn't tied to one renderer.
//!
//! A document can be compiled to different targets, with components providing
//! both global and instance-specific properties that control their behavior.
//!
//! For example:
//!
//! ```tsx
//! <Hl md="italic">
//!     My highlighted text
//! </Hl>
//! ```
//!
//! A Markdown renderer could use the `md` property to produce:
//!
//! ```markdown
//! *My highlighted text*
//! ```
//!
//! while the same underlying component can produce an entirely different
//! representation for another target.
//!
//! This is particularly useful when a component has information that cannot be
//! represented equally well in every output format.
//!
//! For example, Fluster uses this mechanism when rendering content to Swift,
//! where Markdown support is considerably more limited than on the web.
//!
//! Rather than throwing away information when converting formats, Conundrum
//! lets the component carry the instructions necessary to preserve the
//! important meaning.
//!
//! ---
//!
//! # Supported output targets
//!
//! Conundrum currently targets multiple output formats, with additional targets
//! under development.
//!
//! - [x] HTML
//! - [x] JSX
//! - [ ] CommonMark-compatible Markdown — approximately 90% complete
//! - [ ] JSON AST output for rendering from languages such as Swift and Kotlin
//!
//! The architecture also supports customization flags for controlling things
//! such as emoji visibility, embedded JavaScript, and single-file output.
//!
//! The goal is not simply to render Conundrum documents.
//!
//! It is to make the **same structured knowledge portable across
//! environments.**
//!
//! ---
//!
//! # Built completely in Rust
//!
//! Conundrum's language implementation is powered completely by Rust.
//!
//! The core can run:
//!
//! - Natively
//! - In WebAssembly
//! - Inside the desktop application
//! - In browser environments
//! - As part of other Rust applications
//! - Through language bindings and SDKs
//!
//! There is no TypeScript or JavaScript implementation of the language hiding
//! underneath the abstraction.
//!
//! JavaScript and TypeScript remain important parts of the broader
//! ecosystem—particularly for the React frontend and TypeScript SDK—but the
//! language itself is Rust-native.
//!
//! This gives Conundrum a single core implementation across platforms.
//!
//! ---
//!
//! # From Fluster to Conundrum
//!
//! Conundrum came out of **Fluster**.
//!
//! Fluster began after its creator left a job almost five years ago to pursue
//! astrophysics full-time.
//!
//! The project has been rewritten repeatedly since then.
//!
//! What began as an application for academic work has gradually evolved into a
//! general-purpose knowledge infrastructure.
//!
//! The current architecture is another major rewrite—this time with the
//! **vibe-coding era** in mind.
//!
//! The question has changed from:
//!
//! > "How do I build the perfect application?"
//!
//! to:
//!
//! > "How do I build a toolkit that makes it easy for humans and AI to build
//! > the applications they actually need?"
//!
//! That distinction drives much of Conundrum's architecture.
//!
//! ---
//!
//! # More than academic notes
//!
//! Conundrum is heavily focused on academic and STEM knowledge.
//!
//! But the underlying data model is intentionally much broader.
//!
//! Learning a structured knowledge language naturally creates a temptation:
//!
//! > "Why can't I use this for everything?"
//!
//! Conundrum embraces that.
//!
//! The database contains models for things ranging from scientific experiments
//! to everyday entities such as a family pet.
//!
//! A knowledge system shouldn't have to decide that something isn't important
//! simply because it isn't academic.
//!
//! Your life is a graph too.
//!
//! ---
//!
//! # Everything can have a notepad
//!
//! Many Conundrum entities can have their own associated notes.
//!
//! This provides something closer to a persistent contextual memory for
//! individual entities.
//!
//! For example, a `Pet` might have:
//!
//! ```text
//! Pet
//! ├── name
//! ├── species
//! ├── birthday
//! ├── veterinarian
//! ├── medications
//! └── notes
//! ```
//!
//! An `Experiment` might have:
//!
//! ```text
//! Experiment
//! ├── hypothesis
//! ├── methodology
//! ├── variables
//! ├── results
//! ├── references
//! └── notes
//! ```
//!
//! Those notes can evolve over time.
//!
//! This becomes particularly interesting when AI is involved.
//!
//! Instead of an AI repeatedly reconstructing what it knows about an entity
//! from an enormous collection of documents, the entity itself can accumulate
//! relevant context.
//!
//! The system can therefore maintain information **about specific things**, not
//! just information contained inside documents.
//!
//! ---
//!
//! # AI without the context-window problem
//!
//! This architecture is designed around a simple observation:
//!
//! **Your AI shouldn't need to read your entire life to answer one question
//! about your life.**
//!
//! Suppose your knowledge base contains ten thousand documents.
//!
//! A naive AI application might attempt to solve the problem by increasing the
//! context window.
//!
//! Conundrum instead lets the AI:
//!
//! ```text
//! Question
//!    ↓
//! Semantic search
//!    ↓
//! Relevant entities
//!    ↓
//! Graph relationships
//!    ↓
//! Related documents
//!    ↓
//! Entity-specific notes
//!    ↓
//! Focused context
//!    ↓
//! Answer
//! ```
//!
//! The result is a smaller and more relevant context.
//!
//! This can mean:
//!
//! - Lower token usage
//! - Better relevance
//! - Less context pollution
//! - More explainable retrieval
//! - More useful relationships
//! - Better scalability as your knowledge grows
//!
//! The model doesn't need to memorize your knowledge.
//!
//! It needs to know **how to navigate it.**
//!
//! ---
//!
//! # Model Context Protocol
//!
//! Conundrum includes an integrated **Model Context Protocol (MCP)** server.
//!
//! MCP gives AI applications a standardized interface for interacting with
//! external tools and data.
//!
//! Conundrum uses that capability to expose your knowledge system to AI agents.
//!
//! The MCP toolset is growing and is intended to provide operations such as:
//!
//! - Searching knowledge
//! - Retrieving documents
//! - Navigating relationships
//! - Working with structured entities
//! - Querying local data
//! - Working with entity-specific notes
//! - Performing knowledge operations
//! - Extending the system with additional tools
//!
//! This means Conundrum isn't simply an application that happens to have AI
//! features.
//!
//! It can become infrastructure that **AI applications build upon.**
//!
//! ---
//!
//! # Bring your own model
//!
//! Conundrum's knowledge layer is intentionally independent of the AI provider.
//!
//! You can use hosted models such as OpenAI.
//!
//! You can use local models through Ollama.
//!
//! You can use different models for different tasks.
//!
//! You can replace models without replacing your knowledge base.
//!
//! This separation is important.
//!
//! **The model is replaceable. Your knowledge isn't.**
//!
//! ---
//!
//! # LanceDB + graph-oriented data
//!
//! Conundrum combines structured relationships with semantic vector search.
//!
//! LanceDB provides the local vector layer, while Conundrum's data model
//! provides the broader structure around those vectors.
//!
//! This gives AI systems two fundamentally different ways of finding
//! information.
//!
//! ### Similarity
//!
//! > "Find things that mean something similar to this."
//!
//! ### Relationships
//!
//! > "Find things connected to this."
//!
//! Those questions are not interchangeable.
//!
//! Together they are much more powerful.
//!
//! ---
//!
//! # Designed for vibe-coding
//!
//! Conundrum is intentionally modular.
//!
//! You shouldn't have to understand the entire codebase to build something
//! useful with it.
//!
//! The architecture is designed so that AI-assisted development can happen **on
//! top of the toolkit.**
//!
//! For example:
//!
//! > "Build me a tool that finds concepts I haven't reviewed recently and
//! > generates a study plan based on their prerequisite relationships."
//!
//! The goal is that you don't need to first build:
//!
//! - A database
//! - A graph layer
//! - An embedding pipeline
//! - A retrieval system
//! - An MCP server
//! - Authentication between components
//! - A document parser
//! - A knowledge API
//!
//! Those capabilities already exist.
//!
//! You build the thing you actually want.
//!
//! This is what Conundrum means by **vibe-coding on top of the platform.**
//!
//! ---
//!
//! # SDKs
//!
//! Conundrum is intended to be accessible from the languages developers already
//! use.
//!
//! SDKs are available or under active development for:
//!
//! | Language | Status |
//! |---|---|
//! | Rust | 🟢 Core |
//! | TypeScript | 🟢 Active |
//! | Go | 🟡 Developing |
//! | Python | 🟡 Developing |
//! | Lua | 🟡 Developing |
//! | Swift | 🟡 Developing |
//!
//! The SDKs are not intended to turn Conundrum into six unrelated
//! implementations.
//!
//! The Rust-powered core remains the source of truth wherever practical.
//!
//! ---
//!
//! # CLI
//!
//! Conundrum also includes a CLI for working with the system outside of the
//! graphical application.
//!
//! This makes it possible to integrate Conundrum into:
//!
//! - Shell scripts
//! - Development workflows
//! - Research workflows
//! - Automation
//! - CI/CD
//! - AI agents
//! - Custom tooling
//!
//! The application is an interface to the system—not the system itself.
//!
//! ---
//!
//! # React frontend
//!
//! Conundrum includes a full React frontend.
//!
//! React provides a familiar and flexible interface for interacting with the
//! knowledge system while remaining separate from the underlying Rust-powered
//! engine.
//!
//! That separation makes it possible to build other interfaces without
//! rebuilding the core.
//!
//! The same knowledge can be accessed through:
//!
//! ```text
//! React
//!   │
//! CLI ───── SDKs
//!   │         │
//!   └──── MCP ┘
//!        │
//!        ▼
//! Conundrum Core
//!        │
//!  Native / WASM
//!        │
//!        ▼
//!  Local Data
//! ```
//!
//! ---
//!
//! # Offline by necessity, local by philosophy
//!
//! One of the more unusual things about Conundrum's history is that significant
//! portions of it were built while its creator was homeless and offline.
//!
//! That constraint had consequences.
//!
//! For example, even mathematical functionality couldn't simply assume that a
//! CDN would always be available.
//!
//! The project therefore developed with a strong bias toward:
//!
//! - Local execution
//! - Self-contained tooling
//! - Minimal external dependencies
//! - Offline-capable functionality
//! - Portable data
//! - User ownership
//!
//! What began as necessity became philosophy.
//!
//! **Local-first isn't an afterthought. It's part of the project's DNA.**
//!
//! ---
//!
//! # Why STEM?
//!
//! STEM work produces unusually structured knowledge.
//!
//! Equations have relationships.
//!
//! Experiments have variables.
//!
//! Papers cite papers.
//!
//! Theorems depend on definitions.
//!
//! Measurements produce results.
//!
//! Results support or contradict hypotheses.
//!
//! Programs implement algorithms.
//!
//! Algorithms depend on mathematical concepts.
//!
//! The structure is already there.
//!
//! Conundrum's purpose is to make that structure explicit enough that both
//! humans and machines can use it.
//!
//! That makes it particularly well suited to:
//!
//! - Physics
//! - Mathematics
//! - Computer science
//! - Engineering
//! - Chemistry
//! - Biology
//! - Research
//! - Technical education
//! - Software development
//!
//! But the underlying architecture isn't limited to those domains.
//!
//! ---
//!
//! # The bigger idea
//!
//! Conundrum is ultimately an experiment in a different way of thinking about
//! personal software.
//!
//! Most software stores information for a particular application.
//!
//! Conundrum tries to build a **knowledge layer that applications can share.**
//!
//! Your notes aren't just notes.
//!
//! Your experiments aren't just records.
//!
//! Your projects aren't just projects.
//!
//! Your documents, entities, relationships, observations, and accumulated
//! context can become a coherent graph.
//!
//! Then AI can operate on that graph.
//!
//! That changes the role of AI.
//!
//! Instead of:
//!
//! > "AI, remember everything about me."
//!
//! the architecture becomes:
//!
//! > "AI, here's how you can access the things I know."
//!
//! That's a much more scalable proposition.
//!
//! ---
//!
//! # Why should a non-technical person care?
//!
//! You don't need to understand Rust, WebAssembly, vector databases, graphs, or
//! MCP to benefit from Conundrum.
//!
//! The important part is what the architecture enables.
//!
//! Imagine having a notebook that could answer:
//!
//! > "What did I learn about this six months ago?"
//!
//! Or:
//!
//! > "What are the things I know that are related to this?"
//!
//! Or:
//!
//! > "What am I missing before I can understand this paper?"
//!
//! Or:
//!
//! > "Find every experiment I've performed related to this hypothesis."
//!
//! Or:
//!
//! > "What have I said about this person across all of my notes?"
//!
//! Or:
//!
//! > "Build me a study plan using everything I've already learned."
//!
//! The point isn't that AI becomes smarter.
//!
//! The point is that **AI gets access to better organized information.**
//!
//! And because the information is yours and remains local, that intelligence
//! can continue to work for you regardless of which AI model happens to be
//! popular next year.
//!
//! ---
//!
//! # Philosophy
//!
//! Conundrum is built around a few principles.
//!
//! ### Your data should belong to you.
//!
//! Your knowledge shouldn't be locked inside a proprietary cloud.
//!
//! ### Structure beats giant prompts.
//!
//! AI systems should retrieve relevant information rather than repeatedly
//! ingesting everything.
//!
//! ### Relationships matter.
//!
//! Knowledge is more than a collection of documents.
//!
//! ### Semantic information should survive rendering.
//!
//! If a component means "research," that meaning shouldn't disappear simply
//! because the document was rendered to another format.
//!
//! ### One concept should have one canonical representation.
//!
//! Deliberate syntax makes knowledge easier for both humans and machines to
//! understand.
//!
//! ### AI should build tools.
//!
//! The most interesting future isn't an AI that answers every question.
//!
//! It's an AI that can help you build the tools you need.
//!
//! ### The core should be reusable.
//!
//! Conundrum shouldn't be limited to the application that ships with it.
//!
//! ### Models should be replaceable.
//!
//! Your knowledge infrastructure shouldn't depend on one AI provider.
//!
//! ---
//!
//! # Licensing
//!
//! Conundrum is released under the **World-Changing Technology License (WCTL)
//! v1.0**.
//!
//! WCTL is designed to make the technology broadly accessible while creating a
//! sustainable path for contributors and charitable giving.
//!
//! The complete legal terms are available in [`LICENSE`](LICENSE).
//!
//! The README is intentionally only a description of the philosophy behind the
//! license; **the license itself controls.**
//!
//! A central goal of the project is that the people building and maintaining
//! the technology should be compensated fairly.
//!
//! After the project's defined compensation and operating requirements,
//! proceeds above the project's defined developer compensation benchmark are
//! directed toward charitable causes.
//!
//! In particular, the project is designed around the principle that revenue
//! above the **median developer salary** should not simply become additional
//! profit.
//!
//! It should do some good.
//!
//! Please see WCTL v1.0 for the exact definitions, conditions, and legal
//! mechanisms governing that commitment.
//!
//! ---
//!
//! # A personal note
//!
//! There is a slightly unusual story behind this project.
//!
//! **Conundrum was built by someone who has experienced homelessness and built
//! significant portions of the project while offline.**
//!
//! It began with an attempt to pursue astrophysics full-time.
//!
//! It evolved through years of experimentation.
//!
//! It was rebuilt repeatedly.
//!
//! And it has now become an attempt to create infrastructure for the next
//! generation of AI-assisted software.
//!
//! That history matters to the project.
//!
//! It is a reminder that ambitious technical projects don't always begin with
//! funding, a team, or an office.
//!
//! Sometimes they begin with a person, a computer, an idea, and a lot of
//! stubbornness.
//!
//! If you believe in what Conundrum is trying to accomplish, there are several
//! ways to help.
//!
//! ### Use it.
//!
//! The most valuable validation is people finding it useful.
//!
//! ### Contribute.
//!
//! Code, documentation, testing, examples, MCP tools, SDK improvements,
//! language features, and ideas are all welcome.
//!
//! ### Build something.
//!
//! The project is specifically designed to be extended.
//!
//! ### Support it financially.
//!
//! If Conundrum becomes useful to you and you'd like to help keep development
//! moving, financial contributions are more than welcome.
//!
//! ### Offer a job.
//!
//! Seriously.
//!
//! If you're looking for someone interested in Rust, AI infrastructure,
//! developer tooling, knowledge graphs, systems architecture, or building
//! ambitious software from scratch, **job offers are more than welcome too.**
//!
//! Sometimes the most useful contribution isn't a pull request.
//!
//! Sometimes it's an opportunity.
//!
//! ---
//!
//! # The roadmap
//!
//! The long-term vision is considerably larger than a note-taking application.
//!
//! Imagine a personal knowledge system where:
//!
//! - Every document can participate in a graph.
//! - Every relationship can be queried.
//! - Semantic search happens locally.
//! - AI agents can navigate your knowledge through MCP.
//! - Models can be swapped without migrating your data.
//! - Entity-specific context accumulates over time.
//! - The language preserves semantic information across output targets.
//! - The Rust core runs both natively and in WebAssembly.
//! - SDKs let developers build applications in the languages they already use.
//! - AI can help you build new tools directly on top of your knowledge base.
//! - Your knowledge remains yours regardless of which AI company leads the
//!   market.
//!
//! That's the direction of Conundrum.
//!
//! **A knowledge system for humans that is structured enough for machines.**
//!
//! A local second brain.
//!
//! A programmable knowledge layer.
//!
//! A toolkit for the vibe-coding era.
//!
//! And, hopefully, a small piece of technology that makes the world a little
//! more interesting.
//!
//! ---
//!
//! ## Contributing
//!
//! Contributions are welcome.
//!
//! Whether you're interested in:
//!
//! - Rust
//! - React
//! - WebAssembly
//! - AI/LLM infrastructure
//! - MCP
//! - Vector search
//! - Knowledge graphs
//! - SDK development
//! - Language design
//! - STEM tooling
//! - Documentation
//! - Testing
//!
//! there is room to help.
//!
//! Conundrum is intentionally modular.
//!
//! **If you can imagine something that should exist on top of Conundrum, build
//! it.**
//!
//! ---
//!
//! ## License
//!
//! **World-Changing Technology License (WCTL) v1.0**
//!
//! See [WCT-License](https://flusterapp.com/license) for the complete legal terms.
//!
//! ---
//!
//! <p align="center">
//!   <strong>Your knowledge. Your data. Your tools. Your models.</strong>
//!   <br />
//!   <em>Build something interesting.</em>
//! </p>
#![feature(string_replace_in_place)]
uniffi::setup_scaffolding!();

#[cfg(feature = "db")]
pub mod ai;
pub mod bibliography;
pub mod ecosystem;
pub mod embedded;
pub mod lang;
pub mod lifted_models;
pub mod macros;
pub mod output;
pub mod parsers;
pub mod testing;
