const SERVER_PROTOCOL = import.meta.env.VITE_APP_SERVER_PROTOCOL || "http";
const SERVER_PORT = import.meta.env.VITE_APP_SERVER_PORT || "8080";
const SERVER_HOSTNAME = import.meta.env.VITE_APP_SERVER_HOST || "localhost";
let SERVER_HOST = `${SERVER_HOSTNAME}:${SERVER_PORT}`;

// if the domain in which it is connecting is different from the one that is being given
// replace with the current domain
SERVER_HOST =
  window.location.hostname === SERVER_HOSTNAME
    ? SERVER_HOST
    : window.location.hostname + ":" + SERVER_PORT;

const API_ROUTE = import.meta.env.VITE_APP_API_ROUTE || "api/admin";
export const API_URL = `${SERVER_PROTOCOL}://${SERVER_HOST}/${API_ROUTE}`;
export const SOCKET_URL = `ws://${SERVER_HOST}/viewer/`;

export const ADMIN_PERMISSIONS = {
  VIEW_DOCUMENTS: "ViewDocuments",
  VIEW_DASHBOARD: "ViewDashboard",
  VIEW_DISTRIBUTION: "ViewDistribution",
  VIEW_USERS: "ViewUsers",
  VIEW_EDITION: "ViewEdition",
  ADD_ACCOUNTS: "AddAccounts",
  EDIT_ACCOUNTS: "EditAccounts",
  SEND_EMAILS: "SendEmails",
};

export const CHART_COLOR_LIST = [
  "#FF6384", // rosy pink
  "#36A2EB", // bright blue
  "#FFCE56", // sunny yellow
  "#4BC0C0", // sea green
  "#9966FF", // amethyst purple
  "#FF9F40", // vivid orange
  "#C9CB3F", // lime green
  "#50AF95", // teal
  "#703FEB", // deep violet
  "#3B1F2B", // dark maroon
  "#DAB5D7", // soft lavender
  "#77D970", // fresh green
  "#304D6D", // midnight blue
  "#F2C14E", // goldenrod
  "#8C5E58", // muted brown
  "#3DCCC7", // light turquoise
  "#F45B69", // salmon pink
];

export const CHART_DEFAULT_OPTIONS = {
  responsive: true,
  maintainAspectRatio: false,
};

export function getOptions() {
  return {
    ...CHART_DEFAULT_OPTIONS,
  };
}

export const EMAIL_EXAMPLE = `
# Hola!!!,

En Verqor, estamos comprometidos con impulsar la producción agrícola de manera efectiva y sostenible. Como parte de nuestra comunidad, queremos recordarte todos los beneficios que puedes aprovechar al trabajar con nosotros. **Maximiza tus resultados** utilizando los recursos y servicios que ofrecemos específicamente diseñados para tus necesidades agrícolas.

### 🚜 **Financiamiento Personalizado y Seguro**

Ofrecemos créditos remotos con tasas ajustadas a tus necesidades, permitiéndote acceder a insumos de calidad y estructuras agrícolas avanzadas como macrotúneles y sistemas de riego. Nuestro proceso es ágil y confiable, lo que significa que puedes empezar a mejorar tu producción más rápidamente y con total seguridad.

### 🌾 **Entrega Directa de Insumos**

No solo facilitamos el financiamiento, sino que también entregamos los insumos directamente en tu parcela. Trabajamos con las mejores marcas del mercado, asegurando que solo recibas productos de alta calidad y con las certificaciones necesarias.

### 🤝 **Conexión con la Industria Agro**

Extendemos nuestro apoyo más allá del financiamiento, conectándote con una amplia red de empresas dentro de la industria agro. Este acceso te permite explorar nuevas oportunidades de negocio y expandir tu mercado.

### 💼 **Programa B2B para Empresas Agrícolas**

Si eres parte de una empresa agrícola, nuestro programa B2B está diseñado para apoyarte con soluciones específicas que te ayudarán a crecer y prosperar en un mercado competitivo.

### 🛡️ **Protección de Datos Garantizada**

Tu seguridad es nuestra prioridad. Todos los procesos se realizan de manera remota y están protegidos con los protocolos y herramientas necesarias para asegurar que tus datos personales y financieros estén completamente seguros.

### 🌍 **Conexiones para Vender tu Producción**

¿Listo para llevar tu producción al siguiente nivel? Te conectamos con una red extensa de compradores y distribuidores agrícolas, facilitando que tus productos lleguen más lejos y a más clientes.

Queremos que veas a Verqor no solo como un proveedor, sino como un socio en tu camino hacia una producción agrícola más eficiente y rentable. Descubre más y comienza a aprovechar todos estos beneficios hoy mismo. Visita nuestro sitio web: [https://verqor.com/](https://verqor.com/).

Un saludo cordial,

Equipo de Verqor
`.trim();
