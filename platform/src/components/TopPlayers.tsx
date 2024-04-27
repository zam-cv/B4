import { useState, useEffect } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import { API_URL } from "@/utils/constants";
import { getConfig } from "../utils/auth";
import axios from "axios";

export type Payment = {
  name: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "name",
    header: "Nombre",
  },
];

export default function TopPlayers() {
  const [data, setData] = useState([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      const { data } = await axios.get(
        `${API_URL}/players/top-players`,
        config
      );
      console.log(data);

      setData(data.map((player: any) => ({ name: player })));
    })();
  }, []);

  return (
    <div className="grid grid-rows-[auto_1fr] w-full h-full">
      <div className="text-xl font-bold pt-5 text-blue-950 text-center">
        Top jugadores
      </div>
      <div className="w-full h-full py-5">
        <DataTable columns={columns} data={data} />
      </div>
    </div>
  );
}
