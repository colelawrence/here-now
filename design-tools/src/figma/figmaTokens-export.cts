// Exporter for the "Figma Tokens Studio" plugin
// Todo: replace with Figma Plugin for loading the variables
import { writeFileSync } from "fs";
import { noCase } from "no-case";
import {
  SystemColorSettings,
  TypographySettings,
  createArtpromptColorsFromSourceHex,
  designSystemColorSettings,
  designSystemTypographySettings,
  hexFromArgb,
} from "../designSystem.cjs";
import { getAllColorsKebab } from "../color/getAllColorsKebab.cjs";

const _example = {
  Typography: {
    "text-title-lg": {
      value: {
        fontFamily: "$title",
        lineHeight: "28px",
        fontSize: "22.7273px",
        letterSpacing: "0.0005em",
      },
      type: "typography",
    },
    title: {
      value: "Forecaster Work Sans",
      type: "fontFamilies",
    },
  },
  "Blue Light": {
    "sys-primary": {
      value: "#e41111",
      type: "color",
    },
  },
};

const altPrimarySets: { Name: string; SeedHex: string }[] = [
  { Name: "Green", SeedHex: "#23a200" },
  { Name: "Blue", SeedHex: "#0023f2" },
  { Name: "Pink", SeedHex: "#ffd223" },
];

console.log("figmaTokens-export");
const COLOR_ID_ATTRIBUTES_SET = new Set(["container", "inverse", "on"]);

exportForFigma({
  typography: designSystemTypographySettings,
  color: designSystemColorSettings,
});

export function exportForFigma(settings: {
  typography: TypographySettings;
  color: SystemColorSettings;
}) {
  const typographyTokens: {
    [id: string]:
      | {
          type: "typography";
          value: {
            /** "$title" */
            fontFamily: string;
            /** "28px" */
            lineHeight: string;
            /** "22.7273px" */
            fontSize: string;
            /** "0.0005em */
            letterSpacing: string;
          };
        }
      | {
          type: "fontFamilies";
          // "Forecaster Work Sans"
          value: string;
        };
  } = {};
  type ColorSet = {
    [id: string]: {
      /** "#e41111" */
      value: string;
      type: "color";
    };
  };

  const colorTokens: Record<string, ColorSet> = {};

  for (const fam of settings.typography.FontFamilies) {
    typographyTokens[fam.ID] = {
      type: "fontFamilies",
      value: fam.FigmaFontFamilyName,
    };
  }
  for (const font of settings.typography.Fonts) {
    typographyTokens[font.ID] = {
      type: "typography",
      value: {
        fontFamily: `$${font.Value.FontFamily.ref$}`,
        fontSize: font.Value.FontSize,
        letterSpacing: font.Value.Tracking,
        lineHeight: font.Value.LineHeight,
      },
    };
  }

  const colorSets: { Name: string; PrimaryHex: string }[] = [
    { Name: "Default", PrimaryHex: settings.color.config.Primary.Hex },
    ...altPrimarySets.map((set) => ({
      Name: set.Name,
      PrimaryHex: set.SeedHex,
    })),
  ];

  const allColorSets = colorSets
    .map((set) => ({
      Name: set.Name,
      theme: createArtpromptColorsFromSourceHex(
        set.PrimaryHex,
        settings.color.config.extended
      ),
    }))
    .flatMap((set) => [
      {
        Name: `${set.Name} Light`,
        Colors: getAllColorsKebab(set.theme, { dark: false }),
      },
      {
        Name: `${set.Name} Dark`,
        Colors: getAllColorsKebab(set.theme, { dark: true }),
      },
    ]);

  for (const color of allColorSets) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const colorSet = {} as any;
    for (const c of color.Colors) {
      const prefix = [c.ext ? "ext" : "sys"];
      const postfix = [];
      for (const part of noCase(c.id, { delimiter: " " }).split(" ")) {
        if (COLOR_ID_ATTRIBUTES_SET.has(part)) {
          postfix.push(part);
        } else {
          prefix.push(part);
        }
      }
      // ? need to add -color suffix so the colors don't merge over the nests
      postfix.push("color");
      let layerObj = colorSet;
      for (const layer of prefix) {
        layerObj[layer] = layerObj[layer] ?? {};
        if (layerObj.type === "color")
          console.error("Found conflicting item key", {
            prefix,
            postfix,
            layerObj,
          });
        layerObj = layerObj[layer];
      }
      const itemKey = postfix.join("-");
      if (layerObj[itemKey])
        console.error("Found conflicting item key", {
          prefix,
          postfix,
          itemKey,
        });
      layerObj[postfix.join("-")] = {
        type: "color",
        value: hexFromArgb(c.argb),
      };
    }
    colorTokens[color.Name] = colorSet;
  }

  const [outputFile] = process.argv.slice(-1);
  const { "Default Light": defaultColorSet, ...alternativeColorSets } =
    colorTokens;
  const outputJSON = {
    Defaults: {
      ...typographyTokens,
      ...defaultColorSet,
    },
    ...alternativeColorSets,
  };
  if (outputFile && outputFile.endsWith(".json")) {
    console.error(`Writing ${outputFile}`);
    writeFileSync(outputFile, JSON.stringify(outputJSON, null, 2));
  } else {
    console.error(
      "output json file not specified as last argument",
      outputJSON
    );
  }
}
