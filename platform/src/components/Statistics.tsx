import GraphicView from "./GraphicView";
import { useEffect, useState } from "react";
import api from "@/utils/api";

export default function Statistics({ userId }: { userId: string | null }) {
  const [labels, setLabels] = useState<string[]>([]);
  const [data, setData] = useState<number[]>([]);

  useEffect(() => {
    if (!userId) return;
    api.user.getStatistics(userId).then((data) => {
      setData(data.map((d) => parseFloat(d.score.toFixed(2))));
      setLabels(data.map((d) => d.cycle.toString()));
    })
  }, [userId]);

  return (
    <div className="w-full h-full flex flex-col">
      <div>
        <h2 className="text-center text-xl font-bold text-blue-950">
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
