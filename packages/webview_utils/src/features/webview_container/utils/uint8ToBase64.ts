export function uint8ArrayToBase64String(uint8Array: Uint8Array): string {
    let i = uint8Array.length;
    const binaryString = new Array(i);
    while (i--) {
        binaryString[i] = String.fromCharCode(uint8Array[i]);
    }
    const data = binaryString.join('');
    return window.btoa(data); // Encode to Base64
}
