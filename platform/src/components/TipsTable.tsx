import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import Delete from "./Delete";
import ContentEdit from "./ContentEdit";
import api, { Tip } from "@/utils/api";

export const columns: ColumnDef<Tip>[] = [
  {
    accessorKey: "content",
    header: "Tips",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        api.data.updateTip(
          row.row.original.id,
          ref?.textContent || ""
        );
      }

      return (
        <ContentEdit
          value={row.row.original.content}
          onInput={handleContentChange}
        />
      );
    },
  },
  {
    id: "actions",
    enableHiding: false,
    cell: ({ row }) => {
      const payment = row.original;

      async function deleteTip() {
        api.data.deleteTip(payment.id).then(() => {
          // @ts-ignore
          payment.deleteTip(payment.id);
        })
      }

      return <Delete onClick={deleteTip} />;
    },
  },
];

export default function TipsTable({
  data,
  setData,
}: {
  data: Tip[];
  setData: React.Dispatch<React.SetStateAction<Tip[]>>;
}) {
  function deleteTip(id: number) {
    let newData = data.filter((tip) => tip.id !== id);
    setData(newData);
  }

  return (
    <DataTable
      columns={columns}
      data={data.map((tip) => ({ ...tip, deleteTip }))}
    />
  );
}
