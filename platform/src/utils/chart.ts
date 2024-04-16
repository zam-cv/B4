import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  ArcElement,
  BarElement
} from "chart.js";
import { CHART_COLOR_LIST } from "./constants";

export function initChart() {
  ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
    ArcElement,
    BarElement
  );

  ChartJS.defaults.elements.point.pointStyle = false;
}

export function getColors(length: number) {
  const colors = [];

  // The colors are repeated if the number of labels is greater than the number of colors
  for (let i = 0; i < length; i++) {
    colors.push(CHART_COLOR_LIST[i % CHART_COLOR_LIST.length]);
  }

  return colors;
}
