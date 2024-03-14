import { API_URL } from "@/utils/constants";
import { useEffect, useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import { getToken } from "../hooks/useAuth";
import axios from "axios";

type Payment = {
  id: string;
  username: string;
  user_type: string;
  email: string;
  gender: string;
  age: number;
  os: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "username",
    header: "Nombre de usuario",
  },
  {
    accessorKey: "user_type",
    header: "Tipo de usuario",
  },
  {
    accessorKey: "email",
    header: "Correo electrónico",
  },
  {
    accessorKey: "gender",
    header: "Genero",
  },
  {
    accessorKey: "age",
    header: "Edad",
  },
  {
    accessorKey: "os",
    header: "Sistema operativo",
  },
];

async function getData(): Promise<Payment[]> {
  let date = new Date();

  let users = await axios.get(`${API_URL}/users`, {
    withCredentials: true,
    headers: { token: await getToken() },
  });

  users.data.forEach((user: any) => {
    user.age = date.getFullYear() - user.year_of_birth;
  })

  return users.data;
}

export default function PlayersTable() {
  const [data, setData] = useState<Payment[]>([]);

  useEffect(() => {
    getData().then(setData);
  }, []);

  return <DataTable columns={columns} data={data} />;
}
