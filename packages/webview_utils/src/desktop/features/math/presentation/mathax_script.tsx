
import React, { type ReactNode } from "react";
import { ResourceRoutes } from "src/desktop/core/resources/resource_routes";

const MathjaxScript = (): ReactNode => {
    return (
        <>
            <script src={ResourceRoutes.mathjax} async />
        </>
    );
};

MathjaxScript.displayName = "MathjaxScript";

export default MathjaxScript;
