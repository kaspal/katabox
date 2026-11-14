// The part of d3-hierarchy 3.1.2 the Learn tree layout uses (the package ships no types,
// and @types/d3-hierarchy would be another dependency).
declare module "d3-hierarchy" {
  export interface HierarchyNode<T> {
    data: T;
    depth: number;
    x: number;
    y: number;
    parent: HierarchyNode<T> | null;
    children?: HierarchyNode<T>[];
    descendants(): HierarchyNode<T>[];
  }
  export function hierarchy<T>(data: T, children?: (d: T) => T[] | null | undefined): HierarchyNode<T>;
  export interface TreeLayout<T> {
    (root: HierarchyNode<T>): HierarchyNode<T>;
    nodeSize(size: [number, number]): TreeLayout<T>;
    separation(f: (a: HierarchyNode<T>, b: HierarchyNode<T>) => number): TreeLayout<T>;
  }
  export function tree<T>(): TreeLayout<T>;
}
