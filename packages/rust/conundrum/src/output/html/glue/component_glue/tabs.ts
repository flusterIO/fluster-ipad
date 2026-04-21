(() => {
    const data = document.getElementsByClassName("cdrm-tab-group");
    for (var i = 0; i < data.length; i++) {
        const tabGroup = data.item(i);
        const tabs = tabGroup.querySelectorAll(".cdrm-tab-group-tab");
        console.log("tabs: ", tabs)
    }
})();


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
