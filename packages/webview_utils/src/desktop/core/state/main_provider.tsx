import React, { useState, type ReactNode } from 'react'
import { DesktopScaffoldProvider } from 'src/desktop/features/scaffold/main_scaffold/state/main_scaffold_provider'
import ReduxProvider from './redux_provider'
import { ErrorBoundary } from 'react-error-boundary'
import { AppErrorView } from 'src/desktop/features/error_handling/presentation/app_error_view'



interface MainDesktopProviderProps {
    children: ReactNode
}

export const MainDesktopProvider = ({ children }: MainDesktopProviderProps): ReactNode => {
    const [appError] = useState<string | null>(null)
    return (
        <ReduxProvider>
            <DesktopScaffoldProvider>
                <ErrorBoundary
                    onError={(e) => console.error("error: ", e)}
                    fallback={<AppErrorView error={appError} />}
                >
                    {children}
                </ErrorBoundary>
            </DesktopScaffoldProvider>
        </ReduxProvider>
    )
}


MainDesktopProvider.displayName = "MainDesktopProvider"
