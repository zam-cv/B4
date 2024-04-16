import { API_URL } from "@/utils/constants";
import { useEffect, useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import { getConfig } from "../utils/auth";
import axios from "axios";

export type Payment = {
  id: string;
  content: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "content",
    header: "Tips",
  }
];

async function getData(): Promise<Payment[]> {
  let users = await axios.get(`${API_URL}/data/tips`, await getConfig());

  return users.data;
}

export default function UsersTable() {
  const [data, setData] = useState<Payment[]>([]);

  useEffect(() => {
    (async () => {
      setData(await getData());
    })();
  }, []);

  return (
    <DataTable
      columns={columns}
      data={data}
    />
  );
}
