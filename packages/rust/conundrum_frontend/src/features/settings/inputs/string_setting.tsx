import { Input } from "@/components/shad/input";
import { Label } from "@/components/shad/label";
import React, { useState, type ReactNode } from "react";

interface StringSettingProps {
    settingKey: string;
    label: ReactNode;
    desc?: ReactNode;
}

export const StringSetting = ({
    label,
    desc,
}: StringSettingProps): ReactNode => {
    const [value, setValue] = useState("");
    return (
        <div className="w-full h-fit flex flex-col justify-start items-start gap-y-2">
            <Label>{label}</Label>
            <Input
                value={value}
                onChange={(e) => {
                    setValue(e.target.value);
                }}
            />
            {desc ? (
                <div className="text-sm text-muted-foreground">{desc}</div>
            ) : null}
        </div>
    );
};

StringSetting.displayName = "StringSetting";
