import React, { type ReactNode } from 'react'
import { H1, H2, H3, H4, H5, H6 } from '../../../../core/shared_components/typography/typography';
import { cn } from '@/utils/cn';



interface AutoInsertedHeadingProps {
    depth: 1 | 2 | 3 | 4 | 5 | 6;
    id: string | null;
    children: ReactNode;
    subtitle: ReactNode | null;
}


const TitleWithSubTitleContainer = ({ children }: { children: ReactNode }): ReactNode => {
    return (
        <div className="w-full flex flex-col justify-start items-start">
            {children}
        </div>
    )
}


const Subtitle = ({ children, depth, className }: { children: ReactNode, depth: number, className?: string }): ReactNode => {
    return (
        <div className={cn("text-sm text-muted-foreground not-prose", depth <= 2 ? "mb-8" : "mb-6", className)}>{children}</div>
    )
}

export const AutoInsertedHeading = ({ depth, id, children, subtitle }: AutoInsertedHeadingProps): ReactNode => {
    switch (depth) {
        case 1: {
            return subtitle ? (
                <TitleWithSubTitleContainer>
                    <H1 className="mb-0" id={id ?? undefined}>{children}</H1>
                    <Subtitle depth={depth}>
                        {subtitle}
                    </Subtitle>
                </TitleWithSubTitleContainer>
            ) : (
                <H1 id={id ?? undefined}>{children}</H1>
            )
        }
        case 2: {
            return subtitle ? (
                <TitleWithSubTitleContainer>
                    <H2 className="mb-0 w-full" id={id ?? undefined}>{children}</H2>
                    <Subtitle className="mt-2" depth={depth}>
                        {subtitle}
                    </Subtitle>
                </TitleWithSubTitleContainer>
            ) : (
                <H2 id={id ?? undefined}>{children}</H2>
            )
        }
        case 3: {
            return subtitle ? (
                <TitleWithSubTitleContainer>
                    <H3 className="mb-0" id={id ?? undefined}>{children}</H3>
                    <Subtitle depth={depth}>
                        {subtitle}
                    </Subtitle>
                </TitleWithSubTitleContainer>
            ) : (
                <H3 id={id ?? undefined}>{children}</H3>
            )
        }
        case 4: {
            return subtitle ? (
                <TitleWithSubTitleContainer>
                    <H4 className="mb-0" id={id ?? undefined}>{children}</H4>
                    <Subtitle depth={depth}>
                        {subtitle}
                    </Subtitle>
                </TitleWithSubTitleContainer>
            ) : (
                <H4 id={id ?? undefined}>{children}</H4>
            )
        }
        case 5: {
            return subtitle ? (
                <TitleWithSubTitleContainer>
                    <H5 className="mb-0" id={id ?? undefined}>{children}</H5>
                    <Subtitle depth={depth}>
                        {subtitle}
                    </Subtitle>
                </TitleWithSubTitleContainer>
            ) : (
                <H5 id={id ?? undefined}>{children}</H5>
            )
        }
        case 6: {
            return subtitle ? (
                <TitleWithSubTitleContainer>
                    <H6 className="mb-0" id={id ?? undefined}>{children}</H6>
                    <Subtitle depth={depth}>
                        {subtitle}
                    </Subtitle>
                </TitleWithSubTitleContainer>
            ) : (
                <H6 id={id ?? undefined}>{children}</H6>
            )
        }
    }
}


AutoInsertedHeading.displayName = "AutoInsertedHeading"
