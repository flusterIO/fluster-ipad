export const mdxClasses = "prose dark:prose-invert prose-p:text-foreground prose-code:w-fit prose-code:min-w-full prose-code:before:content-none prose-code:after:content-none prose-code:bg-[var(--shiki-light-bg)] dark:prose-code:bg-[var(--shiki-dark-bg)] [&_code_*]:text-[var(--shiki-light)] dark:[&_code_*]:text-[var(--shiki-dark)] "

export const shrinkMdxClasses = "[&_p]:mb-0 [&_p]:mt-0 [&_p]:font-normal"

export const inlineMdxClasses = `${shrinkMdxClasses} [&>p]:my-0 inline-mdx`
