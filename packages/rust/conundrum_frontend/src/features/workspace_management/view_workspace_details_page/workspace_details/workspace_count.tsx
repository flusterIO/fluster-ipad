import { type WorkspaceByPredicate } from "#/database/db_utility_types/workspace";
import { rspc } from "@/app/rspc_client";
import { cn } from "@/utils/shad_utils";
import consola from "consola";
import React, { useMemo, type ReactNode } from "react";
import { WorkspaceCountCard } from "./workspace_count_card";
import {
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/shad/chart";
import { Label, Pie, PieChart } from "recharts";
import {
    CardHeader,
    CardContent,
    CardTitle,
    Card,
    CardDescription,
    CardFooter,
} from "@/components/shad/card";
import { type ParsableFileType } from "@/codegen/bindings";

interface WorkspaceCountProps {
    workspace: WorkspaceByPredicate;
    classes?: {
        cardContainer?: string;
    };
}

export const parsableFileTypeLabelMap = {
    md: "Markdown",
    cdrm: "Conundrum",
    mdx: "Mdx",
    npy: "Numpy",
    typst: "Typst",
    ipynb: "Notebook",
    json: "Json",
    pdf: "Pdf",
    html: "Html",
} satisfies Record<ParsableFileType, string>;

export const WorkspaceCount = ({
    workspace,
    classes = {},
}: WorkspaceCountProps): ReactNode => {
    const { data } = rspc.useQuery([
        "workspace_management.parsable_file_count",
        workspace.root,
    ]);
    const mappedData = useMemo(() => {
        if (data) {
            return Object.keys(data.count).map((k, i) => {
                return {
                    value: data.count[k],
                    label: k,
                    fill: `var(--chart-${(i % 12) + 1})`,
                };
            });
        } else {
            return undefined;
        }
    }, [data]);
    if (!data) {
        return null;
    }
    return (
        <Card className={cn("flex flex-col my-4", classes.cardContainer)}>
            <CardHeader>
                <CardTitle>Parsable File Count</CardTitle>
                <CardDescription>
                    This is the total number of parsable files in this workspace.
                </CardDescription>
            </CardHeader>
            <CardContent className="flex-1 pb-0">
                <ChartContainer
                    className="mx-auto aspect-square max-h-62.5 w-auto p-2"
                    config={{}}
                >
                    <PieChart>
                        <ChartTooltip cursor={false} content={<ChartTooltipContent />} />
                        <Pie
                            innerRadius={60}
                            strokeWidth={5}
                            data={mappedData}
                            nameKey={"label"}
                            dataKey={"value"}
                        >
                            <Label
                                content={({ viewBox }) => {
                                    if (viewBox && "cx" in viewBox && "cy" in viewBox) {
                                        return (
                                            <text
                                                x={viewBox.cx}
                                                y={viewBox.cy}
                                                textAnchor="middle"
                                                dominantBaseline="middle"
                                            >
                                                <tspan
                                                    x={viewBox.cx}
                                                    y={viewBox.cy}
                                                    className="fill-foreground text-3xl font-bold"
                                                >
                                                    {Object.values(data.count).reduce((a, b) => a + b)}
                                                </tspan>
                                                <tspan
                                                    x={viewBox.cx}
                                                    y={(viewBox.cy || 0) + 24}
                                                    className="fill-muted-foreground"
                                                >
                                                    Parsable Files
                                                </tspan>
                                            </text>
                                        );
                                    }
                                }}
                            />
                        </Pie>
                    </PieChart>
                </ChartContainer>
            </CardContent>
            <CardFooter className="flex flex-row justify-between items-start flex-wrap gap-x-3">
                {mappedData
                    ? mappedData.map((d) => {
                        return (
                            <div
                                key={d.label}
                                className="grid grid-cols-[1fr_auto] gap-x-1"
                            >
                                <div
                                    className="w-3 h-3 rounded border place-self-center"
                                    style={{ backgroundColor: d.fill }}
                                />
                                <div className="w-full wrap-break-word">{d.label}</div>
                            </div>
                        );
                    })
                    : null}
            </CardFooter>
        </Card>
    );
};

WorkspaceCount.displayName = "WorkspaceCount";
