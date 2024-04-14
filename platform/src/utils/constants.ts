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
  VIEW_ACCOUNTS: "ViewAccounts",
  ADD_ACCOUNTS: "AddAccounts",
  EDIT_ACCOUNTS: "EditAccounts",
  SEND_EMAILS: "SendEmails",
};

export function getOptions() {
  return {
    responsive: true,
    maintainAspectRatio: false,
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
