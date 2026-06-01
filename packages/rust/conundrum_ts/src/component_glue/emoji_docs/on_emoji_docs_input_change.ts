export const onEmojiDocsInputChange = (e: Event) => {
    const target = e.currentTarget as HTMLInputElement | undefined;
    if (!target) {
        console.error("Could not locate source of docs input change.");
        return;
    }

    if (window.conundrum.searchConundrumEmojisAndAppendToContainer) {
        console.log(`Searching emojis with value: ${target.value}...`);
        window.conundrum.searchConundrumEmojisAndAppendToContainer(target.value);
        const targetDiv = document.getElementById("emoji-docs-content");
        if (targetDiv) {
            console.info("Give up on wasm and use this targetDiv");
        }
    } else {
        console.error(
            "Tried to call window.conundrum.searchEmojis but the function doesn't exist.",
        );
    }
};
