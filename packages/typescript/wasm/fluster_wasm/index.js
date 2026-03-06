import init from "./pkg/";

export const initializeFlusterWasm = async () => {
  try {
    await init();
  } catch (err) {
    console.error("Error: ", err);
  }
};
