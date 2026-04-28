export const capitalize = (s: string): string => {
    return `${s[0].toUpperCase()}${s.slice(1, s.length)}`
}




export const copyStringToClipboard = async (content: string): Promise<boolean> => {
    try {
        await window.navigator.clipboard.writeText(content)
        return true
    } catch {
        return false
    }
}

/**
 * If `s` ends with `x` then just returns `x`, otherwise appends `x` to `s`
 */
export const withTrailingSlash = (s: string, x = "/") => s.endsWith(x) ? s : `${s}${x}`
