import { gen } from "./gen/define.js";
import { extend } from "./extend.js";

// const textStyle = figma.getLocalTextStyles()[0]
// textStyle.fontSize

const system = gen.DesignSystem({ text_styles: [], variables: [] });

function named<T>(id: string, display: string, value: T) {
  return { display, id, value };
}
function wght(id: string, weightValue: number, weightName: string) {
  return { id, weightValue, weightName };
}

const interFamily = {
  name: "Inter",
  weights: [
    wght("thin", 100, "Thin"),
    wght("extralight", 200, "Extra Light"),
    wght("light", 300, "Light"),
    wght("regular", 400, "Regular"),
    wght("medium", 500, "Medium"),
    wght("semibold", 600, "Semi Bold"),
    wght("bold", 700, "Bold"),
    wght("extrabold", 800, "Extra Bold"),
    wght("black", 900, "Black"),
  ],
};

for (const family of [
  named("content", "Content", interFamily),
  named("ui", "UI", interFamily),
]) {
  for (const size of [
    named("xs", "XSmall", { size: 7 }),
    named("sm", "Small", { size: 9 }),
    named("base", "Base", { size: 12 }),
    named("lg", "Large", { size: 16 }),
    named("xl", "XLarge", { size: 24 }),
  ]) {
    for (const weight of family.value.weights) {
      system.text_styles.push(
        gen.Named<gen.TextStyle>({
          name: `${family.display} Text/${size.display}/${weight}`,
          tailwind_id: `text-${family.id}-${size.id} font-${weight.id}`,
          description: `Inter (${weight.weightName})`,
          value: gen.TextStyle({
            font_family: "Inter",
            font_size: 12,
            font_style: weight.weightName,
            letter_spacing: 0.1,
            line_height: 14.5,
          }),
        })
      );
    }
  }
}

for (const color of [
  named("red", "Red", { b: 0, g: 0, r: 1 }),
  named("green", "Green", { b: 0, g: 1, r: 0 }),
  named("blue", "Blue", { b: 1, g: 0, r: 0 }),
]) {
  for (let i = 0; i < 10; i++) {
    system.variables.push(
      gen.Named({
        name: `Color/Red ${i + 1}00`,
        tailwind_id: `color-${color.id}-${i + 1}00`,
        value: gen.FigmaVariable.Color(
          extend({ a: (i + 1) * 0.1 }, color.value)
        ),
      })
    );
  }
}

export default system;
