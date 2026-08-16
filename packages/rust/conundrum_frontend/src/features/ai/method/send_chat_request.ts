export async function sendMessage(message: string) {
    const response = await fetch("/api/ws", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            message,
        }),
    });

    if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
    }

    if (!response.body) {
        throw new Error("Response has no body");
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();

    let accumulated = "";

    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    while (true) {
        const { value, done } = await reader.read();

        if (done) {
            break;
        }

        const text = decoder.decode(value, { stream: true });
        console.log("text: ", text)

        accumulated += text;

        // Update your UI here
        console.log(text);
    }

    // Optional: flush any remaining decoder state
    const decoded = decoder.decode();
    console.log("decoded: ", decoded)
    accumulated += decoded;

    return accumulated;
}
