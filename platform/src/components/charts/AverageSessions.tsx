import Chart from ".";
import { useEffect, useState } from "react";
import { Bar } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import { API_URL } from "../../utils/constants";
import { getConfig } from "../../utils/auth";
import axios from "axios";

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
    (async () => {
      const config = await getConfig();
      axios
        .get(`${API_URL}/users/average-sessions`, config)
        .then(({ data }: { data: any }) => {
          setAverageSessions(data);
        });
    })();
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
