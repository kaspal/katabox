// The part of d3-force 3.0.0 the Learn graph layout uses (the package ships no types, and
// @types/d3-force would be another dependency).
declare module "d3-force" {
  export interface SimulationNodeDatum {
    index?: number;
    x?: number;
    y?: number;
    vx?: number;
    vy?: number;
    fx?: number | null;
    fy?: number | null;
  }
  export interface Force<N> {
    (alpha: number): void;
    initialize?(nodes: N[], random: () => number): void;
  }
  export interface Simulation<N> {
    force(name: string, force: Force<N> | null): this;
    randomSource(source: () => number): this;
    tick(iterations?: number): this;
    stop(): this;
    nodes(): N[];
  }
  export interface LinkForce<N, L> extends Force<N> {
    id(id: (node: N) => string | number): this;
    distance(distance: number): this;
    strength(strength: number): this;
    links(): L[];
  }
  export interface StrengthForce<N> extends Force<N> {
    strength(strength: number): this;
  }
  export function forceSimulation<N extends SimulationNodeDatum>(nodes?: N[]): Simulation<N>;
  export function forceLink<N extends SimulationNodeDatum, L>(links?: L[]): LinkForce<N, L>;
  export function forceManyBody<N extends SimulationNodeDatum>(): StrengthForce<N>;
  export function forceCollide<N extends SimulationNodeDatum>(radius?: number): StrengthForce<N>;
  export function forceX<N extends SimulationNodeDatum>(x?: number): StrengthForce<N>;
  export function forceY<N extends SimulationNodeDatum>(y?: number): StrengthForce<N>;
}
