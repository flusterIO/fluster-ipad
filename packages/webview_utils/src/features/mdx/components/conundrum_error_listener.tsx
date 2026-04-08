import { type GlobalWebviewStateDeepNullable } from '#/webview_global_state/cross_language_state_types';
import { useIsomorphicLayoutEffect } from '@/state/hooks/use_isomorphic_layout_effect';
import { type ReactNode } from 'react'
import { connect } from 'react-redux';

const connector = connect((state: GlobalWebviewStateDeepNullable) => ({
    errors: state.conundrum.errors
}))

interface ConundrumErrorListenerProps {
    errors: GlobalWebviewStateDeepNullable["conundrum"]["errors"]
}


export class ConundrumErrorWrapper extends Error { }



export const ConundrumErrorListener = connector(({ errors }: ConundrumErrorListenerProps): ReactNode => {
    useIsomorphicLayoutEffect(() => {
        if (errors.length) {
            // Throw a useless error to trigger the UI that can read from state.
            throw new ConundrumErrorWrapper()
        }
    }, [errors])
    return (
        null
    )
})


ConundrumErrorListener.displayName = "ConundrumErrorListener"
