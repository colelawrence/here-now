/** `#[codegen(tags = "data-browser", template = "data-collections")]` */
export type DataCollections = {
  header: PageHeader;
};
/** `#[codegen(tags = "data-browser", template = "data-collections")]` */
export function DataCollections(inner: DataCollections): DataCollections {
  return inner;
}
/** `#[codegen(tags = "data-browser")]` */
export type PageHeader = {
  title: string;
  /** label, then href */
  links: Array<[string, string]>;
  warning?: string | undefined | null | null | undefined;
};
/** `#[codegen(tags = "data-browser")]` */
export function PageHeader(inner: PageHeader): PageHeader {
  return inner;
}
/** `#[codegen(tags = "data-browser", template = "collection-page")]` */
export type CollectionPage = {
  header: PageHeader;
  rows: Array<CollectionRow>;
};
/** `#[codegen(tags = "data-browser", template = "collection-page")]` */
export function CollectionPage(inner: CollectionPage): CollectionPage {
  return inner;
}
/** `#[codegen(tags = "data-browser")]` */
export type CollectionRow = {
  /** `#[codegen(ts_as = "string")]` */
  id: string;
  /** `#[codegen(ts_as = "unknown")]` */
  content: unknown;
  ecs_content?: string | undefined | null | null | undefined;
};
/** `#[codegen(tags = "data-browser")]` */
export function CollectionRow(inner: CollectionRow): CollectionRow {
  return inner;
}