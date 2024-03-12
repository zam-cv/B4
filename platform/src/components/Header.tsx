import { useAuth } from "../hooks/useAuth";

export default function Header() {
  const { signout, admin } = useAuth();

  return (
    <div className="grid grid-cols-[1fr_auto] p-1 shadow-lg px-5">
      <div className="flex">
        <div className="p-4 px-4 font-bold">Verqor</div>
        <div className="p-4 px-4 font-bold">Dashboard</div>
      </div>
      <div className="flex items-center">
        <span
          onClick={signout}
          className="p-4 px-4 font-bold hover:underline cursor-pointer"
        >
          Cerrar Sesión
        </span>
        <div>
          <span className="p-2 px-3 font-bold uppercase rounded-full border-2 border-gray-600">
            {admin?.email.charAt(0)}
          </span>
        </div>
      </div>
    </div>
  );
}