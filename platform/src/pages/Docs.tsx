import { useState, useEffect } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";
import api from "@/utils/api";

export default function Docs() {
  const [spec, setSpec] = useState({});

  useEffect(() => {
    api.docs.getApi().then((data) => {
      setSpec(data);
    });
  }, []);

  return <SwaggerUI spec={spec} />;
}
