/**
 * Returns true if the error is this shit error that I can't get rid of that's super indescriptive and not helpful at all.
 */
export const isShitError = (errorMessage: string): boolean => {
    return errorMessage === "Resolver(ResolverError(None))"
}
