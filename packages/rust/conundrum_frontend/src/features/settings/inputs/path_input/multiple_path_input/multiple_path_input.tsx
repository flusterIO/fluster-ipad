import React, { type ReactNode } from "react";
import { FieldValues, Path } from "react-hook-form";
import { PathList } from "./path_list";
import { Input } from "@/components/shad/input";

interface MultiplePathInputProps<T extends FieldValues> {
    name: Path<T>;
}

export const MultiplePathInput = (props: MultiplePathInputProps): ReactNode => {
    return (
        <div className="w-full flex flex-col justify-center items-center">
            <PathList />
            <Input className="w-full font-mono" />
        </div>
    );
};

MultiplePathInput.displayName = "MultiplePathInput";
