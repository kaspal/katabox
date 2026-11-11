import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { createMemoryHistory } from "@tanstack/react-router";
import App from "@/App";

/** Render the whole app at `path` (memory history). `@/lib/api` must be mocked. */
export function renderApp(path = "/") {
  const history = createMemoryHistory({ initialEntries: [path] });
  const user = userEvent.setup();
  const view = render(<App history={history} />);
  return { ...view, user, history };
}
