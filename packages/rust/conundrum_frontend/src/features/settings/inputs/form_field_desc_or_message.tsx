import React, { type ReactNode } from 'react'
import { FormMessage } from "@/components/shad/form";



interface FormFieldDescOrMessageProps {
    desc?: ReactNode
}

export const FormFieldDescOrMessage = ({ desc }: FormFieldDescOrMessageProps): ReactNode => {
    return (
        <FormMessage>
            {desc ? <div className="text-sm text-foreground/60!">{desc}</div> : undefined}
        </FormMessage>
    )
}


FormFieldDescOrMessage.displayName = "FormFieldDescOrMessage"
