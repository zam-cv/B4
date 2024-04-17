import UsersByType from "@/components/charts/UsersByType";
import UsersByGender from "@/components/charts/UsersByGender";
import UsersByAgeRange from "@/components/charts/UsersByAgeRange";
import Details from "@/components/Details";
import AverageSessions from "@/components/charts/AverageSessions";
import AverageTimeInGame from "@/components/charts/AverageTimeInGame";

export default function Dashboard() {
  return (
    <div className="grid grid-cols-3 grid-rows-2 gap-5 h-full">
      <UsersByType />
      <UsersByAgeRange />
      <UsersByGender />
      <Details />
      <AverageSessions />
      <AverageTimeInGame />
    </div>
  );
}
