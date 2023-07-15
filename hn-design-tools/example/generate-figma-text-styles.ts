import { figmaTypographyExtension } from "./figma-typography-extension.ts";
import { allTokensSampleData } from "./sample-output-data.ts";
import * as output from "./output.gen.ts";

// Maybe more of this logic should be in Rust or in Figma?
// Hard to say since we also want to support custom combinations in
// Figma plugin, which require this querying logic to be executed in
// the plugin itself.

class TypographyTokenLookup {
  constructor(private tokens: output.TypographyExport) {}
  query(tokens: string[]): output.TypographyProperty[] {
    // precedence + props
    const found: [number, number[]][] = [];
    possible: for (const [reqs, propIdxs] of this.tokens.tokens) {
      let precedence = -1;
      for (const req of reqs) {
        const idx = tokens.indexOf(req);
        if (idx === -1) {
          continue possible;
        }
        precedence = Math.max(precedence, idx);
      }

      // matched
      found.push([precedence, propIdxs]);
    }

    const allProps: output.TypographyProperty[] = [];
    const byPrecedence = found.sort((a, b) => a[0] - b[0]);
    for (const [_, idxs] of byPrecedence) {
      for (const idx of idxs) {
        allProps.push(this.tokens.properties[idx]);
      }
    }
    return allProps;
  }
}

function harness(tokens: output.TypographyExport) {
  const lookup = new TypographyTokenLookup(tokens);
  return {
    query(tokens: TemplateStringsArray) {
      const tokensTrimmed = splitTokens(String.raw(tokens));
      console.log(tokensTrimmed.join(", "), lookup.query(tokensTrimmed));
    },
  };
}

const SPLIT_RE = /[,\s]+/g;
function splitTokens(x: string): string[] {
  const trimmed = x.trim();
  if (trimmed.length === 0) return [];
  return trimmed.split(SPLIT_RE);
}

const h = harness(allTokensSampleData);
h.query`W100 mono`;
h.query`W100 text content W200 xs`;
h.query`text content base quote W800`;
/*
W100, text, content, W200, xs [
  { FontStyle: { CSS: null, Figma: null } },
  { FontFamily: { family_name: "Inter" } },
  { FontStyle: { CSS: null, Figma: null } },
  { FontSize: { px: 10.197560814372599 } },
  { LetterSpacing: { px: 0.04092898010051371 } },
  { LineHeight: { px: 16.499999999999993 } }
]
*/

// for (const textStyle of figmaTypographyExtension.FigmaTextStyles) {
//   let allTextStyles: {
//     names: string[];
//     /** split and flattened */
//     tokens: string[];
//   }[] = [{ names: [textStyle.BaseName], tokens: splitTokens(textStyle.BaseTokens) }];
//   for (const group of textStyle.Groups) {
//     const originalTextStyles = allTextStyles;
//     allTextStyles = new Array(originalTextStyles.length * group.Options.length);
//     let i = 0;
//     for (const original of originalTextStyles) {
//       for (const option of group.Options) {
//         allTextStyles[i] = {
//           names: [...original.names, option.Name],
//           tokens: [...original.tokens, ...splitTokens(option.Tokens)],
//         };
//         i++;
//       }
//     }
//   }

//   // console.log(allTextStyles);

//   const len = allTextStyles.length;
//   const figmaTextStyles: any[] = new Array(len);
//   const lookup = new TypographyTokenLookup(allTokensSampleData);
//   for (let i = 0; i < len; i++) {
//     const textStyle = allTextStyles[i];
//     figmaTextStyles[i] = {
//       name: textStyle.names.join("/"),
//       props: lookup.query(textStyle.tokens),
//     };
//   }

//   // console.log(figmaTextStyles)
// }
