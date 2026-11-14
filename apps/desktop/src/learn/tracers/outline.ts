// The decision outline the backtracking tracers record (ADR 0029, Phase 9): one line per
// call, indented by depth; a line's role changes as the search learns more about it.
import type { DecisionData, Role } from "../engine/types";

export class Outline {
  private readonly lines: { depth: number; text: string; role?: Role }[] = [];
  current: number | null = null;

  /** A new call: returns its line number. */
  open(depth: number, text: string): number {
    this.lines.push({ depth, text, role: "key" });
    if (this.current !== null && this.lines[this.current]!.role === "key") this.lines[this.current]!.role = undefined;
    this.current = this.lines.length - 1;
    return this.current;
  }

  /** Mark a line (a solution, a dead end) and make it current. */
  mark(line: number, role: Role | undefined) {
    if (this.current !== null && this.current !== line && this.lines[this.current]!.role === "key") this.lines[this.current]!.role = undefined;
    this.lines[line]!.role = role;
    this.current = line;
  }

  /** Back in a call (after a child returned): it becomes current again. */
  back(line: number) {
    if (this.current !== null && this.lines[this.current]!.role === "key") this.lines[this.current]!.role = undefined;
    if (this.lines[line]!.role === undefined) this.lines[line]!.role = "key";
    this.current = line;
  }

  data(): DecisionData {
    return { lines: this.lines.map((l) => ({ ...l })), current: this.current };
  }
}
