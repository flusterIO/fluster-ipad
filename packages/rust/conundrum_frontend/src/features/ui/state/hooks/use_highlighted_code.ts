import { useEffect, useEffectEvent, useState } from "react";
import { rspc } from "@/app/rspc_client";
import consola from "consola";
import { type Procedures } from "@/codegen/bindings_t";

export type HighlightCodeRequest =
  Procedures["code"]["highlight_code"]["input"];

export interface UseHighlightedCodeProps {
  req: HighlightCodeRequest;
}

/**
 * Returns null while the content is loading.
 */
export const useHighlightedCode = ({
  req,
}: UseHighlightedCodeProps): string | null => {
  const [content, setContent] = useState<null | string>(null);
  const { mutateAsync } = rspc.useMutation("code.highlight_code", {});
  const getContent = useEffectEvent(async (_req: HighlightCodeRequest) => {
    try {
      const res = await mutateAsync(_req);
      consola.info("Sending highlight request.");
      setContent(res);
    } catch (err: unknown) {
      consola.error(`Error: ${err}`);
    }
  });
  useEffect(() => {
    getContent(req).catch((err: unknown) => {
      consola.error(`Error: ${err}`);
    });
  }, [req]);
  return content;
};
