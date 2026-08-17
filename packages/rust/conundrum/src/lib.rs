//! # Conundrum
//!
//! > Conundrum is a modular academic tool-kit that was built in the pursuit of
//! quantized gravity, rebuilt for the vibe-coding era. [See it in action in our
//! blog](https://flusterapp.com).
//!
//!
//! ## Conundrum, the language
//!
//! Conundrum is a language with syntax built on top of mdx. In fact, most mdx
//! notes with components of the same name and properties would compile just
//! fine, but it uses no javascript at all and offers additional syntax
//! features like equation ids, special link syntaxes and more. The language is
//! compiled completely by Rust, supports wasm targets, and of course integrates
//! with the rest of the Conundrum ecosystem flawlessly.
//!
//! In fact, you can compile your notes to a number of targets, with each
//! supported component offering both global and instance specific properties
//! that can customize the output to specific targets. In short, you can add a
//! field like:
//!
//! ```tsx
//! <Hl md="italic">My highlighted text</Hl>
//! ```
//!
//! To make the output of the markdown target italic. That's actually how
//! Fluster render's user's markdown content to Swift when it needs to display a
//! title, despite Swift's extremely limited markdown support.
//!
//! Passing these component properties down to these secondary ouptuts keeps the
//! vital information stored in the 'mdx' components that Conundrum supports
//! across output targets that may or may not support complex UIs like the web.
//!
//! In the age of AI when everything is about the amount of information per
//! token, a language that is more descriptive than markdown yet more deliberate
//! than html may be a more ideal form of communication. Where markdown excels
//! in it's simplicity, it sacrifices in it's expressiveness which is a hurdle
//! to both human and AI. HTML on the other hand can be incredibly expressive
//! with css applied, but much of this information is lost during the parsing
//! process, and even if it were to survive parsing, the incredible variety of
//! the web would produce less specific data.
//!
//! Conundrum aims to walk this middle ground by providing a very deliberate set
//! of components with the _high-level_ properties required for users to
//! document almost anything, in a style that agrees with them, while not
//! providing so many low level properties that it conflates AI.
//!
//! Conundrum will never offer more than 1 of anything, and that's for a reason.
//! An `<Admonition ... />` component conveys information to AI that would be
//! lost if there were a hundred ways to create the same component... even ore
//! so if you add an `emphasis` property like `error`, `warn` or `research`.
//! These properties not only style your notes to match accordingly, but pass
//! along important information to AI.
//!
//! #### Supported Outputs
//!
//! - [x] HTML
//! - [x] Jsx
//! - [ ] Commonmark compatible Markdown (90% there. A couple components are
//!   missing their
//! templates)
//! - [ ] Json, for rendering from the AST in languages like Swift and Kotlin.
//!
//! On top of these output targets, Conundrum offers a number of customization
//! flags that can be used to do everything from hidding emojis, to embedding
//! javascript in a single file output. As I built this while homeless and
//! offline, not even the math needs to be loaded from a CDN.
//!
//! ---
//!
//! ## Conundrum, the ecosystem
//!
//! Conundrum came to be out of [Fluster](https://flusterapp.com), which itself came to be after I quit my
//! job almost 5 years ago to focus on astrophysics full-time. I've since
//! rewritten it to the point that it's unrecognizable, and now I'm re-working
//! things once again for the vibe-coding era.
//!
//! Conundrum has a few pieces that all come together to make one cohesive,
//! local second brain that only reaches out to an outside service when it's
//! necessary to contact AI. Your database remains on your machine, with models
//! to _completely_ describe your life.
//!
//! While the Conundrum ecosystem is heavily focused on academic note-taking, a
//! natural extension of learning a note-taking DSL for any new programmer is to
//! extend it's capabilities to the rest of their life. For this reason, the
//! Conundrum database includes database models for everything from the results
//! of a scientific experiment to your family pet, all structured in a **very**
//! graph oriented design.
//!
//! This approach let's AI get as close to a perfect answer as exists within
//! your knowledge base with smaller context requirements by using both vector
//! similarity methods and this graph architecture.
//!
//! Further, Conundrum adds a sort of 'notepad' to many of these models,
//! allowing AI to keep track of information associated with specific instances
//! of that model over time.
#![feature(string_replace_in_place)]
uniffi::setup_scaffolding!();

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
