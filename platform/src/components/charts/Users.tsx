import Chart from ".";
import { useEffect, useState } from "react";
import { Pie } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import { API_URL } from "../../utils/constants";
import { getConfig } from "../../utils/auth";
import axios from "axios";

function getData(typesUsers: string[], users: [string, number][]): number[] {
  const map = new Map<string, number>();

  for (const [type, count] of users) {
    map.set(type, count);
  }

  return typesUsers.map((type) => map.get(type) ?? 0);
}

export default function Users() {
  const [typesUsers, setTypesUsers] = useState<string[]>([]);
  const [users, setUsers] = useState<[string, number][]>([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      axios.get(`${API_URL}/users/types`, config).then(({ data }) => {
        setTypesUsers(data);
      });

      axios.get(`${API_URL}/users/types/count`, config).then(({ data }) => {
        setUsers(data);
      });
    })();
  }, []);

  return (
    <Chart title="Cantidad de usuarios">
      <Pie
        data={{
          labels: typesUsers,
          datasets: [
            {
              data: getData(typesUsers, users),
              backgroundColor: getColors(3),
            },
          ],
        }}
        options={CHART_DEFAULT_OPTIONS}
      />
    </Chart>
  );
}
