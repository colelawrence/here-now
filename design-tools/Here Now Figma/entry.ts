import { assign, KEEP } from "./assign.js";
import { fromEntries } from "./fromEntries.js";
import { gen } from "./gen/define.js";
import testData from "./test-data.js";
const allLocalVariables = figma.variables.getLocalVariables();

function ensureMode(c: VariableCollection, modeName: string) {
  const modeId =
    c.modes.find((a) => a.name === modeName)?.modeId ?? c.addMode(modeName);
  return {
    ensureAndSetVariable(variable: gen.Named<gen.FigmaVariable>) {
      const matchingExisting = allLocalVariables.filter(matchesName(variable));
      gen.FigmaVariable.match(variable.value, {
        Color(inner) {
          if (matchingExisting.length === 0) {
            matchingExisting.push(
              figma.variables.createVariable(variable.name, c.id, "COLOR")
            );
          }
          // this should probably only allow one variable
          for (const variable of matchingExisting) {
            variable.setValueForMode(modeId, inner);
          }
        },
        Length(inner) {
          if (matchingExisting.length === 0) {
            matchingExisting.push(
              figma.variables.createVariable(variable.name, c.id, "FLOAT")
            );
          }
          // this should probably only allow one variable
          for (const variable of matchingExisting) {
            variable.setValueForMode(modeId, inner);
          }
        },
      });
    },
  };
  // variable.setValueForMode(modeId, { r: 1, g: 0, b: 0, a: 1 });
}

const matchesName =
  (name: gen.Named<any>) =>
  (input: {
    name: string;
    description?: undefined | null | string;
  }): boolean => {
    if (input.name.split(/[\/ ]/g).find((a) => a === name.tailwind_id)) {
      return true;
    }
    // if (input.description && input.description.split(/[\/ ]/g).find(a => a === name.tailwind_id)) {
    //   return true
    // }
    return false;
  };

function ensureCollection(collectionName: string) {
  const collections = figma.variables.getLocalVariableCollections();
  return (
    collections.find((a) => a.name === collectionName) ??
    figma.variables.createVariableCollection(collectionName)
  );
}

console.log({
  // input
  textStyles: figma.getLocalTextStyles().map((a) => ({
    fontName: a.fontName,
    documentationLinks: a.documentationLinks,
    fontSize: a.fontSize,
    id: a.id,
    key: a.key,
    letterSpacing: a.letterSpacing,
    lineHeight: a.lineHeight,
    name: a.name,
  })),
});

async function importDesignSystem(data: gen.DesignSystem) {
  console.log("importing", data);
  await importDesignSystemTextStyles(data.text_styles);
  await importDesignSystemVariables(data.variables);
}
async function importDesignSystemTextStyles(
  styles: gen.DesignSystem["text_styles"]
) {
  const existingStyles = figma.getLocalTextStyles();
  for (const textStyle of styles) {
    const matchingExisting = existingStyles.filter(matchesName(textStyle));
    if (matchingExisting.length === 0) {
      const existingStyle = figma.createTextStyle();
      existingStyle.name = textStyle.name;
      matchingExisting.push(existingStyle);
    }

    const fontName: FontName = {
      family: textStyle.value.font_family,
      style: textStyle.value.font_style,
    };

    await figma.loadFontAsync(fontName);

    for (const existingStyle of matchingExisting) {
      assign(existingStyle, {
        fontName,
        fontSize: textStyle.value.font_size,
        description: textStyle.description || KEEP,
      });
    }
  }
}

async function importDesignSystemVariables(
  varis: gen.DesignSystem["variables"]
) {
  const collection = ensureCollection("Here Now");
  const mode = ensureMode(collection, "Light");
  // const existingVariablesRec = fromEntries(
  //   figma.variables.getLocalVariables().map((vari) => [vari.name, vari])
  // );
  for (const _key in varis) {
    const variable = varis[_key];
    mode.ensureAndSetVariable(variable);
  }
}

importDesignSystem(testData).then(() => {
  // Make sure to close the plugin when you're done. Otherwise the plugin will
  // keep running, which shows the cancel button at the bottom of the screen.
  figma.closePlugin();
});
