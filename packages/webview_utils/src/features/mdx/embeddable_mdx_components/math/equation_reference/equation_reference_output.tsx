import { cn } from '@/utils/cn';
import React, { type ReactNode } from 'react'
import { type ZodStylesGroup } from '../../schemas/emphasis_schema';


interface EquationReferenceOutputProps {
    idx: number;
    id: string;
    super?: boolean
    sub?: boolean
    emphasisClasses?: ZodStylesGroup
}

export const EquationReferenceOutput = ({ idx, super: _super, sub: subscript, emphasisClasses }: EquationReferenceOutputProps): ReactNode => {

    const handleEquationTagClick = (): void => {
        const em = document.querySelector(`[data-conundrum-eq-idx="${idx}"]`);
        console.log("em: ", em)
        if (em) {
            em.scrollIntoView({
                behavior: "smooth",
                block: "start",
                inline: "start"
            })
        }
    }

    if (_super) {
        return (
            <sup style={emphasisClasses?.css} onClick={handleEquationTagClick} className={cn("text-sm", emphasisClasses?.classes)}>{idx + 1}</sup>
        )
    } else if (subscript) {
        return (
            <sub style={emphasisClasses?.css} onClick={handleEquationTagClick} className={cn("text-sm", emphasisClasses?.classes)}>{idx + 1}</sub>
        )
    } else {
        return (
            <span style={emphasisClasses?.css} className={cn("font-bold", emphasisClasses?.classes)} onClick={handleEquationTagClick}>{idx + 1}</span>
        )
    }
}


EquationReferenceOutput.displayName = "EquationReferenceOutput"
