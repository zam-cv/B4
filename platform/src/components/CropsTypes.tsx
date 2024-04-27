import { useState, useEffect } from "react";
import { DataTable } from "./DataTable";
import { ColumnDef } from "@tanstack/react-table";
import ContentEdit from "./ContentEdit";
import api, { Crop } from "@/utils/api";

export const columns: ColumnDef<Crop>[] = [
  {
    accessorKey: "name",
    header: "Nombre",
  },
  {
    accessorKey: "price",
    header: "Precio",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        api.data.updateCropPrice(
          row.row.original.name,
          parseFloat(ref?.textContent || "0")
        );
      }

      return (
        <ContentEdit
          value={row.row.original.price.toString()}
          onInput={handleContentChange}
        />
      );
    },
  },
  {
    accessorKey: "duration",
    header: "Duración",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        api.data.updateCropDuration(
          row.row.original.name,
          parseFloat(ref?.textContent || "0")
        );
      }

      return (
        <ContentEdit
          value={row.row.original.duration.toString()}
          onInput={handleContentChange}
        />
      );
    },
  },
  {
    accessorKey: "description",
    header: "Descripción",
    cell: (row) => {
      async function handleContentChange(ref: HTMLParagraphElement | null) {
        api.data.updateCropDescription(
          row.row.original.name,
          ref?.textContent || ""
        );
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
  const [data, setData] = useState<Crop[]>([]);

  useEffect(() => {
    api.data.getCrops().then((crops) => {
      setData(crops);
    });
  }, []);

  return (
    <div className="p-5 flex flex-col gap-5 h-full">
      <h1 className="text-2xl font-bold text-blue-950">
        Editar tipos de Cultivo
      </h1>
      <div className="relative w-full h-full overflow-auto">
        <div className="absolute w-full h-full">
          <DataTable columns={columns} data={data} />
        </div>
      </div>
    </div>
  );
}
