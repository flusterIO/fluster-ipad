
// superJsonTransform.js
import { createTransform } from 'redux-persist';
import SuperJSON from 'superjson';

// PERFORMANCE: Fix this... Implement a flatbuffer serializer for the entire global state, both the editor and the container at least for performance reasons.
const superJsonTransform = createTransform(
    (inboundState) => {
        return SuperJSON.serialize(inboundState);
    },
    (outboundState) => {
        return SuperJSON.deserialize(outboundState);
    }
);

export default superJsonTransform;
