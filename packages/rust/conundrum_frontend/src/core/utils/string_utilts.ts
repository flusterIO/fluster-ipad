export const capitalize = (val: string): string => {
    if (!val.length) {
        return "";
    }
    return `${val[0].toUpperCase()}${val.slice(1)}`;
};
