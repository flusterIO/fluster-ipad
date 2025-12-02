import { Checkbox } from "@/shared_components/shad/checkbox";
import { useIsomorphicLayoutEffect } from "@/state/hooks/use_isomorphic_layout_effect";
import React, { useRef, type ReactNode } from "react";

interface MdxCheckboxProps {
    checked?: boolean;
    defaultChecked?: boolean;
    disabled?: boolean;
}

export const MdxCheckbox = (props: MdxCheckboxProps): ReactNode => {
    const ref = useRef<HTMLButtonElement>(null!);
    const removeLiMarker = () => {
        if (!ref.current) return;
        if (ref.current.parentElement?.nodeName?.toLowerCase() === "li") {
            ref.current.parentElement.classList.add("hide-li-marker");
        }
    };

    useIsomorphicLayoutEffect(() => {
        removeLiMarker();
    }, []);

    return (
        <Checkbox
            ref={ref}
            disabled={props.disabled}
            checked={props.checked || props.defaultChecked}
        />
    );
};

MdxCheckbox.displayName = "MdxCheckbox";
