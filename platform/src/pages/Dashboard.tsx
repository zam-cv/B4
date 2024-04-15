import Users from "../components/charts/Users";

export default function Dashboard() {
  return (
    <div className="grid grid-cols-3 grid-rows-2 gap-5 h-full">
      <Users />
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
    </div>
  );
}
