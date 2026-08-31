# Fluster & Conundrum

![Fluster screenshot](shared_assets/logo/fluster_banner_with_name.png)

**A research environment for writing, computing, organizing, and exploring ideas.**

Fluster is a research and note-taking application built around **Conundrum**, a Rust-powered content and computation platform designed for students, researchers, academics, and technically minded people.

Together, Fluster and Conundrum are intended to turn your notes into something more than documents.

They are a place where **writing, mathematics, code, data, references, visualizations, AI, and research can live together.**

> **Fluster is the application. Conundrum is the platform underneath it.**

---

## What is Fluster?

[Fluster](https://www.flusterapp.com/) is a high-performance research environment built for people whose work doesn't fit neatly into a traditional note-taking application.

It combines the things researchers routinely need:

- Markdown and MDX authoring
- Mathematical notation and equations
- Code and executable snippets
- Interactive visualizations
- Bibliography and reference management
- Tasks and project organization
- PDFs and research materials
- Tabular and numerical data
- Images and video
- AI-assisted research
- Local and remote AI models
- Semantic search and retrieval
- Whiteboards and interactive content
- Integrations with external research tools

The goal isn't to build another application that helps you write prettier notes.

The goal is to build an environment where **the things you write about can become things you can work with.**

Fluster is currently focused on Apple's platforms, with a cross-platform Conundrum application in development.

---

## What is Conundrum?

Conundrum is the underlying technology that makes Fluster possible.

It is a Rust-powered content and application platform built around an extended Markdown language, structured data, plugins, AI tooling, and a high-performance local data layer.

At its core is **MDX**: Markdown extended with components and richer semantics.

Instead of treating a note as a static document, Conundrum allows a document to become a structured, interactive environment.

For example, a research note can contain:

```mdx
# Orbital Mechanics

The orbital velocity is:

$$
v = \sqrt{\frac{GM}{r}}
$$

<Plot data={orbitalData} x="radius" y="velocity" />
```

The same document can contain prose, equations, executable concepts, structured data, visualizations, references, and application components.

Conundrum is intended to make those pieces composable.

---

# Why two projects?

Fluster and Conundrum deliberately have different responsibilities.

### Fluster

Fluster is the user-facing application.

It provides the interface, editors, navigation, reading experiences, research workflows, and platform-specific integrations that make the underlying system approachable.

### Conundrum

Conundrum provides the underlying language, runtime, data model, tooling, and extensibility infrastructure.

It is designed to work independently of a particular user interface and eventually power applications beyond Fluster.

This separation makes it possible to build:

```text
                    ┌─────────────────────┐
                    │       Fluster       │
                    │   Research App      │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │      Conundrum      │
                    │ Language & Runtime  │
                    └──────────┬──────────┘
                               │
          ┌────────────────────┼────────────────────┐
          ▼                    ▼                    ▼
       Documents            Plugins              Data
          │                    │                    │
          ▼                    ▼                    ▼
        MDX                 Tools              LanceDB
       Markdown              MCP               Vectors
        Code                AI/RAG             Search
        Math               Integrations       Structured Data
```

The long-term goal is for Conundrum to become a platform that can support multiple clients, while Fluster remains the flagship research experience.

---

# Built for research

Most productivity applications are designed around documents.

Research is not.

A research workflow might involve a paper, a PDF, a dataset, a Jupyter notebook, a collection of equations, a bibliography, several experiments, some source code, a lecture recording, and hundreds of pages of notes.

Those things shouldn't have to live in completely separate applications.

Fluster is designed around the idea that **research is a connected graph of information**, rather than a collection of isolated files.

---

## Write in MDX

Conundrum extends Markdown with components and richer document semantics.

That means you retain the simplicity of Markdown while gaining the ability to embed richer content directly into your documents.

```mdx
# My Research

This is ordinary Markdown.

<Chart data={results} />

<Whiteboard />

<Code language="rust">
fn calculate(x: f64) -> f64 {
    x.powi(2)
}
</Code>
```

The result is a document format that remains readable as text while being capable of representing substantially more complex applications.

---

# One environment for your research materials

Fluster is designed to work with the kinds of files researchers already use.

### Markdown & MDX

Write notes using familiar Markdown syntax while extending documents with interactive components.

### PDFs

Keep papers and reference material alongside the notes that explain them.

### Tabular data

Work with CSV, Excel, and other tabular formats supported by common data-processing ecosystems.

### Numerical data

Bring numerical results from calculations, scripts, notebooks, or experiments into the same environment as the research surrounding them.

### Images

Keep figures, diagrams, screenshots, and other visual material connected to the notes that explain them.

### Video

Connect lectures, demonstrations, and recordings to notes and relevant timestamps.

The objective is simple:

**Stop moving your research between applications just because the data changed format.**

---

# AI that works with your research

Fluster is designed around retrieval-augmented AI rather than treating an LLM as an isolated chatbot.

Your notes, documents, bibliography, and other research material can become part of a searchable knowledge base.

This allows AI systems to work with the material you actually care about.

The architecture is designed to support both:

- **local models**, including Ollama
- **remote models**, including OpenAI and other providers

Local AI is particularly important for research workflows where privacy, reproducibility, cost, or offline operation matters.

The underlying Conundrum ecosystem provides the infrastructure for:

- embeddings
- vector search
- semantic retrieval
- structured generation
- tool calling
- MCP
- AI agents
- local data storage

AI should be able to work _with_ your research rather than requiring you to copy your research into a separate application.

---

# A local-first data layer

Conundrum uses a structured local data architecture designed around high-performance Rust components.

The system can combine:

- structured records
- full-text search
- vector embeddings
- metadata
- relationships
- semantic retrieval

This provides the foundation for features such as:

```text
                    Your Research
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
        Notes          Papers         Data
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  Structured Data
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
         Full Text Search       Vector Search
              │                     │
              └──────────┬──────────┘
                         ▼
                    AI / RAG
```

The result is a research environment where search doesn't have to stop at exact words.

---

# Extensible by design

Conundrum is built around a plugin-oriented architecture.

Instead of requiring every feature to become part of the core application, functionality can be provided through modular components.

This makes it possible for the ecosystem to evolve without requiring the core platform to understand every possible use case.

Potential integrations include:

- Google Calendar
- Jupyter
- AI providers
- MCP servers
- external data sources
- community plugins
- custom research tools

The goal is an ecosystem where the application can adapt to the researcher rather than forcing the researcher into a predetermined workflow.

---

# Developer ecosystem

Conundrum is being built as more than an application.

The project includes tooling intended to make the underlying platform accessible from multiple languages and environments.

The ecosystem includes work around:

- Rust
- TypeScript
- Python
- Go
- Swift
- Lua
- WebAssembly
- MCP

This allows Conundrum to serve both as a user-facing platform and as infrastructure for developers building research-oriented applications.

---

# Native applications

Fluster's current application experience is focused on Apple's platforms.

The native applications are being rebuilt around the new Rust-powered architecture while taking advantage of platform-specific capabilities such as:

- native macOS interfaces
- native iPad interfaces
- Apple Pencil
- PaperKit
- high-performance editing
- native file handling
- platform-specific integrations

The longer-term architecture separates platform-specific presentation from the underlying Conundrum system.

This means the same research content and core functionality can eventually be presented through multiple clients.

---

# Cross-platform Conundrum

A cross-platform Conundrum application is also in development.

The goal is for the platform to eventually be usable beyond Apple's ecosystem without sacrificing the underlying architecture that makes Fluster possible.

In other words:

```text
                         Conundrum
                            │
             ┌──────────────┼──────────────┐
             ▼              ▼              ▼
          Fluster       Web / Desktop   Future Clients
          Apple            Clients
             │
             └──────────────┬──────────────┘
                            ▼
                     Shared Research
                        Ecosystem
```

Fluster is the first major expression of the platform, not necessarily the last.

---

# A project built around curiosity

Fluster and Conundrum began as a personal attempt to build a better environment for doing research.

That origin still matters.

The project is particularly interested in the intersection of:

- physics
- mathematics
- computer science
- artificial intelligence
- scientific computing
- knowledge management
- programming languages
- human-computer interaction

The software is therefore being developed around a simple question:

> **What would a computer look like if it were designed around the way researchers actually think?**

That question drives both projects.

---

# Repository structure

The repository contains the applications, libraries, tooling, and infrastructure that make up the Fluster and Conundrum ecosystem.

A simplified view looks something like:

```text
fluster/
│
├── Fluster/                  # Apple applications
│
├── packages/
│   ├── rust/
│   │   ├── conundrum/        # Core Conundrum platform
│   │   ├── conundrum_macros/ # Procedural macros
│   │   ├── conundrum_server/ # Server / MCP infrastructure
│   │   └── ...
│   │
│   ├── typescript/           # TypeScript ecosystem
│   ├── python/               # Python tooling
│   ├── swift/                # Swift interfaces
│   └── ...
│
├── examples/
│
├── docs/
│
└── README.md
```

The exact structure will continue to evolve as the platform matures.

---

# Status

Fluster and Conundrum are actively under development.

The project is currently undergoing a substantial architectural rewrite centered around Rust and the new Conundrum architecture.

Expect things to move quickly.

APIs, package boundaries, application interfaces, and documentation may change while the new architecture settles.

This repository should therefore be considered **active development software** rather than a stable release.

---

# Contributing

Contributions, ideas, experimentation, and criticism are welcome.

There are many areas where the project can benefit from outside expertise, particularly around:

- Rust
- Swift
- React
- WebAssembly
- compilers and parsers
- scientific computing
- physics
- mathematics
- AI / RAG
- vector databases
- information retrieval
- UI/UX
- documentation
- plugins and integrations

If something interests you, open an issue or start a discussion.

The project is being built in public, and the architecture is intentionally designed to make experimentation possible.

---

# Philosophy

Fluster is not intended to be another closed productivity silo.

Conundrum is not intended to be another proprietary document format.

The broader goal is to create an open, composable research environment where:

**your data belongs to you,**

**your documents remain useful outside the application,**

**your tools can be replaced,**

**your AI can be local,**

and **your research can become computational rather than merely textual.**

---

## Fluster + Conundrum

Fluster gives Conundrum a home.

Conundrum gives Fluster a foundation.

Together, they are an attempt to build a research environment where the distance between an idea and the tools needed to explore it becomes as small as possible.

**Write it.**

**Compute it.**

**Visualize it.**

**Connect it.**

**Question it.**

**Explore it.**

---

**Fluster:** [flusterapp.com](https://www.flusterapp.com/)

**Conundrum:** the open platform underneath it.
