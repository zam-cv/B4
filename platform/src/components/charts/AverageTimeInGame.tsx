import Chart, { getData } from ".";
import { useEffect, useState } from "react";
import { Pie } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import { API_URL } from "../../utils/constants";
import { getConfig } from "../../utils/auth";
import axios from "axios";

export default function AverageTimeInGame() {
  const [typesUsers, setTypesUsers] = useState<string[]>([]);
  const [users, setUsers] = useState<[string, number][]>([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      axios.get(`${API_URL}/users/types`, config).then(({ data }) => {
        setTypesUsers(data);
      });

      axios.get(`${API_URL}/users/average-time-in-game`, config).then(({ data }) => {
        setUsers(data);
      });
    })();
  }, []);

  return (
    <Chart title="Tiempo promedio en el juego por tipo de usuario">
      <Pie
        data={{
          labels: typesUsers,
          datasets: [
            {
              data: getData(typesUsers, users),
              backgroundColor: getColors(typesUsers.length),
            },
          ],
        }}
        options={CHART_DEFAULT_OPTIONS}
      />
    </Chart>
  );
}
