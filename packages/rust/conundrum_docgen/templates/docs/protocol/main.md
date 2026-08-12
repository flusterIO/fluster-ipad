# Conundrum Protocol

Conundrum is not just a language, it's a modular ecosystem of tools built to help user's utilize their computer as the academic powerhouse that it is. For user's that never intend to become full-time developers or dedicate years to learning their first programming language well, Conundrum aims to be the middle ground by providing a set of _high-level_ properties that are domain specific to note-taking, and now a _protocol_ that will allow the user to take their notes to other applications should they choose.


### Motivation

#### Technical Motivation

While html and css are far more flexible than markdown, their extensive property list is less descriptive for AI. Take a css class for example, `red-500` which may indicate an error, or simply a brand color.

Markdown on the other hand is very simple, providing only a handful of _non-verbal_ pieces of information to models trained on markdown (ie: is the text in a header?), but this limits the expressiveness of the user's content and limits the potential information that can be passed both to AI, and visually to other humans.

Conundrum aims to take a different approach: Make the language the user writes ***simple*** but descriptive, build everything manually, document it, and let AI build on top of the pieces built by the community of developers, while distributing any potential earnings back to the developers that contributed to the project in a manner that is objective and fair. 

##### Financial Motivation

Should the project ever be so lucky as to exceed this amount, any excess funds will go to charity, as no individual person should ever make more than the median annual developer salary proportional to the amount of time that they've contributed to the project and the 'seniority' of the task, as determined by AI. 

Once the project is properly established, AI will be asked to review each _merged_ pull request with the following prompt and pseudo-code:


```
let {
    developer_id: "xyz",
   difficulty: "junior" | "staff" | "senior",
    hours: u16
} = AiPrompt({
    system_prompt: """
You are an unbiased judge for an open-source software platform. Answer all prompts honestly while embodying the core principles of fairness, logic and empathy.
""",
    prompt: f"""
{data.user} just merged the most recent pull request, {data.commit}. Review Conundrum's Git history and assess the number of hours {data.user} dedicated to this commit, and the difficulty of the feature that they are adding or expanding upon.

If the task was very difficult, set difficulty to "senior".
If the task was something an entry level or junior software engineer can handle, set difficulty to "junior".
If the task falls somewhere in the middle, mark it as "staff".
""",
})
```

This approach of course will need to scale with the scope of the project. It's illogical for somebody that's borderline homeless to establish accounts for a project that is yet to receive any attention, but if this project ever receives any repetitive income, a server will be setup with this functionality within 60 days.


## Goals

### Note-Taking DSL

The primary goal of Conundrum and the Conundrum ecosystem is to be a drop-in, first-step for any user looking to get more from their device, without the desire to learn a complete note taking language.

Once the `Rhai` integration is complete, Conundrum may grow to support other **high-level** functionalities like renaming files with AI, or eventually to become something of a scripting language with a native ai module, but it will always first and foremost be a note taking DSL with an academic bias.

### Programmability 

Conundrum's primary goal is to be the middle ground between the simplicity of Markdown and the complexity but expressiveness of HTML/JS or other full-scale programming languages. The xml style `.jsx` syntax was an obvious pick for this, as the 'children' property is a natural fit for many note taking tasks.

While Conundrum, as of now does not support typical 'programming', an integration with **Rhai** is planned as an embedded language, but the goals don't end there. Once integration with Rhai is enabled, it should be within reach to create a shared memory block that can be utilized by _all_ code-blocks within the note, should those languages be available for compilation at compile time.


## Appendix

### Intended Starter Templates

- [ ] Tauri (work has been started)
- [ ] A blog via vite


[^1]: The 'code' has not yet been implemented, as I don't even have internet let alone income, but the prompts will not change in the interest of fairness.
