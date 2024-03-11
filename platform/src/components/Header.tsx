import { useAuth } from "../hooks/useAuth";

export default function Header() {
  const { signout } = useAuth();

  return (
    <div className="grid grid-cols-[1fr_auto] shadow-lg px-5">
      <div className="flex">
        <div className="p-4 px-4 font-bold">Verqor</div>
        <div className="p-4 px-4 font-bold">Dashboard</div>
      </div>
      <div className="flex">
        <span
          onClick={signout}
          className="p-4 px-4 font-bold hover:underline cursor-pointer"
        >
          Cerrar Sesión
        </span>
      </div>
    </div>
  );
}
