import { useEffect, useState } from "react";
import { getColors } from "../utils/chart";
import {
  ComposableMap,
  Geographies,
  Geography,
  ZoomableGroup,
  Marker,
} from "react-simple-maps";
import api from "@/utils/api";

interface Response {
  colors: string[];
  labels: string[];
  data: [number, number][][];
}

function getData(
  labels: string[],
  data: [string, [number, number][]][]
): Response {
  const colors = getColors(labels.length);
  const map = new Map<string, [number, number][]>();

  for (const [label, values] of data) {
    map.set(label, values);
  }

  const newData = labels.map((label) => map.get(label) ?? []);
  return { colors, labels, data: newData };
}

export default function Distribution() {
  const [typesUsers, setTypesUsers] = useState<string[]>([]);
  const [locations, setLocations] = useState<[string, [number, number][]][]>(
    []
  );
  const [visibility, setVisibility] = useState<boolean[]>(
    locations.map(() => true)
  );
  const data = getData(typesUsers, locations);

  useEffect(() => {
    api.users.getUsersTypes().then((data) => {
      setTypesUsers(data);
      setVisibility(data.map(() => true));
    });

    api.users.getUsersLocations().then((data) => {
      setLocations(data);
    });
  }, []);

  function handleVisibility(index: number) {
    let newVisibility = [...visibility];
    newVisibility[index] = !newVisibility[index];
    setVisibility(newVisibility);
  }

  return (
    <div className="w-full h-full">
      <div className="col-span-3 row-span-4 w-full h-full overflow-hidden">
        <div className="grid grid-rows-[1fr_auto] grid-cols-1 w-full h-full">
          <div className="w-full h-full relative flex justify-center items-center">
            <div className="w-full h-full absolute flex justify-center items-center">
              <ComposableMap className="w-full h-full">
                <ZoomableGroup center={[0, 0]} zoom={1}>
                  <Geographies geography="/features.json">
                    {({ geographies }) =>
                      geographies.map((geo) => (
                        <Geography
                          key={geo.rsmKey}
                          fill="#214770"
                          geography={geo}
                        />
                      ))
                    }
                  </Geographies>
                  {data.data.map((coordinates, index) =>
                    visibility[index]
                      ? coordinates.map((coordinate, i) => (
                          <Marker key={i} coordinates={coordinate}>
                            <circle r={2} fill={data.colors[index]} />
                          </Marker>
                        ))
                      : null
                  )}
                </ZoomableGroup>
              </ComposableMap>
            </div>
          </div>
          <div className="col-span-3">
            <div className="flex flex-col gap-5 p-5">
              <h1 className="text-xl font-bold text-center text-blue-950">
                Tipos de usuarios
              </h1>
              <div className="flex flex-wrap justify-center">
                {data.labels.map((label, index) => (
                  <div key={index} className="py-2 px-7">
                    <span
                      className="flex items-center gap-2 cursor-pointer select-none"
                      style={
                        visibility[index]
                          ? { opacity: 1 }
                          : { opacity: 0.5, textDecoration: "line-through" }
                      }
                      onClick={() => handleVisibility(index)}
                    >
                      <div
                        className="w-4 h-4 rounded-full"
                        style={{ backgroundColor: data.colors[index] }}
                      ></div>
                      <div>{label}</div>
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
