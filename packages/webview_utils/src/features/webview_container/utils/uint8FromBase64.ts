export function uint8FromBase64String(base64String: string): Uint8Array {
    // Decode Base64 string to a binary string
    const binaryString = window.atob(base64String);
    const len = binaryString.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    // 'bytes' is the original Uint8Array data
    return bytes
}
