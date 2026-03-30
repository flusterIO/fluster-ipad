import { createSlice, type PayloadAction } from '@reduxjs/toolkit'
import { initialMediaState } from './initial_media_state'

export interface CounterState {
    value: number
}

export const mediaSlice = createSlice({
    name: 'media',
    initialState: initialMediaState,
    reducers: {
        placeHolderSetImage(state, action: PayloadAction<number[]>) {
            console.log(state, action)
        }
    },
    // extraReducers: (builder) => {
    //     builder.addCase(handleSwiftAction, (state): MediaState => {
    //         return swiftMediaActionReducer(state)
    //     })
    //     return builder
    // }
})

// // Action creators are generated for each case reducer function
// export const { } = webviewContainerSlice.actions

export default mediaSlice.reducer
