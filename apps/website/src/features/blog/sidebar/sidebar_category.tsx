import { type ReactNode } from "react";

export interface BlogSidebarItem {
    label: string;
    href: string;
}

export interface BlogSidebarCategoryProps {
    label: string;
    items: BlogSidebarItem[];
}

export const BlogSidebarCategory = ({
    label,
    items,
}: BlogSidebarCategoryProps): ReactNode => {
    return (
        <div>
            <div>{label}</div>
            <div>
                {items.map((item) => {
                    return <a href={item.href}>{item.label}</a>;
                })}
            </div>
        </div>
    );
};
