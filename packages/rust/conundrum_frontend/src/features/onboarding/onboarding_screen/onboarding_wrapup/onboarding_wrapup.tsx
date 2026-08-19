import { CdrmContent } from '#/cdrm/cdrm_content'
import React, { type ReactNode } from 'react'

export const OnboardingWrapup = (): ReactNode => {
    return (
        <div className="w-full h-full min-h-screem flex flex-col justify-center items-center p-6 overflow-x-hidden overflow-y-auto">
            <div className="cdrm @container/mdx fluster-mac flex flex-col justify-center items-center w-full max-w-270">
                <CdrmContent
                    content={`
# That's it! <Emoji name="boom" inline medium />

So what's next? You can use the Conundrum ecosystem of tools as regular markdown, or you can learn _Conundrum_, a super-set of markdown that offers additional features on-top of the markdown syntax you're already familiar with[^1].

Of course there are the basics like math:

$$
\\delta = 2 G \\frac{M_\\oplus}{R_\\oplus^3} \\hat{R}
$$

Things like this admonition:

<Admonition title="Admonition" info>
This admonition can be created with the following syntax.

'info' can be any of the supported emphasis', 
each styling the component accordingly.
</Admonition>

\`\`\`tsx -- title="admonition.cdrm"

<Admonition title="Admonition" info>
This admonition can be created with the following syntax.

'info' can be any of the supported emphasis', 
each styling the component accordingly.
</Admonition>

\`\`\`

Or this card:

<Card title="Or this card!" desc="And don't forget...">
Every component is documented right inside the Conundrum compiler.
</Card>

Or any of the growing number of components that Conundrum supports. Want to learn more? Just ask AI, or compile any Conundrum note with \`Docs\` on a line all by it's self to view the embedded documentation.

### You're All Set...

One last thing though. I built this all while homeless, and I'm still _sort-of_ homeless after leaving my career in software to focus on a [modified model of relativity](https://flusterapp.com/blog/by_path/on_the_gravitational_nature_of_time) full-time. If you see the value in Conundrum and the Conundrum ecosystem of tools, please consider supporting the project. While this project is still in it's early days, just imagine what it can become if I can actually afford an internet connection.

[^1]: The note you're looking at right now is compiled from Conundrum
`}
                />
            </div>
        </div>
    )
}


OnboardingWrapup.displayName = "OnboardingWrapup"
