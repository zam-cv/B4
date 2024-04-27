import Chart from ".";
import { useEffect, useState } from "react";
import { Bar } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import api from "@/utils/api";

export default function UsersByGender() {
  const [range, setRanges] = useState<[string, number][]>([]);

  useEffect(() => {
    api.users.getUsersByAgeRange().then((data) => {
      setRanges(data);
    });
  }, []);

  return (
    <Chart title="Cantidad de usuarios por rango de edades">
      <Bar
        data={{
          labels: range.map(([range]) => range),
          datasets: [
            {
              label: "Usuarios",
              data: range.map(([, count]) => count),
              backgroundColor: getColors(range.length),
            },
          ],
        }}
        options={CHART_DEFAULT_OPTIONS}
      />
    </Chart>
  );
}
