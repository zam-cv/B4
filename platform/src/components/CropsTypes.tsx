import { useState, useEffect } from "react";
import { DataTable } from "./DataTable";
import { API_URL } from "@/utils/constants";
import { ColumnDef } from "@tanstack/react-table";
import { getConfig } from "../utils/auth";
import ContentEdit from "./ContentEdit";
import axios from "axios";

export type Payment = {
  name: string;
  price: number;
  duration: number;
  description: string;
};

export const columns: ColumnDef<Payment>[] = [
  {
    accessorKey: "name",
    header: "Nombre",
  },
  {
    accessorKey: "price",
    header: "Precio",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        const config = await getConfig();

        axios
          .put(
            `${API_URL}/data/crops/${row.row.original.name}/price`,
            ref?.textContent,
            config
          )
          .catch((error) => {
            console.error(error);
          });
      }

      return (
        <ContentEdit
          value={row.row.original.price.toString()}
          onInput={handleContentChange}
        />
      );
    }
  },
  {
    accessorKey: "duration",
    header: "Duración",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        const config = await getConfig();

        axios
          .put(
            `${API_URL}/data/crops/${row.row.original.name}/duration`,
            ref?.textContent,
            config
          )
          .catch((error) => {
            console.error(error);
          });
      }

      return (
        <ContentEdit
          value={row.row.original.duration.toString()}
          onInput={handleContentChange}
        />
      );
    }
  },
  {
    accessorKey: "description",
    header: "Descripción",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        const config = await getConfig();

        axios
          .put(
            `${API_URL}/data/crops/${row.row.original.name}/description`,
            ref?.textContent,
            config
          )
          .catch((error) => {
            console.error(error);
          });
      }

      return (
        <ContentEdit
          value={row.row.original.description}
          onInput={handleContentChange}
        />
      );
    },
  },
];

export default function CropsTypes() {
  const [data, setData] = useState<Payment[]>([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      axios.get(`${API_URL}/data/crops`, config).then(({ data }) => {
        setData(data);
      });
    })();
  }, []);

  return (
    <div className="p-5 flex flex-col gap-5 h-full">
      <h1 className="text-2xl font-bold">Editar tipos de Cultivo</h1>
      <div className="relative w-full h-full overflow-auto">
        <div className="absolute w-full h-full">
          <DataTable columns={columns} data={data} />
        </div>
      </div>
    </div>
  );
}
