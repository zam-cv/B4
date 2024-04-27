import Chart, { getData } from ".";
import { useEffect, useState } from "react";
import { Pie } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import api from "@/utils/api";

export default function UsersByGender() {
  const [money, setMoney] = useState<[string, number][]>([]);

  useEffect(() => {
    api.players.getAverageMoney().then((data) => {
      setMoney(data);
    })
  }, []);

  return (
    <Chart title="Dinero/crédito promedio de los jugadores">
      <Pie
        data={{
          labels: money.map(m => m[0]),
          datasets: [
            {
              data: getData(money.map(m => m[0]), money),
              backgroundColor: getColors(money.length),
            },
          ],
        }}
        options={CHART_DEFAULT_OPTIONS}
      />
    </Chart>
  );
}
