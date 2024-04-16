const SERVER_HOST = import.meta.env.VITE_APP_SERVER_HOST || "localhost";
const SERVER_PORT = import.meta.env.VITE_APP_SERVER_PORT || "8080";
const API_ROUTE = import.meta.env.VITE_APP_API_ROUTE || "/api/admin";
export const API_URL = `http://${SERVER_HOST}:${SERVER_PORT}${API_ROUTE}`;
export const SERVER_URL = `http://${SERVER_HOST}:${SERVER_PORT}`;
export const SOCKET_URL = `ws://${SERVER_HOST}:${SERVER_PORT}/viewer/`;

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
  '#FF6384',  // rosy pink
  '#36A2EB',  // bright blue
  '#FFCE56',  // sunny yellow
  '#4BC0C0',  // sea green
  '#9966FF',  // amethyst purple
  '#FF9F40',  // vivid orange
  '#C9CB3F',  // lime green
  '#50AF95',  // teal
  '#703FEB',  // deep violet
  '#3B1F2B',  // dark maroon
  '#DAB5D7',  // soft lavender
  '#77D970',  // fresh green
  '#304D6D',  // midnight blue
  '#F2C14E',  // goldenrod
  '#8C5E58',  // muted brown
  '#3DCCC7',   // light turquoise
  '#F45B69',  // salmon pink
];

export const CHART_DEFAULT_OPTIONS = {
  responsive: true,
  maintainAspectRatio: false,
}

export function getOptions() {
  return {
    ...CHART_DEFAULT_OPTIONS,
    scales: {
      x: {
        ticks: {
          color: "white",
          stepSize: 110,
        },
        grid: {
          display: false,
        },
        display: false,
      },
      y: {
        ticks: {
          stepSize: 110,
        },
        grid: {
          display: false,
        },
      },
    },
    plugins: {
      legend: {
        display: false,
      },
      title: {
        display: true,
        text: "",
        color: "white",
      },
    },
    animation: {
      duration: 200,
    },
  };
}
