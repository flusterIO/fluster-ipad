export const initializeFlusterWasm = async () => {
    import("./pkg/index.js").then((init) => init());
};
