import {useEffect, useState} from "react";

export const useAleoWASM = () => {
    const [aleo, setAleo] = useState(null);

    useEffect(() => {
        if (aleo === null) {
            import('../../wasm/pkg').then(module => setAleo(module));
        }
    }, []);  // eslint-disable-line react-hooks/exhaustive-deps
    return aleo;
};