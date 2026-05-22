export const topicToUrl = (topic: string, baseUrl: string): string => {
    return `${baseUrl}?topic=${topic}`;
};

export const subjectToUrl = (subject: string, baseUrl: string): string => {
    return `${baseUrl}?subject=${subject}`;
};

export const tagsToAnyOfUrl = (tags: string[], baseUrl: string): string => {
    const sp = new URLSearchParams();
    for (const t of tags) {
        sp.append("tagAnyOf", t);
    }
    return `${baseUrl}?=${sp.toString()}`;
};

export const tagsToAllOfUrl = (tags: string[], baseUrl: string): string => {
    const sp = new URLSearchParams();
    for (const t of tags) {
        sp.append("tagAllOf", t);
    }
    return `${baseUrl}?=${sp.toString()}`;
};
