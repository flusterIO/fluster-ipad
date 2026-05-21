import { cn } from "#/lib/cn";
import { type ReactNode } from "react";

export interface BlogSidebarItem {
    label: string;
    href: string;
    active?: boolean;
}

export interface BlogSidebarCategoryProps {
    label: string;
    items: BlogSidebarItem[];
    icon: ReactNode;
}

export const BlogSidebarCategory = ({
    label,
    items,
    icon,
}: BlogSidebarCategoryProps): ReactNode => {
    return (
        <div className="pl-2 pr-4 mt-2">
            <div className="font-semibold grid grid-cols-[auto_1fr] place-items-center gap-x-2">
                {icon}
                <div className="w-full text-left">{label}</div>
            </div>
            <div>
                {items.length ? (
                    items.map((item) => {
                        return (
                            <a
                                key={`${item.href}-${item.label}`}
                                href={item.href}
                                className={cn(
                                    "text-muted-foreground text-sm pl-6",
                                    item.active && "text-foreground/90",
                                )}
                            >
                                {item.label}
                            </a>
                        );
                    })
                ) : (
                    <div className="text-muted-foreground text-sm pl-6">None</div>
                )}
            </div>
        </div>
    );
};
