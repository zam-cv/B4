import { useState, useEffect } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";
import axios from "axios";
import { API_URL } from "@/utils/constants";
import { getConfig } from "../utils/auth";

export default function Docs() {
  const [spec, setSpec] = useState({});

  useEffect(() => {
    (async () => {
      axios
        .get(`${API_URL}/docs/swagger.json`, await getConfig())
        .then(({ data }) => {
          setSpec(data);
        })
        .catch((error) => {
          console.error(error);
        });
    })();
  }, []);

  return <SwaggerUI spec={spec} />;
}
