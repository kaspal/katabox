// Focus mode (ADR 0013): the workspace asks the shell to hide its chrome.
import { createContext, useContext, useMemo, useState } from "react";

type Focus = { focus: boolean; setFocus: (on: boolean) => void };

const FocusContext = createContext<Focus>({ focus: false, setFocus: () => {} });

export function FocusModeProvider({ children }: { children: React.ReactNode }) {
  const [focus, setFocus] = useState(false);
  const value = useMemo(() => ({ focus, setFocus }), [focus]);
  return <FocusContext.Provider value={value}>{children}</FocusContext.Provider>;
}

export const useFocusMode = () => useContext(FocusContext);
