import React, { useEffect, useState, type ReactNode } from "react";
import { ModularDashboardCard } from "../../modular_dashboard_card";
import { EmptyCardDataText } from "../empty_card_text";
import { TaggableListItem } from "./taggable_list_item";
import { AppPaths } from "#/navigation/app_paths";
import { faker } from "@faker-js/faker/locale/en";
import { TaggablesListWrapper } from "./taggables_list_wrapper";

export const TagsDashboardCard = (): ReactNode => {
    const [tags, setTags] = useState<{ value: string }[]>([]);
    useEffect(() => {
        const tags: { value: string }[] = [];
        Array(faker.number.int({ min: 5, max: 20 }))
            .fill(0)
            .forEach(() => {
                tags.push({ value: faker.lorem.words({ min: 1, max: 5 }) });
            });
        setTags(tags);
    }, []);
    return (
        <ModularDashboardCard title="Tags" className="pb-0">
            <TaggablesListWrapper>
                {tags.length ? (
                    tags.map((t) => {
                        const sp = new URLSearchParams();
                        sp.set("byTag", t.value);
                        return (
                            <TaggableListItem key={t.value} href={`${AppPaths.search}?${sp.toString()}`}>
                                {t.value}
                            </TaggableListItem>
                        );
                    })
                ) : (
                    <EmptyCardDataText>No tags found</EmptyCardDataText>
                )}
            </TaggablesListWrapper>
        </ModularDashboardCard>
    );
};

TagsDashboardCard.displayName = "TagsDashboardCard";
