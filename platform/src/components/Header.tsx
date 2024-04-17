import { Link } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import { pagePermissions } from "../App";

export function Sub({ title, route }: { title: string; route: string }) {
  return (
    <Link to={route}>
      <div className="p-4 px-4 font-bold hover:underline cursor-pointer select-none">
        {title}
      </div>
    </Link>
  );
}

export default function Header() {
  const { signout, admin, permissions } = useAuth();

  return (
    <div className="grid grid-cols-[1fr_auto] p-1 shadow-lg px-5">
      <div className="flex">
        <div className="p-4 px-4 font-bold hover:underline cursor-pointer">
          Verqor
        </div>
        {pagePermissions.map((page) => {
          if (permissions?.has(page.permission)) {
            return <Sub key={page.title} title={page.title} route={page.route} />;
          }
          return null;
        })}
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
