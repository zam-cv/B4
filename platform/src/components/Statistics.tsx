import GraphicView from "./GraphicView";
import axios from "axios";
import { API_URL } from "../utils/constants";
import { getConfig } from "../utils/auth";
import { useEffect, useState } from "react";

interface Statistic {
  cycle: number;
  score: number;
}

export default function Statistics({ userId }: { userId: string | null }) {
  const [labels, setLabels] = useState<string[]>([]);
  const [data, setData] = useState<number[]>([]);

  useEffect(() => {
    (async () => {
      if (!userId) return;

      const config = await getConfig();
      axios
        .get(`${API_URL}/user/statistics/${userId}`, config)
        .then(({ data }: { data: Statistic[] }) => {
          setData(data.map((d) => parseFloat(d.score.toFixed(2))));
          setLabels(data.map((d) => d.cycle.toString()));
        });
    })();
  }, [userId]);

  return (
    <div className="w-full h-full flex flex-col">
      <div>
        <h2 className="text-center text-xl font-bold text-gray-800">
          Rendimiento del jugador
        </h2>
      </div>
      <div className="relative w-full h-full">
        <div className="absolute h-full w-full">
          <GraphicView
            options={{
              responsive: true,
              maintainAspectRatio: false,
              plugins: {
                legend: {
                  display: false,
                },
              },
            }}
            values={{
              labels,
              datasets: [
                {
                  data,
                  borderColor: "rgb(0, 200, 255)",
                },
              ],
            }}
          />
        </div>
      </div>
    </div>
  );
}
