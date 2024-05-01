import { useState, useEffect } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "./DataTable";
import api from "@/utils/api";

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
  const [data, setData] = useState<Payment[]>([]);

  useEffect(() => {
    api.players.getTopPlayers().then((players) => {
      setData(players.map((player) => ({ name: player })));
    });
  }, []);

  return (
    <div className="grid grid-rows-[auto_1fr] w-full h-full p-5">
      <div className="text-xl font-bold text-blue-950 text-center">
        Top jugadores
      </div>
      <div className="w-full h-full py-5">
        <DataTable columns={columns} data={data} />
      </div>
    </div>
  );
}
