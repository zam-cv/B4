import { useEffect, useState } from "react";
import { DataTable } from "./DataTable";
import { Button } from "@/components/ui/button";
import { ArrowUpDown } from "lucide-react";
import { ColumnDef } from "@tanstack/react-table";
import api, { User } from "@/utils/api";

export const columns: ColumnDef<User>[] = [
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

export default function UsersTable({
  setUserId,
  setUserInfo,
}: {
  setUserId: React.Dispatch<React.SetStateAction<string | null>>;
  setUserInfo: React.Dispatch<React.SetStateAction<User | null>>;
}) {
  const [data, setData] = useState<User[]>([]);

  useEffect(() => {
    api.users.getUsers().then((users) => {
      let date = new Date();

      users.forEach((user) => {
        // @ts-ignore
        user.age = date.getFullYear() - user.year_of_birth;
      });

      setUserId(users[0].id);
      setUserInfo(users[0]);
      setData(users);
    });
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
