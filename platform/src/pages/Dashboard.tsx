import UsersByType from "@/components/charts/UsersByType";
import UsersByGender from "@/components/charts/UsersByGender";
import UsersByAgeRange from "@/components/charts/UsersByAgeRange";
import Details from "@/components/Details";
import AverageSessions from "@/components/charts/AverageSessions";
import AverageTimeInGame from "@/components/charts/AverageTimeInGame";
import TopPlayers from "@/components/TopPlayers";
import AverageMoney from "@/components/charts/AverageMoney";

export default function Dashboard() {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-1 sm:grid-flow-row sm::auto-rows-[400px] md:grid-cols-7 md:grid-flow-row md:auto-rows-[400px] h-full lg:grid-cols-10 lg:grid-rows-2">
      <div className="lg:col-span-3 md:col-span-3">
        <UsersByType />
      </div>
      <div className="lg:col-span-3 md:col-span-4">
        <UsersByAgeRange />
      </div>
      <div className="lg:col-span-2 md:col-span-2">
        <UsersByGender />
      </div>
      <div className="lg:col-span-2 md:col-span-2">
        <TopPlayers />
      </div>

      <div className="lg:col-span-2 md:col-span-3">
        <Details />
      </div>
      <div className="lg:col-span-2 md:col-span-3">
        <AverageMoney />
      </div>
      <div className="lg:col-span-3 md:col-span-4">
        <AverageSessions />
      </div>
      <div className="lg:col-span-3 md:col-span-7">
        <AverageTimeInGame />
      </div>
    </div>
  );
}
