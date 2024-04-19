import { ColumnDef } from "@tanstack/react-table";
import { API_URL } from "@/utils/constants";
import { DataTable } from "./DataTable";
import { getConfig } from "@/utils/auth";
import Delete from "./Delete";
import axios from "axios";
import ContentEdit from "./ContentEdit";

export type Payment = {
  id: number;
  content: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "content",
    header: "Tips",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        const config = await getConfig();

        axios
          .put(
            `${API_URL}/data/tips/${row.row.original.id}`,
            {
              content: ref?.textContent,
            },
            config
          )
          .catch((error) => {
            console.error(error);
          });
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
        const config = await getConfig();

        axios.delete(`${API_URL}/data/tips/${payment.id}`, config).then(() => {
          // @ts-ignore
          payment.deleteTip(payment.id);
        });
      }

      return <Delete onClick={deleteTip} />;
    },
  },
];

export default function TipsTable({
  data,
  setData,
}: {
  data: Payment[];
  setData: React.Dispatch<React.SetStateAction<Payment[]>>;
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
