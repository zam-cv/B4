import Chart from ".";
import { useEffect, useState } from "react";
import { Bar } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import { API_URL } from "../../utils/constants";
import { getConfig } from "../../utils/auth";
import axios from "axios";

export default function UsersByGender() {
  const [range, setRanges] = useState<[string, number][]>([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();
      axios.get(`${API_URL}/users/ages/count`, config).then(({ data }) => {
        setRanges(data);
      });
    })();
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
