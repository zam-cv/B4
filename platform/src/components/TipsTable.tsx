import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";

export type Payment = {
  id: number;
  content: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "content",
    header: "Tips",
  }
];

export default function UsersTable({ data }: { data: Payment[] }) {
  return (
    <DataTable
      columns={columns}
      data={data}
    />
  );
}
