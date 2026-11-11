import { useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider, type RouterHistory } from "@tanstack/react-router";
import { TooltipProvider } from "@/components/ui/tooltip";
import { createAppRouter } from "@/app/router";
import { FocusModeProvider } from "@/lib/focus";
import { PrefsProvider } from "@/lib/prefs";

export function createQueryClient() {
  return new QueryClient({
    defaultOptions: {
      // Commands are local: no retries hiding errors, no refetch on window focus.
      queries: { retry: false, refetchOnWindowFocus: false, staleTime: 30_000 },
    },
  });
}

export default function App({ history }: { history?: RouterHistory }) {
  const [queryClient] = useState(createQueryClient);
  const [router] = useState(() => createAppRouter(history));
  return (
    <QueryClientProvider client={queryClient}>
      <PrefsProvider>
        <TooltipProvider delayDuration={400}>
          <FocusModeProvider>
            <RouterProvider router={router} />
          </FocusModeProvider>
        </TooltipProvider>
      </PrefsProvider>
    </QueryClientProvider>
  );
}
