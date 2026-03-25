import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from './api';
import type { EmpireResponse, PlanetResources, BuildingStatus, GameData } from './types';

export function useEmpire(universeId: string | null) {
  return useQuery<EmpireResponse>({
    queryKey: ['empire', universeId],
    queryFn: () => api.get(`/api/universes/${universeId}/empire`).then(r => r.data),
    enabled: !!universeId,
    staleTime: 30_000,
  });
}

export function useResources(planetId: string | null) {
  return useQuery<PlanetResources>({
    queryKey: ['resources', planetId],
    queryFn: () => api.get(`/api/planets/${planetId}/resources`).then(r => r.data),
    enabled: !!planetId,
    refetchInterval: 60_000,
    staleTime: 30_000,
  });
}

export function useBuildings(planetId: string | null) {
  return useQuery<BuildingStatus[]>({
    queryKey: ['buildings', planetId],
    queryFn: () => api.get(`/api/planets/${planetId}/buildings`).then(r => r.data),
    enabled: !!planetId,
    staleTime: 10_000,
  });
}

export function useGameData() {
  return useQuery<GameData>({
    queryKey: ['gameData'],
    queryFn: () => api.get('/api/game-data').then(r => r.data),
    staleTime: Infinity,
  });
}

export function useStartBuild(planetId: string | null) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (buildingId: string) =>
      api.post(`/api/planets/${planetId}/buildings`, { building_id: buildingId }).then(r => r.data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['buildings', planetId] });
      qc.invalidateQueries({ queryKey: ['resources', planetId] });
    },
  });
}

export function useCancelBuild(planetId: string | null) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: () =>
      api.delete(`/api/planets/${planetId}/buildings`).then(r => r.data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['buildings', planetId] });
    },
  });
}
