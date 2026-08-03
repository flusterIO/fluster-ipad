import { client } from "@/app/rspc_client";

export const pathExists = async (fp: string): Promise<boolean> => {
    return await client.query(["fs.path_exists", fp]);
};
