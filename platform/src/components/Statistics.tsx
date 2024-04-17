import { faker } from "@faker-js/faker";
import { getOptions } from "../utils/constants";
import GraphicView from "./GraphicView";

const labels = ["1", "2", "3", "4", "5", "6", "7"];
const options = getOptions();
options.plugins.title.text = "Statistics";
// options.scales.y.min = 80;
// options.scales.y.max = 100;
options.scales.y.ticks.stepSize = 5;
// options.scales.x.ticks.stepSize = 5;

const statistics = {
  labels,
  datasets: [
    {
      data: labels.map(() => faker.number.int({ min: 20, max: 100 })),
      borderColor: "rgb(0, 200, 255)",
    },
  ],
};

export default function Statistics() {
  return (
    <GraphicView options={options} values={statistics} />
  );
}
