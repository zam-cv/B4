import UsersByType from "../components/charts/UsersByType";
import UsersByGender from "@/components/charts/UsersByGender";

export default function Dashboard() {
  return (
    <div className="grid grid-cols-3 grid-rows-2 gap-5 h-full">
      <UsersByType />
      <UsersByGender />
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
    </div>
  );
}
