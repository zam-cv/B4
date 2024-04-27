import Chart, { getData } from ".";
import { useEffect, useState } from "react";
import { Pie } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import api from "@/utils/api";

export default function UsersByGender() {
  const [genders, setGender] = useState<string[]>([]);
  const [users, setUsers] = useState<[string, number][]>([]);

  useEffect(() => {
    api.users.getGenders().then((data) => {
      setGender(data);
    });

    api.users.getCountUsersByGender().then((data) => {
      setUsers(data);
    });
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
