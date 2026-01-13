import React, { type ReactNode } from "react";
import { useLoaderData } from "react-router";

export const MdxNotePage = (): ReactNode => {
    const data = useLoaderData();
    console.log("data: ", data);
    return <div></div>;
};

MdxNotePage.displayName = "MdxNotePage";
