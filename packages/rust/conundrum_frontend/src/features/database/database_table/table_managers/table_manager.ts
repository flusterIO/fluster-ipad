import { type RowData, type ColumnDef } from "@tanstack/react-table";
import consola from "consola";

interface DatabaseTableManagerProps {
    /**
     * TODO: Convert this to a `DatabaseTable` once that code has been generated in typescript.
     * This must be unique
     */
    table: string;
}

export abstract class DatabaseTableManager<DataType extends RowData> {
    props: DatabaseTableManagerProps;
    constructor(props: DatabaseTableManagerProps) {
        this.props = props;
    }

    abstract getColumns(): ColumnDef<DataType>[];
    abstract getData(perPage: number, page: number): Promise<DataType[]>;
    abstract entityName(): string;
    static table(): string {
        consola.error(
            "THis should never be reached. This method must be overriden.",
        );
        return "";
    }
}
