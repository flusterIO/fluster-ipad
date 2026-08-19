import { CdrmContent } from "#/cdrm/cdrm_content";
import { type BackendStatus } from "#/database/db_utility_types/health";
import {
    REMOTE_AI_ENVIRONMENT_VARIABLE,
    REMOTE_AI_PROVIDER_NAME,
} from "@/ai_constants";
import React, { type ReactNode } from "react";

export const OnboardingWrapup = ({
    results,
}: {
    results: BackendStatus | null;
}): ReactNode => {
    const ollama = {
        valid: `- [x] Ollama
  Your Ollama environment is valid and Conundrum was able to connect successfully. You'll have access to a full offline fall-back and local models when the task is fitting for local hardware.`,
        invalid: `- [ ] Ollama
  Your Ollama environment appears to be invalid or completely absent. Conundrum could not connect to Ollama which means you'll need to rely only on remote infereence.`,
    };
    const remote = {
        valid: `- [x] Server Scale AI
  Your ${REMOTE_AI_PROVIDER_NAME} environment is valid and Conundrum was able to connect successfully. While Ollama provides local fallback for cost efficent tasks that your local hardware can handle, ${REMOTE_AI_PROVIDER_NAME} offers all the horsepower a modern academic needs.`,
        invalid: `- [ ] Server Scale AI
  Conundrum could not connect to the remote server. It appears as if you're missing your \`${REMOTE_AI_ENVIRONMENT_VARIABLE}\` environment variable, are perhaps offline, or are encountering another environment issue.`,
    };
    return (
        <div className="w-full h-full min-h-screem flex flex-col justify-start items-center p-6 overflow-x-hidden overflow-y-auto min-scrollbar scrollbarMuted">
            <div className="cdrm @container/mdx fluster-mac flex flex-col justify-center items-center w-full max-w-270">
                <CdrmContent
                    content={`
# That's it! <Emoji name="boom" inline medium />

<Hint>
Use cmd+shift+p for the command palette!
</Hint>

<Card title="Results">
${results?.local_client_access ? ollama.valid : ollama.invalid}
${results?.remote_client_access ? remote.valid : remote.invalid}
</Card>

So what's next? You can use the Conundrum ecosystem of tools as regular markdown (just drop in your Obsidian vault), or you can learn _Conundrum_, a super-set of markdown that offers additional features on-top of the markdown syntax you're already familiar with[^1].

Of course there are the basics like math:

$$ {#myEquationId}
\\delta = 2 G \\frac{M_\\oplus}{R_\\oplus^3} \\hat{R}
$$

But with Conundrum you can add things like this admonition:

<Admonition title="My Admonition" info foldable>

I can even reference my equation by id like <EqRef id="myEquationId" />.

</Admonition>


\`\`\`\`tsx -- title="admonition.cdrm"

<Admonition title="My Admonition" info foldable>
I can even reference my equation by id like <EqRef id="myEquationId" />.

This equation id works because I used the special Conundrum syntax:

$$ {#myEquationIdHere} <- Your Id goes here
\\delta = 2 G \\frac{M_\\oplus}{R_\\oplus^3} \\hat{R}
$$

</Admonition>

\`\`\`\`

Or this card:

<Card title="Or this card!" desc="And don't forget..." sidebar right>
Every component is documented right inside the Conundrum compiler. Just type the name of the component followed by either 1 or 2 \`??\` on a line all by it's self in any Conundrum file and the compiler will embed the documentation.
</Card>

Or any of the growing number of components that Conundrum supports. Want to learn more? Just ask AI, or compile any Conundrum note with \`Docs??\` on it's own line to view the embedded documentation.

### You're All Set...

One last thing though. I built this all while homeless, and I'm still _sort-of_ homeless after leaving my career in software to focus on a [modified model of relativity](https://flusterapp.com/blog/by_path/on_the_gravitational_nature_of_time) full-time. If you see the value in Conundrum and the Conundrum ecosystem of tools, please consider supporting the project. While this project is still in it's early days, just imagine what it can become if I can actually afford an internet connection.

> Oh yea... while not open-source in the truest sense of the word, the project gives everything beyond the median developer salary to charity. See the license [here](https://flusterapp.com/license).

[^1]: The note you're looking at right now is compiled from Conundrum
`}
                />
            </div>
        </div>
    );
};

OnboardingWrapup.displayName = "OnboardingWrapup";
