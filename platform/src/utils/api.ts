import { get, post, put, del } from "./methods";

export interface Admin {
  id: string;
  email: string;
  role_id: string;
}

export interface Crop {
  name: string;
  price: number;
  duration: number;
  description: string;
}

export interface Event {
  event_type: "Positive" | "Negative";
  content: string;
}

export interface Statistic {
  cycle: number;
  score: number;
}

export interface Player {
  current_cycle: number;
  current_score: number;
  balance_cash: number;
  balance_verqor: number;
  balance_coyote: number;
}

export interface Tip {
  id: number;
  content: string;
}

export interface User {
  id: string;
  username: string;
  user_type: string;
  email: string;
  gender: string;
  age: number;
  os: string;
  latitude: number;
  longitude: number;
}

export interface Info {
  token: string;
  admin: Admin;
}

export interface Filters {
  by_age_range?: [number | null, number | null];
  by_user_type?: string;
  by_gender?: string;
  by_extension?: string;
}

export default {
  auth: {
    signout: (): Promise<void> => del("/auth/signout"),
    authenticate: (): Promise<Admin> => get("/auth"),
    signin: (email: string, password: string): Promise<Info> => {
      return post("/auth/signin", { email, password });
    },
    registerAdmin: (email: string, password: string): Promise<Admin> => {
      return post("/auth/register", { email, password });
    },
  },
  permissions: {
    getPermissionsTypes: (roleId: string): Promise<string[]> => {
      return get(`/permissions/types/${roleId}`);
    },
    getUserPermissions: (id: string): Promise<string[]> => {
      return get(`/permissions/${id}`);
    },
    setPermission: (id: string, permission: string): Promise<void> => {
      return post(`/permissions`, { id, permission });
    },
    deletePermission: (id: string, permission: string): Promise<void> => {
      return del(`/permissions/${id}/${permission}`);
    },
    getPermissions: (): Promise<string[]> => get("/permissions"),
  },
  admins: {
    getAdmins: (): Promise<Admin[]> => get("/admins"),
    deleteAdmin: (id: string): Promise<void> => del(`/admins/${id}`),
  },
  data: {
    getCrops: (): Promise<Crop[]> => get("/data/crops"),
    getEvents: (): Promise<Event[]> => get("/data/events"),
    getTips: (): Promise<Tip[]> => get("/data/tips"),
    updateCropPrice: (name: string, price: number): Promise<void> => {
      return put(`/data/crops/${name}/price`, price);
    },
    updateCropDuration: (name: string, duration: number): Promise<void> => {
      return put(`/data/crops/${name}/duration`, duration);
    },
    updateCropDescription: (
      name: string,
      description: string
    ): Promise<void> => {
      return put(`/data/crops/${name}/description`, description);
    },
    setTip: (content: string): Promise<string> => {
      return post("/data/tips", { content });
    },
    updateTip: (id: number, content: string): Promise<void> => {
      return put(`/data/tips/${id}`, { content });
    },
    deleteTip: (id: number): Promise<void> => del(`/data/tips/${id}`),
  },
  players: {
    getAverageTimeInGame: (): Promise<number> => get("/players/average-time"),
    getTopPlayers: (): Promise<string[]> => get("/players/top-players"),
    getCountPlayers: (): Promise<number> => get("/players/count"),
    getAverageMoney: (): Promise<[string, number][]> => get("/players/average-money"),
    getAverageScore: (): Promise<number> => get("/players/average-score"),
  },
  player: {
    getPlayer: (id: string): Promise<Player> => get(`/player/${id}`),
    getHistory: (id: string): Promise<[Statistic, [string, string][]][]> => {
      return get(`/player/${id}/history`);
    },
  },
  users: {
    getUsersTypes: (): Promise<string[]> => get("/users/types"),
    getGenders: (): Promise<string[]> => get("/users/genders"),
    getUsersByType: (): Promise<string[]> => get("/users/types"),
    getAverageAge: (): Promise<number> => get("/users/average-age"),
    getUsers: (): Promise<User[]> => get("/users"),
    getAverageSessions: (): Promise<[number, number][]> => {
      return get("/users/average-sessions");
    },
    getAverageTimeInGame: (): Promise<[string, number][]> => {
      return get("/users/average-time-in-game");
    },
    getUsersByAgeRange: (): Promise<[string, number][]> => {
      return get("/users/ages/count");
    },
    getCountUsersByGender: (): Promise<[string, number][]> => {
      return get("/users/genders/count");
    },
    getCountUsersByType: (): Promise<[string, number][]> => {
      return get("/users/types/count");
    },
    getUsersLocations: (): Promise<[string, [number, number][]][]> => {
      return get(`/users/locations/types`);
    },
  },
  user: {
    getStatistics: (id: string): Promise<Statistic[]> => {
      return get(`/user/statistics/${id}`);
    },
  },
  docs: {
    getApi: (): Promise<any> => get("/docs/swagger.json"),
  },
  mail: {
    sendMail: (
      title: string,
      body: string,
      filters: Filters
    ): Promise<void> => {
      return post("/mail", { title, body, filters });
    },
    countMails: (filters: Filters): Promise<number> => {
      return post("/mail/count", filters);
    },
  },
};
