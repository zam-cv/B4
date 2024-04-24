import { useEffect, useState } from "react";
import { API_URL } from "@/utils/constants";
import { getConfig } from "@/utils/auth";
import axios from "axios";
import { DataTable } from "./DataTable";

// Definición de las columnas para la tabla de eventos
const columns = [
  {
    accessorKey: "content",
    header: "Contenido",
  },
];

interface Event {
  event_type: "Positive" | "Negative",
  content: string,
}

export default function Events() {
  const [positives, setPositives] = useState<Event[]>([]);
  const [negatives, setNegatives] = useState<Event[]>([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();
      let { data: events } = await axios.get(`${API_URL}/data/events`, config);
      setPositives(events.filter((event: Event) => event.event_type === "Positive"));
      setNegatives(events.filter((event: Event) => event.event_type === "Negative"));
    })();
  }, []);

  return (
    <div className="p-5 grid grid-rows-[auto_1fr] h-full">
      <h1 className="text-2xl font-bold text-blue-950">Lista de Eventos</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
        <div className="grid grid-rows-[auto_1fr] gap-5">
          <h2 className="text-xl font-bold">Positivos</h2>
          <div className="relative w-full h-full overflow-auto">
            <div className="absolute w-full h-full">
                <DataTable columns={columns} data={positives} />
            </div>
          </div>
        </div>
        <div className="grid grid-rows-[auto_1fr] gap-5">
          <h2 className="text-xl font-bold">Negativos</h2>
          <div className="relative w-full h-full overflow-auto">
            <div className="absolute w-full h-full">
                <DataTable columns={columns} data={negatives} />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
