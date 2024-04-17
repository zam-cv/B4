import { Line, Bubble } from "react-chartjs-2";

export default function GraphicView({
  options,
  values,
}: {
  options?: any;
  values: {
    labels?: any[];
    datasets: {
      data: number[];
      borderColor: string;
    }[];
  };
}) {
  return (
    <div className="h-full">
      <Line data={values} options={options} height={"50%"} />
    </div>
  );
}

export function GraphicViewBubble({
  options,
  values,
}: {
  options?: any;
  values: {
    labels?: any[];
    datasets: {
      data: number[];
      borderColor: string;
    }[];
  };
}) {
  return (
    <div className="h-full">
      <Bubble data={values} options={options} height={"50%"} />
    </div>
  );
}
