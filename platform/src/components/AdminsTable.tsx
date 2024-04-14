import { API_URL } from "@/utils/constants";
import { useEffect } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import { getConfig } from "../utils/auth";
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

      return (
        <div className="flex justify-end px-2 hover:text-red-600">
          <span className="cursor-pointer" onClick={deleteAdmin}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={1.5}
              stroke="currentColor"
              className="w-6 h-6"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
              />
            </svg>
          </span>
        </div>
      );
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
      setUserInfo(null)
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
