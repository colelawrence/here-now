export type FontStyleRule = {
  CSS: { FontWeight: string | number } | { FontStyle: string } | { Variation: { Key: string; Value: string } };
  Figma: { Suffix: string } | { Variation: { Key: string; Value: string } };
};
