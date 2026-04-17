// (() => {
//     const tabGroups = document.getElementsByClassName("cdrm-tab-group") as HTMLCollectionOf<HTMLDivElement>;

//     const alignTabGroup = (t: HTMLDivElement): void => {
//         const tabs = t.querySelectorAll("cdrm-tab-group-tab") as NodeListOf<HTMLDivElement>;
//         tabs.forEach((tabItem) => {
//         })
//     }

//     for (const tabGroup of tabGroups) {
//         alignTabGroup(tabGroup)
//     }
// })()

const handleTabClick = (e: Event) => {
    const em = e.target as HTMLDivElement
    let lastIdx = em.id.lastIndexOf("-");
    let groupId = ""
    let idx = ""
    for (var i = 0; i < em.id.length; i++) {
        const item = em.id[i]
        if (i < lastIdx) {
            groupId += item
        } else if (i > lastIdx) {
            idx += item
        }
    }
    const group = document.getElementById(groupId) as HTMLDivElement;

    let oldActiveIdx = group.classList.values().find((s) => s.startsWith("active-"))?.replace("active-", "");
    if (typeof oldActiveIdx === "undefined") {
        return
    }
    if (group) {
        group.classList.add(`active-${idx}`)
    }
}
