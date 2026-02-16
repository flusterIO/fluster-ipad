import { parser } from "./fluster_math.grammar";
import {
  LRLanguage,
  LanguageSupport,
  indentNodeProp,
  foldNodeProp,
  foldInside,
  delimitedIndent,
} from "@codemirror/language";
import { styleTags, tags as t } from "@lezer/highlight";

export const pythonBraceLanguage = LRLanguage.define({
  parser: parser.configure({
    props: [
      // 1. Indentation
      indentNodeProp.add({
        Block: delimitedIndent({ closing: "}", align: false }),
        ObjectExpression: delimitedIndent({ closing: "}", align: false }),
        ArrayExpression: delimitedIndent({ closing: "]", align: false }),
        ParamList: delimitedIndent({ closing: ")", align: false }),
        ArgList: delimitedIndent({ closing: ")", align: false }),
      }),

      // 2. Code Folding
      foldNodeProp.add({
        Block: foldInside,
        ObjectExpression: foldInside,
        ArrayExpression: foldInside,
      }),

      // 3. Syntax Highlighting
      styleTags({
        "def class if elif else while for return in": t.controlKeyword,
        "and or not": t.logicOperator,
        "True False": t.bool,
        None: t.null,

        Identifier: t.variableName,
        "CallExpression/Identifier": t.function(t.variableName),
        "FunctionDefinition/Identifier": t.function(
          t.definition(t.variableName),
        ),
        "ClassDefinition/Identifier": t.className,
        "MemberExpression/Identifier": t.propertyName,
        "ObjectProp/Identifier": t.propertyName,
        "Decorator/Identifier": t.meta, // Highlights @decorator

        Type: t.typeName,
        String: t.string,
        Number: t.number,
        Comment: t.lineComment,

        ArithOp: t.arithmeticOperator,
        CompareOp: t.compareOperator,
        LogicOp: t.logicOperator,
        "=": t.definitionOperator,

        "( )": t.paren,
        "[ ]": t.squareBracket,
        "{ }": t.brace,
        ".": t.derefOperator,
        ", ; :": t.punctuation,
      }),
    ],
  }),
  languageData: {
    commentTokens: { line: "#" },
    closeBrackets: { brackets: ["(", "[", "{", "'", '"'] },
    indentOnInput: /^\s*(\}|\]|\)|\:)$/,
  },
});

export function pythonBraceSupport() {
  return new LanguageSupport(pythonBraceLanguage);
}
