// Learn data for UI tests (ADR 0029): hand-written subjects and topic metadata, with the
// real samples, theory and quizzes read from content/learn (Vitest runs in apps/desktop).
import { readFileSync } from "node:fs";
import path from "node:path";
import type { LearnSubject, LearnTopic, QuizQuestion } from "@/bindings";

/** quiz.json's shape (both languages). */
type PackQuizQuestion = {
  prompt_en: string;
  prompt_es: string;
  options: { en: string; es: string }[];
  answer: number;
  explain_en: string;
  explain_es: string;
};

const CONTENT = path.resolve(process.cwd(), "../../content/learn");
const read = (...p: string[]) => readFileSync(path.join(CONTENT, ...p), "utf8");

type Lang = "en" | "es";
const SORTING: { id: string; en: string; es: string; problems: string[]; stable: boolean }[] = [
  { id: "bubble-sort", en: "Bubble Sort", es: "Ordenación de burbuja", problems: ["two-sum"], stable: true },
  { id: "insertion-sort", en: "Insertion Sort", es: "Ordenación por inserción", problems: [], stable: true },
  { id: "quick-sort", en: "Quick Sort", es: "Ordenación rápida (Quick Sort)", problems: ["two-sum", "valid-brackets"], stable: false },
];

export function fakeCatalog(lang: Lang): LearnSubject[] {
  return [
    {
      id: "foundations",
      name: lang === "es" ? "Fundamentos" : "Foundations",
      blurb: "Big-O",
      icon: "chart-line",
      topics: [],
    },
    {
      id: "sorting",
      name: lang === "es" ? "Ordenación" : "Sorting",
      blurb: lang === "es" ? "Burbuja, inserción, rápida" : "Bubble, insertion, quick",
      icon: "chart-column",
      topics: SORTING.map((t) => ({ id: t.id, subject: "sorting", title: t[lang], tags: ["comparison-sort"] })),
    },
    {
      id: "graphs",
      name: lang === "es" ? "Grafos" : "Graphs",
      blurb: "BFS, DFS",
      icon: "graph",
      topics: [],
    },
  ];
}

/** The topic as kb-store returns it (problem summaries are filled by the fake API). */
export function fakeTopic(id: string, lang: Lang): LearnTopic | null {
  const t = SORTING.find((x) => x.id === id);
  if (!t) return null;
  const dir = ["sorting", id];
  const quiz = (JSON.parse(read(...dir, "quiz.json")) as PackQuizQuestion[]).map(
    (q): QuizQuestion => ({
      prompt: lang === "es" ? q.prompt_es : q.prompt_en,
      options: q.options.map((o) => (lang === "es" ? o.es : o.en)),
      answer: q.answer,
      explain: lang === "es" ? q.explain_es : q.explain_en,
    }),
  );
  return {
    id,
    subject: "sorting",
    subject_name: lang === "es" ? "Ordenación" : "Sorting",
    title: t[lang],
    tags: ["comparison-sort"],
    complexity: {
      best: "O(n)",
      average: id === "quick-sort" ? "O(n log n)" : "O(n²)",
      worst: "O(n²)",
      space: "O(1)",
      stable: t.stable,
      in_place: true,
    },
    problems: t.problems,
    presets: ["random", "nearly-sorted", "reversed", "few-unique"],
    input: { min_len: 2, max_len: 16, min: 1, max: 99 },
    default_input: [38, 27, 43, 3, 9, 82, 10, 55, 21, 64],
    theory: read(...dir, lang === "es" ? "theory.es.md" : "theory.md"),
    samples: [
      { language: "python", source: read(...dir, "sample.py") },
      { language: "typescript", source: read(...dir, "sample.ts") },
    ],
    quiz,
  };
}

/** A file of a topic package, raw (samples for marker tests, topic.toml for limits). */
export const topicFile = (subject: string, id: string, file: string) => read(subject, id, file);

/** Raw sample text, for marker tests. */
export const sampleSource = (subject: string, id: string, ext: "py" | "ts") => read(subject, id, `sample.${ext}`);

/** topic.toml's `[input]` table (the only TOML the tests need). */
export function topicLimits(subject: string, id: string) {
  const text = topicFile(subject, id, "topic.toml");
  const table = /\[input\]([\s\S]*?)(?:\n\[|$)/.exec(text.replace(/\r\n/g, "\n"))?.[1] ?? "";
  const num = (k: string) => Number(new RegExp(`^${k}\\s*=\\s*(-?\\d+)`, "m").exec(table)?.[1]);
  return { min_len: num("min_len"), max_len: num("max_len"), min: num("min"), max: num("max") };
}

/** topic.toml's `presets` list. */
export function topicPresets(subject: string, id: string): string[] {
  const text = topicFile(subject, id, "topic.toml").replace(/\r\n/g, "\n");
  const line = /^presets\s*=\s*\[(.*)\]/m.exec(text)?.[1] ?? "";
  return [...line.matchAll(/"([^"]+)"/g)].map((m) => m[1]!);
}
