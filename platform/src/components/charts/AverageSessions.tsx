import Chart from ".";
import { useEffect, useState } from "react";
import { Bar } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import api from "@/utils/api";

const DAYS = [
  "Domingo",
  "Lunes",
  "Martes",
  "Miercoles",
  "Jueves",
  "Viernes",
  "Sabado",
];

export default function AverageSessions() {
  const [averageSessions, setAverageSessions] = useState<[number, number][]>(
    []
  );

  useEffect(() => {
    api.users.getAverageSessions().then((data) => {
      setAverageSessions(data);
    });
  }, []);

  return (
    <Chart title="Trafico de usuarios en los dias de la semana">
      <Bar
        data={{
          labels: averageSessions.map(([index]) => DAYS[index - 1]),
          datasets: [
            {
              label: "Usuarios",
              data: averageSessions.map(([, count]) => count),
              backgroundColor: getColors(DAYS.length),
            },
          ],
        }}
        options={CHART_DEFAULT_OPTIONS}
      />
    </Chart>
  );
}
