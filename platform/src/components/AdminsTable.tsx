import { API_URL } from "@/utils/constants";
import { useEffect } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import { getConfig } from "../utils/auth";
import Delete from "./Delete";
import axios from "axios";

export type Payment = {
  id: string;
  email: string;
  role_id: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "email",
    header: "Correo electrónico",
  },
  {
    id: "actions",
    enableHiding: false,
    cell: ({ row }) => {
      const payment = row.original;

      async function deleteAdmin() {
        const config = await getConfig();

        axios.delete(`${API_URL}/admins/${payment.id}`, config).then(() => {
          // @ts-ignore
          payment.deleteAdmin(payment.id);
        });
      }

      return <Delete onClick={deleteAdmin} />;
    },
  },
];

async function getData(
  setUserId: React.Dispatch<React.SetStateAction<string | null>>,
  setUserInfo: React.Dispatch<React.SetStateAction<Payment | null>>
): Promise<Payment[]> {
  let admins = await axios.get(`${API_URL}/admins`, await getConfig());
  setUserId(admins.data[0].id);
  setUserInfo(admins.data[0]);
  return admins.data;
}

export default function AdminsTable({
  data,
  setData,
  setUserId,
  setUserInfo,
}: {
  data: Payment[];
  setData: React.Dispatch<React.SetStateAction<Payment[]>>;
  setUserId: React.Dispatch<React.SetStateAction<string | null>>;
  setUserInfo: React.Dispatch<React.SetStateAction<Payment | null>>;
}) {
  useEffect(() => {
    getData(setUserId, setUserInfo).then(setData);
  }, []);

  function deleteAdmin(id: string) {
    let newData = data.filter((admin) => admin.id !== id);

    setData(newData);

    let first = newData[0];

    if (first) {
      setUserId(first.id);
      setUserInfo(first);
    } else {
      setUserInfo(null);
    }
  }

  return (
    <DataTable
      columns={columns}
      data={data.map((a) => ({ ...a, deleteAdmin }))}
      setUserId={setUserId}
      setUserInfo={setUserInfo}
    />
  );
}
