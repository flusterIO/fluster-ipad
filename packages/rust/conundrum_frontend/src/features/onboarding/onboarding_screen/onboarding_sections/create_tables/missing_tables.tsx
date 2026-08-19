import { CdrmContent } from "#/cdrm/cdrm_content";
import { type HealthReport } from "#/database/db_utility_types/health";
import { cn } from "@/utils/shad_utils";
import React, {
    type CSSProperties,
    useEffect,
    useRef,
    type ReactNode,
} from "react";

interface MissingTablesProps {
    table: HealthReport["table_reports"][number];
    className?: string;
    active: boolean;
    setActiveHeight: (h: number) => void;
    style: CSSProperties;
}

export const MissingTable = ({
    table,
    className,
    active,
    setActiveHeight,
    style,
}: MissingTablesProps): ReactNode => {
    const ref = useRef<HTMLDivElement>(null);
    const sendHeight = (): void => {
        const h = ref.current?.getBoundingClientRect().height;
        if (h) {
            setActiveHeight(h);
        }
    };
    useEffect(() => {
        if (!ref.current) {
            return;
        }
        if (active) {
            sendHeight();
        }
    }, [active]);
    return (
        <div
            className={cn(
                "w-full flex flex-col justify-start items-start indent-0",
                className,
            )}
            ref={ref}
            style={style}
        >
            <div className="font-bold text-sm">{table.description.entity_name}</div>
            <div className="text-sm w-full">
                <CdrmContent
                    em="div"
                    className="w-full"
                    content={table.description.description}
                    onLoad={() => {
                        sendHeight();
                    }}
                />
            </div>
        </div>
    );
};

MissingTable.displayName = "MissingTables";
