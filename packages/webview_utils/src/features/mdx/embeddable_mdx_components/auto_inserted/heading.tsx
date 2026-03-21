import React, { type ReactNode } from 'react'
import { H1, H2, H3, H4, H5, H6 } from '../../../../core/shared_components/typography/typography';



interface AutoInsertedHeadingProps {
    depth: 1 | 2 | 3 | 4 | 5 | 6;
    id: string | null;
    children: ReactNode;
}

export const AutoInsertedHeading = ({ depth, id, children }: AutoInsertedHeadingProps): ReactNode => {
    switch (depth) {
        case 1: {
            return <H1 id={id ?? undefined}>{children}</H1>
        }
        case 2: {
            return <H2 id={id ?? undefined}>{children}</H2>
        }
        case 3: {
            return <H3 id={id ?? undefined}>{children}</H3>
        }
        case 4: {
            return <H4 id={id ?? undefined}>{children}</H4>
        }
        case 5: {
            return <H5 id={id ?? undefined}>{children}</H5>
        }
        case 6: {
            return <H6 id={id ?? undefined}>{children}</H6>
        }
    }
}


AutoInsertedHeading.displayName = "AutoInsertedHeading"
