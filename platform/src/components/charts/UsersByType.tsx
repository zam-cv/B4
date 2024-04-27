import Chart, { getData } from ".";
import { useEffect, useState } from "react";
import { Pie } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import api from "@/utils/api";

export default function UsersByType() {
  const [typesUsers, setTypesUsers] = useState<string[]>([]);
  const [users, setUsers] = useState<[string, number][]>([]);

  useEffect(() => {
    api.users.getUsersTypes().then((data) => {
      setTypesUsers(data);
    });

    api.users.getCountUsersByType().then((data) => {
      setUsers(data);
    });
  }, []);

  return (
    <Chart title="Cantidad de usuarios por tipo">
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
