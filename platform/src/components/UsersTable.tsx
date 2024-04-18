import { API_URL } from "@/utils/constants";
import { useEffect, useState } from "react";
import { DataTable } from "./DataTable";
import { getConfig } from "../utils/auth";
import { Button } from "@/components/ui/button";
import { ArrowUpDown } from "lucide-react";
import { ColumnDef } from "@tanstack/react-table";
import axios from "axios";

export type Payment = {
  id: string;
  username: string;
  user_type: string;
  email: string;
  gender: string;
  age: number;
  os: string;
  latitude: number;
  longitude: number;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "username",
    header: "Nombre de usuario",
  },
  {
    accessorKey: "user_type",
    header: ({ column }) => {
      return (
        <Button
          variant="ghost"
          onClick={() =>
            column.toggleSorting(column.getIsSorted() === "asc", true)
          }
        >
          Tipo de usuario
          <ArrowUpDown className="ml-2 h-4 w-4" />
        </Button>
      );
    },
  },
  {
    accessorKey: "email",
    header: "Correo electrónico",
  },
  {
    accessorKey: "gender",
    header: ({ column }) => {
      return (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
        >
          Género
          <ArrowUpDown className="ml-2 h-4 w-4" />
        </Button>
      );
    },
  },
  {
    accessorKey: "age",
    header: ({ column }) => {
      return (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === "asc", true)}
        >
          Edad
          <ArrowUpDown className="ml-2 h-4 w-4" />
        </Button>
      );
    },
  },
  {
    accessorKey: "os",
    header: "Sistema operativo",
  },
];

async function getData(
  setUserId: React.Dispatch<React.SetStateAction<string | null>>,
  setUserInfo: React.Dispatch<React.SetStateAction<Payment | null>>
): Promise<Payment[]> {
  let date = new Date();
  let users = await axios.get(`${API_URL}/users`, await getConfig());

  users.data.forEach((user: any) => {
    user.age = date.getFullYear() - user.year_of_birth;
  });

  setUserId(users.data[0].id);
  setUserInfo(users.data[0]);
  return users.data;
}

export default function UsersTable({
  setUserId,
  setUserInfo,
}: {
  setUserId: React.Dispatch<React.SetStateAction<string | null>>;
  setUserInfo: React.Dispatch<React.SetStateAction<Payment | null>>;
}) {
  const [data, setData] = useState<Payment[]>([]);

  useEffect(() => {
    getData(setUserId, setUserInfo).then(setData);
  }, []);

  return (
    <DataTable
      columns={columns}
      data={data}
      setUserId={setUserId}
      setUserInfo={setUserInfo}
    />
  );
}
