export const initializeConundrumWasm = async () => {
    import("./pkg/index.js").then((init) => init());
};
