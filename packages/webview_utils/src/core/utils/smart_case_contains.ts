export const smartCaseContains = (body: string, query: string) => {
    const q = query.toLowerCase()
    const isLower = q == query
    if (isLower) {
        return body.toLowerCase().includes(q)
    } else {
        return body.includes(query)
    }
}
