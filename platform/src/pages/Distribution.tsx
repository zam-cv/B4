import { useEffect, useState } from "react";
import { getColors } from "../utils/chart";
import { API_URL } from "../utils/constants";
import { getConfig } from "../utils/auth";
import axios from "axios";
import {
  ComposableMap,
  Geographies,
  Geography,
  ZoomableGroup,
  Marker,
} from "react-simple-maps";

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
  const data = getData(typesUsers, locations);

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      axios.get(`${API_URL}/users/types`, config).then(({ data }) => {
        setTypesUsers(data);
      });

      axios.get(`${API_URL}/users/locations/types`, config).then(({ data }) => {
        setLocations(data);
      });
    })();
  }, []);

  return (
    <div className="grid grid-cols-4 grid-rows-4 gap-5 h-full">
      <div className="bg-gray-300 row-span-3"></div>
      <div className="col-span-3 row-span-4 w-full h-full overflow-hidden">
        <div className="grid grid-rows-[1fr_auto] grid-cols-1 w-full h-full">
          <div className="w-full h-full relative flex justify-center items-center overflow-hidden">
            <div className="w-full h-full absolute flex justify-center items-center">
              <ComposableMap>
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
                  {data.data.map((coordinates) =>
                    coordinates.map((coordinate, index) => (
                      <Marker key={index} coordinates={coordinate}>
                        <circle r={3} fill={data.colors[index]} />
                      </Marker>
                    ))
                  )}
                </ZoomableGroup>
              </ComposableMap>
            </div>
          </div>
          <div className="col-span-3">
            <div className="flex flex-col gap-5 p-5">
              <h1 className="text-xl font-bold text-center">
                Tipos de usuarios
              </h1>
              <div className="flex flex-wrap justify-center">
                {data.labels.map((label, index) => (
                  <div key={index} className="flex items-center gap-2 py-2 px-7">
                    <div
                      className="w-4 h-4 rounded-full"
                      style={{ backgroundColor: data.colors[index] }}
                    ></div>
                    <div>{label}</div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="bg-gray-300"></div>
    </div>
  );
}
