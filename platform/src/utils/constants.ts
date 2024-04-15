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
  VIEW_ACCOUNTS: "ViewAccounts",
  ADD_ACCOUNTS: "AddAccounts",
  EDIT_ACCOUNTS: "EditAccounts",
  SEND_EMAILS: "SendEmails",
};

export const CHART_COLOR_LIST = [
  "#008B8B", // Dark Cyan
  "#4169E1", // Royal Blue
  "#32CD32", // Lime Green
  "#FF6347", // Tomato
  "#40E0D0", // Turquoise
  "#FF69B4", // Hot Pink
  "#FA8072", // Salmon
  "#2E8B57", // Sea Green
  "#4682B4", // Steel Blue
  "#FAFAD2", // Light Goldenrod Yellow
  "#DB7093", // Pale Violet Red
  "#DAA520", // Goldenrod
  "#FF1493", // Deep Pink
  "#1E90FF", // Dodger Blue
  "#007BFF", // Bright Blue
  "#50C878", // Emerald
  "#FFD700", // Gold
  "#DC143C", // Crimson
  "#9B30FF", // Vivid Purple
  "#00FA9A", // Medium Spring Green
  "#FF00FF", // Fuchsia
  "#FF4500", // Orange Red
  "#C0C0C0", // Silver
  "#FF007F", // Bright Pink
  "#FFD700", // Bright Yellow
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
