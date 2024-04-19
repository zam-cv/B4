import { useState, useEffect } from "react";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import axios from "axios";
import { API_URL } from "@/utils/constants";
import { getConfig } from "../utils/auth";
import MDEditor from "@uiw/react-md-editor";
import juice from "juice";
import { EMAIL_EXAMPLE } from "@/utils/constants";

interface Filters {
  by_age_range?: [number | null, number | null];
  by_user_type?: string;
  by_gender?: string;
  by_extension?: string;
}

export default function Emails() {
  const [typesUsers, setTypesUsers] = useState<string[]>([]);
  const [genders, setGender] = useState<string[]>([]);

  const [selectedUsers, setSelectedUsers] = useState<number>(0);

  const [title, setTitle] = useState<string>("¡Explora lo Nuevo en Qrops y Aprovecha Exclusivas Ventajas!");
  const [value, setValue] = useState<any>(EMAIL_EXAMPLE);

  const [filters] = useState<Filters>({});

  async function sendEmail() {
    const cssStyles = Array.from(document.querySelectorAll("style"))
      .map((style) => style.innerHTML)
      .join("\n");

    const htmlContent = document.getElementsByClassName(
      "w-md-editor-preview"
    )[0];

    const fullHtml = `${htmlContent.innerHTML}`;
    const withInlineStyles = juice.inlineContent(fullHtml, cssStyles);

    const parser = new DOMParser();
    const doc = parser.parseFromString(withInlineStyles, "text/html");

    const elements = doc.querySelectorAll("*");

    elements.forEach((element) => {
      element.removeAttribute("class");
    });

    const body = doc.documentElement.outerHTML;

    const config = await getConfig();

    if (title !== "" && value !== "") {
      axios
        .post(
          `${API_URL}/mail`,
          {
            title,
            body,
            filters,
          },
          config
        )
        .then(() => {
          setValue("");
        })
        .catch((error) => {
          console.error(error);
        });
    }
  }

  async function getSelectedUsers() {
    const config = await getConfig();
    let tmp: Filters = {};

    if (
      filters.by_age_range &&
      filters.by_age_range[0] &&
      filters.by_age_range[1] &&
      filters.by_age_range[0] < filters.by_age_range[1]
    ) {
      tmp.by_age_range = filters.by_age_range;
    }

    if (
      filters.by_user_type &&
      filters.by_user_type.length !== 0 &&
      filters.by_user_type !== "all"
    ) {
      tmp.by_user_type = filters.by_user_type;
    }

    if (
      filters.by_gender &&
      filters.by_gender.length !== 0 &&
      filters.by_gender !== "all"
    ) {
      tmp.by_gender = filters.by_gender;
    }

    if (filters.by_extension && filters.by_extension.length !== 0) {
      tmp.by_extension = filters.by_extension;
    }

    if (Object.keys(tmp).length !== 0) {
      axios
        .post(`${API_URL}/mail/count`, tmp, config)
        .then(({ data }) => {
          setSelectedUsers(data);
        })
        .catch((error) => {
          console.error(error);
        });
    }
  }

  function setValueInFilter(callback: (filters: Filters) => void) {
    callback(filters);
    getSelectedUsers();
  }

  useEffect(() => {
    (async () => {
      const config = await getConfig();

      axios
        .post(`${API_URL}/mail/count`, {}, config)
        .then(({ data }) => {
          setSelectedUsers(data);
        })
        .catch((error) => {
          console.error(error);
        });

      axios.get(`${API_URL}/users/genders`, config).then(({ data }) => {
        setGender(data);
      });

      axios.get(`${API_URL}/users/types`, config).then(({ data }) => {
        setTypesUsers(data);
      });
    })();
  }, []);

  return (
    <div className="h-full">
      <ResizablePanelGroup direction="vertical" className="rounded-lg border">
        <ResizablePanel defaultSize={65}>
          <div className="p-5 flex flex-col gap-5 w-full h-full">
            <h1 className="text-2xl font-bold">
              Redacción de correo electrónico
            </h1>
            <div className="flex justify-start">
              <div className="mr-auto w-[500px]">
                <Input
                  placeholder="Asunto"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                />
              </div>
            </div>
            <div className="relative w-full h-full overflow-auto">
              <div className="absolute w-full h-full p-1">
                <MDEditor value={value} onChange={setValue} height="100%" />
              </div>
            </div>
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={35}>
          <div className="p-5 flex flex-col gap-5">
            <h1 className="text-2xl font-bold">Filtros</h1>
            <div className="flex items-center gap-5">
              <h2 className="text-lg font-medium">
                Cantidad de usuarios seleccionados: {selectedUsers}
              </h2>
            </div>
            <div className="flex gap-5 flex-wrap">
              <div className="flex gap-5 items-center">
                <h2 className="text-lg font-semibold">Edad</h2>
                <div className="flex gap-5 mr-10">
                  <Input
                    type="number"
                    placeholder="Desde"
                    onChange={(e) =>
                      setValueInFilter(
                        (f) =>
                          (f.by_age_range = [
                            parseInt(e.target.value),
                            f.by_age_range ? f.by_age_range[1] : null,
                          ])
                      )
                    }
                  />
                  <Input
                    type="number"
                    placeholder="Hasta"
                    onChange={(e) =>
                      setValueInFilter(
                        (f) =>
                          (f.by_age_range = [
                            f.by_age_range ? f.by_age_range[0] : null,
                            parseInt(e.target.value),
                          ])
                      )
                    }
                  />
                </div>
              </div>
              <div className="flex gap-5 items-center">
                <h2 className="text-lg font-semibold">Tipo de usuario</h2>
                <div className="mr-10">
                  <Select
                    onValueChange={(v) => {
                      setValueInFilter((f) => (f.by_user_type = v));
                    }}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Selecciona un tipo de usuario" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectLabel>Usuarios</SelectLabel>
                        {typesUsers.map((type, i) => (
                          <SelectItem key={i} value={type}>
                            {type}
                          </SelectItem>
                        ))}
                        <SelectItem value="all">Todos</SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                </div>
              </div>
              <div className="flex gap-5 items-center">
                <h2 className="text-lg font-semibold">Género</h2>
                <div className="mr-10">
                  <Select
                    onValueChange={(v) => {
                      setValueInFilter((f) => (f.by_gender = v));
                    }}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Selecciona un genero" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectLabel>Generos</SelectLabel>
                        {genders.map((gender, i) => (
                          <SelectItem key={i} value={gender}>
                            {gender}
                          </SelectItem>
                        ))}
                        <SelectItem value="all">Todos</SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                </div>
              </div>
              <div className="flex gap-5 items-center">
                <h2 className="text-lg font-semibold">Extensión</h2>
                <div className="mr-10">
                  <Input
                    placeholder="@outlook.com"
                    onChange={(e) => {
                      setValueInFilter(
                        (f) => (f.by_extension = e.target.value)
                      );
                    }}
                  />
                </div>
              </div>
            </div>
            <div className="flex justify-end">
              <Button className="px-10" onClick={sendEmail}>
                Enviar
              </Button>
            </div>
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
