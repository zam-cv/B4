import { useRef } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { API_URL } from "@/utils/constants";
import { DataTable } from "./DataTable";
import { getConfig } from "@/utils/auth";
import axios from "axios";

export type Payment = {
  id: number;
  content: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "content",
    header: "Tips",
    cell: (row) => {
      const textareaRef = useRef<HTMLParagraphElement>(null);

      async function handleContentChange() {
        const config = await getConfig();

        axios
          .put(`${API_URL}/data/tips/update/${row.row.original.id}`, {
            content: textareaRef.current?.textContent,
          }, config)
          .catch((error) => {
            console.error(error);
          });
      }

      // Prevent the user from creating a new line when pressing enter
      function handleKeyDown(event: React.KeyboardEvent<HTMLParagraphElement>) {
        if (event.key === "Enter") {
          event.preventDefault();
        }
      }

      return (
        <p
          ref={textareaRef}
          contentEditable
          className="outline-none"
          onInput={handleContentChange}
          onKeyDown={handleKeyDown}
        >
          {row.row.original.content}
        </p>
      );
    },
  },
];

export default function UsersTable({ data }: { data: Payment[] }) {
  return <DataTable columns={columns} data={data} />;
}
