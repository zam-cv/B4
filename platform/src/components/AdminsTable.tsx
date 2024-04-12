import { API_URL } from "@/utils/constants";
import { useEffect, useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import { getConfig } from "../utils/auth";
import axios from "axios";

export type Payment = {
  id: string;
  email: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "email",
    header: "Correo electrónico",
  },
];

async function getData(): Promise<Payment[]> {
  let admins = await axios.get(`${API_URL}/admins`, await getConfig());
  return admins.data;
}

export default function AdminsTable({
  setAdminsId,
  setAdminsInfo,
}: {
  setAdminsId: React.Dispatch<React.SetStateAction<string | null>>;
  setAdminsInfo: React.Dispatch<React.SetStateAction<Payment | null>>;
}) {
  const [data, setData] = useState<Payment[]>([]);

  useEffect(() => {
    getData().then(setData);
  }, []);

  return (
    <DataTable
      columns={columns}
      data={data}
      setUserId={setAdminsId}
      setUserInfo={setAdminsInfo}
    />
  );
}
