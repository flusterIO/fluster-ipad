import React from "react";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import flusterWasm from "@fluster/wasm/fluster";
flusterWasm().then((a) => console.log("Wasm Output: ", a));

/* import whyDidYouRender from "@welldone-software/why-did-you-render"; */
/* whyDidYouRender(React, { trackAllPureComponents: true }); */

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
