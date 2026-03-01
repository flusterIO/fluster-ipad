import React, { useMemo } from 'react';
import { z } from "zod"
import { _sizableObjectSchema, sizableObjectSchema } from '../../../mdx/embeddable_mdx_components/schemas/sizable_object_schema';
import { embeddableResponsiveGridPropsSchema } from '#/mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props';


type ZodSchemaTableKey = "sizable-object" | "grid-props"

interface ZodTableProps {

    schema: ZodSchemaTableKey
}

export type SchemaRow = {
    field: string;
    type: string;
    required: boolean;
    description: string;
};

function flattenZodSchema(schema: z.ZodTypeAny, keys: string[], prefix = ""): SchemaRow[] {
    if (!(schema instanceof z.ZodObject)) return [];

    const rows: SchemaRow[] = [];
    const shape = schema.shape;

    for (const key in keys) {
        const field = shape[key];
        const name = prefix ? `${prefix}.${key}` : key;

        // Unwrap ZodOptional, ZodNullable, etc.
        let coreType = field;
        while (coreType._def.innerType) {
            coreType = coreType._def.innerType;
        }

        const typeName = coreType._def.typeName.replace("Zod", "");

        rows.push({
            field: name,
            type: typeName,
            required: !field.isOptional(),
            description: field.description ?? "No description",
        });

        // Recursively handle nested objects
        if (typeName === "Object") {
            rows.push(...flattenZodSchema(
                /* eslint-disable-next-line  -- Need to use any here. */
                coreType as z.ZodObject<any>, Object.keys(coreType), name));
        }
    }
    return rows;
}

export const InContentDocumenationSchemaTable: React.FC<ZodTableProps> = ({ schema }) => {
    /* eslint-disable-next-line  -- Need to use any here */
    const schemaMap: { [K in ZodSchemaTableKey]: z.ZodTypeAny } = {
        "sizable-object": sizableObjectSchema,
        "grid-props": embeddableResponsiveGridPropsSchema
    }
    const data = useMemo(() => flattenZodSchema(schemaMap[schema], Object.keys(_sizableObjectSchema)), [schema, schemaMap]);

    return (
        <div className="overflow-hidden rounded-lg border border-gray-200 shadow-sm">
            <table className="min-w-full divide-y divide-gray-200 bg-white text-sm text-left">
                <thead className="bg-gray-50 text-gray-700 uppercase text-xs font-semibold">
                    <tr>
                        <th className="px-4 py-3">Field</th>
                        <th className="px-4 py-3">Type</th>
                        <th className="px-4 py-3">Req</th>
                        <th className="px-4 py-3">Description</th>
                    </tr>
                </thead>
                <tbody className="divide-y divide-gray-200">
                    {data.map((row) => (
                        <tr key={row.field} className="hover:bg-gray-50 transition-colors">
                            <td className="px-4 py-3 font-mono text-blue-600 font-medium">
                                {row.field}
                            </td>
                            <td className="px-4 py-3">
                                <span className="px-2 py-1 rounded bg-gray-100 text-gray-600 border text-xs">
                                    {row.type}
                                </span>
                            </td>
                            <td className="px-4 py-3">
                                {row.required ? (
                                    <span className="text-red-500">Yes</span>
                                ) : (
                                    <span className="text-gray-400">No</span>
                                )}
                            </td>
                            <td className="px-4 py-3 text-gray-600 italic">
                                {row.description}
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};
