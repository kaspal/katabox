// Reading exercises (expansion M7, ADR 0035): the sets, progress and the answer check.
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { ReadingAnswer, ReadingAnswerRow } from "@/bindings";
import { api } from "@/lib/api";
import { usePrefs } from "@/lib/prefs";

export const readingKeys = {
  sets: (lang: string) => ["reading_sets", lang] as const,
  set: (id: string, lang: string) => ["reading_set", id, lang] as const,
  answers: ["reading_answers"] as const,
  links: (topic: string, lang: string) => ["reading_links", topic, lang] as const,
};

export function useReadingSets() {
  const { lang } = usePrefs();
  return useQuery({ queryKey: readingKeys.sets(lang), queryFn: () => api.readingSets(lang), staleTime: Infinity });
}

export function useReadingSet(id: string) {
  const { lang } = usePrefs();
  return useQuery({ queryKey: readingKeys.set(id, lang), queryFn: () => api.readingSet(id, lang), staleTime: Infinity });
}

export function useReadingAnswers() {
  return useQuery({ queryKey: readingKeys.answers, queryFn: () => api.readingAnswers() });
}

export function useReadingLinks(topic: string) {
  const { lang } = usePrefs();
  return useQuery({
    queryKey: readingKeys.links(topic, lang),
    queryFn: () => api.readingLinks(topic, lang),
    staleTime: Infinity,
  });
}

export function useAnswerReading() {
  const { lang } = usePrefs();
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ item, answer }: { item: string; answer: ReadingAnswer }) => api.answerReading(item, answer, lang),
    onSuccess: () => void qc.invalidateQueries({ queryKey: readingKeys.answers }),
  });
}

/** Items of `set` answered correctly at least once. */
export function solvedIn(set: string, answers: ReadingAnswerRow[] | undefined): number {
  return (answers ?? []).filter((a) => a.item_id.startsWith(`${set}/`) && a.correct_at !== null).length;
}

/** The time and space answers a complexity item offers (the classes kb-pack checks). */
export const TIME_CLASSES = ["O(1)", "O(log n)", "O(n)", "O(n log n)", "O(n²)", "O(2ⁿ)"] as const;
export const SPACE_CLASSES = ["O(1)", "O(log n)", "O(n)", "O(n²)"] as const;
