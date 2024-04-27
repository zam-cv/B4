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
    <div className="grid grid-cols-10 grid-rows-2 gap-5 h-full">
      <div className="col-span-3">
        <UsersByType />
      </div>
      <div className="col-span-3">
        <UsersByAgeRange />
      </div>
      <div className="col-span-2">
        <UsersByGender />
      </div>
      <div className="col-span-2">
        <TopPlayers />
      </div>

      <div className="col-span-2">
        <Details />
      </div>
      <div className="col-span-2">
        <AverageMoney />
      </div>
      <div className="col-span-3">
        <AverageSessions />
      </div>
      <div className="col-span-3">
        <AverageTimeInGame />
      </div>
    </div>
  );
}
