export interface User {
  id: string;
  username: string;
  email: string;
  universe_id?: string;
}

export interface Planet {
  id: string;
  name: string;
  galaxy: number;
  solar_system: number;
  position: number;
  diameter?: number;
  temperature?: number;
  fields_used?: number;
  fields_total?: number;
  empire_id?: string;
}

export interface Empire {
  id: string;
  name: string;
  user_id: string;
  universe_id: string;
  technologies?: Record<string, number>;
  created_at?: string;
}

export interface ResourceCost {
  metal: number;
  crystal: number;
  deuterium: number;
  energy?: number;
}

export interface Building {
  id: string;
  name: string;
  category: string;
  description: string;
  base_cost: ResourceCost;
  cost_factor: number;
  base_production?: ResourceCost;
}

export interface BuildingStatus {
  building_id: string;
  name: string;
  category: string;
  description?: string;
  level: number;
  is_upgrading: boolean;
  upgrade_finish_at?: string;
  upgrade_cost?: ResourceCost;
}

export interface PlanetResources {
  metal: number;
  crystal: number;
  deuterium: number;
  energy: number;
  metal_production: number;
  crystal_production: number;
  deuterium_production: number;
  energy_production: number;
  last_updated?: string;
}

export interface Technology {
  id: string;
  name: string;
  category: string;
  description: string;
  base_cost: ResourceCost;
  cost_factor: number;
}

export interface Ship {
  id: string;
  name: string;
  category: string;
  description: string;
  base_cost: ResourceCost;
  attack: number;
  defense: number;
  speed: number;
  cargo: number;
}

export interface GameData {
  buildings: Building[];
  technologies: Technology[];
  ships: Ship[];
}

export interface EmpireResponse {
  empire: Empire;
  planets: Planet[];
}
