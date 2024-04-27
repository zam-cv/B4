import { useEffect } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import Delete from "./Delete";
import api, { Admin } from "@/utils/api";

export const columns: ColumnDef<Admin>[] = [
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
        await api.admins.deleteAdmin(payment.id);
        // @ts-ignore
        payment.deleteAdmin(payment.id);
      }

      return <Delete onClick={deleteAdmin} />;
    },
  },
];

export default function AdminsTable({
  data,
  setData,
  setUserId,
  setUserInfo,
}: {
  data: Admin[];
  setData: React.Dispatch<React.SetStateAction<Admin[]>>;
  setUserId: React.Dispatch<React.SetStateAction<string | null>>;
  setUserInfo: React.Dispatch<React.SetStateAction<Admin | null>>;
}) {
  useEffect(() => {
    api.admins.getAdmins().then((admins) => {
      setUserId(admins[0].id);
      setUserInfo(admins[0]);
      setData(admins);
    });
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
