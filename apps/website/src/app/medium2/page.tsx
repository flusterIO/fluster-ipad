import React, { type ReactNode } from "react";

const MediumPage = (): ReactNode => {
    return (
        <article className="w-270 mx-auto">
            <h1>Introducing flusterapp.com: A Home for Fluster and Conundrum</h1>
            <p className="subtitle">
                A note-taking app for people whose notes need to do more than plain text
                allows — and now, a place on the web to find it.
            </p>
            <p className="byline">By Andrew Mueller</p>

            <figure>
                <img
                    src="/assets/logo/fluster_banner_with_name.png"
                    alt="Fluster's logo on a deep navy background"
                />
            </figure>

            <p>
                Today I'm launching <a href="">flusterapp.com</a>, the new home of
                Fluster.
            </p>

            <p>
                Fluster is a note-taking app built around a simple idea: your notes
                should be as expressive as your thinking. Sometimes that means clean
                prose. Sometimes it means a code block, a warning nobody should miss, a
                citation to a paper from 1905, or a diagram you can only get out of your
                head with a pen. Most note apps make you pick one of those. Fluster is
                built to hold all of them in the same place.
            </p>

            <p>
                The website is where you can learn what Fluster is, get the app, and
                follow along as it grows.
            </p>

            <h2>Write on the left, see it on the right</h2>

            <p>
                At the heart of Fluster is a split-view editor. You write on the left
                and see a fully rendered note on the right, live, as you type. The
                editor has line numbers, syntax highlighting and collapsible sections,
                and your work is saved with every keystroke. If live rendering feels too
                busy, you can switch to on-save mode, and the preview updates only when
                you press ⌘S.
            </p>

            <figure>
                <img
                    src="/assets/images/mockups/ipad/mockups/welcome_splitview.png"
                    alt="Fluster's split-view editor with Markdown source on the left and the rendered note on the right"
                />
                <figcaption>
                    The split-view editor: source on the left, rendered note on the right.
                </figcaption>
            </figure>

            <h2>Meet Conundrum</h2>

            <p>
                The language you write in is called Conundrum. It's a superset of
                Markdown, inspired by MDX, which means everything you already know about
                Markdown just works. On top of that, Conundrum gives you components you
                can drop straight into a note: hints and goals that call out what
                matters, tabs that organize one note for different readers, warning
                cards, titled code blocks with one-tap copy, inline emoji and
                highlights.
            </p>

            <p>Here's what a hint looks like in Conundrum:</p>

            <pre>
                <code>
                    &lt;Hint&gt; Consider setting the save method to 'on-save' if live
                    rendering feels too jittery or distracting. &lt;/Hint&gt;
                </code>
            </pre>

            <p>
                Conundrum is also designed to travel. A note written in Conundrum can be
                converted to standard Markdown or to plain text, so the expressive,
                stylized version is there for people to read, while a clean version is
                always available for search and for AI tools.
            </p>

            <blockquote>
                The goal of Conundrum is to let anyone write notes as expressive as the
                examples by the end of their first day — without looking at the
                documentation.
            </blockquote>

            <p>
                You don't need to be a developer to use it. Type a single command on its
                own line and Fluster brings up its built-in documentation, with separate
                paths for complete beginners, people who already know Markdown, and
                experienced developers.
            </p>

            <h2>Paper, for the ideas that start with a pen</h2>

            <p>
                Not every idea starts as text. Every note in Fluster has its own Paper
                pages for handwriting and sketching, with pens, markers, a highlighter,
                an eraser, a ruler and a full color picker. Add as many pages as you
                need and flip between them, all attached to the note they belong to.
            </p>

            <figure>
                <img
                    src="/assets/images/mockups/ipad/mockups/paper.png"
                    alt="Fluster's Paper view with handwritten notes and the drawing tool palette"
                />
                <figcaption>
                    Paper: handwriting and sketching, attached to the note it belongs to.
                </figcaption>
            </figure>

            <h2>Built for research</h2>

            <p>
                Fluster grew out of research work, and it shows. Each note keeps its own
                bibliography, so every source stays attached to the writing that uses
                it. Citations are listed in clean, consistent formatting with DOI links,
                so a reference is always one tap away.
            </p>

            <figure>
                <img
                    src="/assets/images/mockups/ipad/mockups/note-bib.png"
                    alt="Fluster's Bibliography view listing formatted citations with DOI links"
                />
                <figcaption>Each note keeps its own bibliography.</figcaption>
            </figure>

            <p>
                The Details view gives you the whole note at a glance: its title, a
                short summary, its tags, every citation it contains and when it was last
                modified. And when you need something, search finds it across all of
                your notes.
            </p>

            <figure>
                <img
                    src="/assets/images/mockups/ipad/mockups/note_details.png"
                    alt="Fluster's Details view showing a note's title, summary, tags and citations"
                />
                <figcaption>
                    The Details view: title, summary, tags and citations in one place.
                </figcaption>
            </figure>

            <h2>What you'll find on flusterapp.com</h2>

            <p>
                The website is the hub for everything Fluster. It's where you'll find
                the app, learn how Fluster and Conundrum work, and read the blog, where
                I'll be sharing updates, new features and some of the writing I've done
                in Fluster myself, including{" "}
                <strong>
                    <a href="/blog/by_path/on_the_gravitational_nature_of_time">
                        On the Gravitational Nature of Time
                    </a>
                </strong>
                , a plain-English tour of a new geometric idea about time, gravity and
                the expanding universe.
            </p>

            <figure>
                <img
                    src="/assets/images/mockups/website/website_home_macbook.png"
                    alt="The flusterapp.com home page"
                />
                <figcaption>flusterapp.com</figcaption>
            </figure>

            <h2>Why Fluster exists</h2>

            <p>
                I built Fluster because I needed it. I wanted one place where my
                writing, my sketches and my sources could live together, and where the
                notes I wrote for myself could look as good as anything I'd publish.
            </p>

            <p>
                Fluster was built under less-than-ideal circumstances, in a language I
                was learning as I went, often without reliable internet or a power
                outlet. That shaped some early decisions, and it's why I'm upfront that
                this is the beginning, not the finished product. Conundrum isn't yet
                fully CommonMark compatible, the default standard for Markdown, but it
                will be soon. I'm proud of what Fluster and Conundrum are today, and I
                expect them to be unrecognizable in a year.
            </p>

            <h2>Get started</h2>

            <p>
                Fluster is available now for iPad. Head to{" "}
                <a href="https://flusterapp.com">flusterapp.com</a> to get the app,
                explore Conundrum, and follow along on the blog. If you try it, I'd love
                to hear what you think, especially what you'd want it to do next.
            </p>

            <p>Thanks for reading.</p>

            <p>— Andrew</p>
        </article>
    );
};

MediumPage.displayName = "MediumPage";

export default MediumPage;
