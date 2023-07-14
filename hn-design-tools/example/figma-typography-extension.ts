import * as figma from "./figma.gen.ts";

const figmaWeightsGroup = figma.FigmaTextStyleMatrixGroup({
  Description: "Font weight",
  Options: [
    { Name: "Base", Tokens: "" },
    { Name: "Thin", Tokens: "w100" },
    { Name: "Extra Light", Tokens: "w200" },
    { Name: "Light", Tokens: "w300" },
    { Name: "Normal", Tokens: "w400" },
    { Name: "Medium", Tokens: "w500" },
    { Name: "Semi Bold", Tokens: "w600" },
    { Name: "Bold", Tokens: "w700" },
    { Name: "Extra Bold", Tokens: "w800" },
    { Name: "Black", Tokens: "w900" },
  ],
});
const figmaItalicGroup = figma.FigmaTextStyleMatrixGroup({
  Description: "Font italicized",
  Options: [
    { Name: "Base", Tokens: "" },
    { Name: "Italic", Tokens: "italic" },
  ],
});
const figmaProseStyleGroup = figma.FigmaTextStyleMatrixGroup({
  Description: "Prose stylization",
  Options: [
    { Name: "Base", Tokens: "" },
    { Name: "Code", Tokens: "code" },
  ],
});
export const figmaTypographyExtension = figma.FigmaTypographyConfig({
  FigmaTextStyles: [
    {
      BaseName: "Content",
      BaseTokens: "text content",
      Groups: [
        {
          Options: [
            { Name: "Smaller", Tokens: "xs" },
            { Name: "Small", Tokens: "sm" },
            { Name: "Base", Tokens: "base" },
            { Name: "Quote", Tokens: "lg w500" },
            { Name: "Heading 3", Tokens: "lg w700", Description: "Use gray color" },
            { Name: "Heading 2", Tokens: "xl w700" },
            { Name: "Heading 1", Tokens: "2xl w700" },
            { Name: "Hero Title (3XL)", Tokens: "3xl w700" },
            { Name: "Hero Title (4XL)", Tokens: "4xl w700" },
          ],
        },
        figmaWeightsGroup,
        figmaProseStyleGroup,
        figmaItalicGroup,
      ],
    },
    {
      BaseName: "UI",
      BaseTokens: "text ui",
      Groups: [
        {
          Description: "text size",
          Options: [
            { Name: "Smaller", Tokens: "xs" },
            { Name: "Small", Tokens: "sm" },
            { Name: "Base", Tokens: "base" },
            { Name: "Large", Tokens: "lg" },
            { Name: "Larger", Tokens: "xl" },
            // Add 2X if you like.
            { Name: "3X Large", Tokens: "3xl" },
          ],
        },
        figmaWeightsGroup,
        figmaProseStyleGroup,
        figmaItalicGroup,
      ],
    },
    {
      BaseName: "Codeblock",
      BaseTokens: "text code base",
      Groups: [figmaWeightsGroup, figmaItalicGroup],
    },
  ],
});
