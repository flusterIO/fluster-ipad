import { type MdxEditorAppState } from './store';
import { type CodeEditorImplementation } from '@/code_gen/typeshare/fluster_core_utilities';
import { initialMdxEditorState } from './initial_mdx_editor_state';

export interface EditorStoreSetup {
    editorImplementation: CodeEditorImplementation
}

export type StoreSetupData = EditorStoreSetup


/**
 * Want to ideally inject state on first load to avoid a flash, but it's not worth the time right now.
 * Revisit it if the flash is noticable even with loading components.
 */
export const getPreloadedState = (setup: StoreSetupData): MdxEditorAppState => {
    return {
        ...initialMdxEditorState,
        container: {
            ...initialMdxEditorState.container,
            editorImplementation: setup.editorImplementation
        }
    }
}
