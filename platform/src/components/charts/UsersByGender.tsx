import Chart, { getData } from ".";
import { useEffect, useState } from "react";
import { Pie } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import { API_URL } from "../../utils/constants";
import { getConfig } from "../../utils/auth";
import axios from "axios";

export default function UsersByGender() {
  const [genders, setGender] = useState<string[]>([]);
  const [users, setUsers] = useState<[string, number][]>([]);

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      axios.get(`${API_URL}/users/genders`, config).then(({ data }) => {
        setGender(data);
      });

      axios.get(`${API_URL}/users/genders/count`, config).then(({ data }) => {
        setUsers(data);
      });
    })();
  }, []);

  return (
    <Chart title="Cantidad de usuarios por género">
      <Pie
        data={{
          labels: genders,
          datasets: [
            {
              data: getData(genders, users),
              backgroundColor: getColors(genders.length),
            },
          ],
        }}
        options={CHART_DEFAULT_OPTIONS}
      />
    </Chart>
  );
}
