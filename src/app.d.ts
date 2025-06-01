import "@tanstack/table-core";
import { RowData } from "@tanstack/table-core";

declare module "@tanstack/table-core" {
  interface ColumnMeta<TData extends RowData, TValue> {
    class?: string;
  }
}
