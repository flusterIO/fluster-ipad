import { Button } from '@/shared_components/shad/button'
import React, { ReactPortal } from 'react'
import { createPortal } from 'react-dom'
import { loadFakeNote } from './test_methods/set_test_content'


export const DevelopmentWindowUtils = (): ReactPortal => {
    return createPortal((
        <div className="w-fit h-fit text-sm fixed top-4 left-4 rounded bg-black text-white dark:bg-white dark:text-black p-2">
            <div>Utils</div>
            <div className="w-fit flex flex-row justify-center items-center gap-2">
                <Button className="text-sm" onClick={loadFakeNote}>Load Note</Button>
            </div>
        </div>
    ), document.body!)
}


DevelopmentWindowUtils.displayName = "DevelopmentWindowUtils"
