# KataBox user guide

KataBox is a free, offline desktop app for coding practice. You solve problems in
Python, JavaScript/TypeScript, PHP or Rust (and SQL for SQL problems); your code runs on
your own computer, inside a sandbox, and is judged against public and hidden tests.
Nothing is sent anywhere: the only downloads are the language runtimes you choose to
install.

What is inside:
- **209 problems:** 150 function, 35 program (stdin/stdout) and 24 SQL; 68 easy,
  90 medium and 51 hard, grouped into 27 patterns.
- **Learn:** 118 animated topics in 14 subjects, plus 3 learning paths.

Button names below are the English labels; the Spanish guide uses the Spanish ones.

## Getting started

### Install
- KataBox runs on **Windows 10/11 (x64)**. Run the `KataBox_<version>_x64-setup.exe`
  installer: it installs for your user only, with no administrator rights.
- The installer is not code-signed yet, so Windows SmartScreen may warn on first
  launch: choose **More info**, then **Run anyway**.

### The welcome flow
On the first start KataBox walks you through four short steps. **Skip the welcome**
(top right) is there at every step.
1. **Interface language:** English or Español, at the top of the welcome screen.
2. **Choose languages to install:** pick one or more of Python, JavaScript /
   TypeScript, PHP and Rust, then **Install selected** (or **Not now**). Each runtime
   is downloaded once from its official site, checked against a pinned checksum and
   kept inside KataBox's folder. The download and disk sizes are shown before you
   install.
3. **How much have you practised?**
   - **New to this:** start at the fundamentals.
   - **I've practised before:** a short **Placement** of three problems (two easy, one
     medium). Solve each one if you can, or press **Next problem**; **Finish
     placement** sets your starting pattern and difficulty.
4. **The tour:** five coach marks in the workspace (Run, Submit, Hints, Results and
   debugger, Learn). Use **Next** / **Back**, or → / ←; Esc or **Skip tour** ends it.

You can run it again from **Settings → Practice → Show the welcome again**.

### Finding your way
The rail on the left opens **Home**, **Library**, **Learn**, **Interview**,
**Progress** and **Settings**. The **Command palette** (Ctrl+K, or the button in the
top bar) jumps to any page, problem or Learn topic, and switches theme and language.

- **Home:** **Continue** (your last draft), **Solved**, the **Daily problem**, your
  **Review** queue and a **Review a topic** card.
- **Library:** search (press `/`), filter by difficulty, status, pattern, tag and
  problem type (Function, Program, SQL), sort by any column. ↑ / ↓ move, Enter opens.
  The **Lists** tab holds your own lists.
- **Progress**, in four tabs:
  - **Overview:** solved counts, a 12-month **Activity** heatmap, **Solved by
    difficulty**, the **Review queue** and **Solved by language**;
  - **By pattern:** the **Patterns** map;
  - **By topic:** solved by tag (**Show all** for every tag) and the **Learn topics
    viewed** per subject;
  - **History:** past mock interviews.

  The tab you used last opens next time.

## Problems and the workspace

### Three problem types
- **Function** (the default): write the function in the starter code; the judge calls
  it with each test's arguments and compares the returned value.
- **Program** (badge **Program · stdin/stdout**): your file is the whole program. Read
  all of standard input, write the answer to standard output. Output is compared
  token by token (some problems compare lines or allow a float tolerance). Print debug
  messages to **stderr**, since stdout is the answer. Times include each run's
  start-up.
- **SQL** (badge **SQL · one query**): write one `SELECT` (or `WITH … SELECT`) over the
  problem's tables, shown in the **Tables** tab with columns, keys and sample rows.
  Queries run read-only on SQLite through the bundled Python, so **Python must be
  installed**. The problem says whether row order matters. SQL problems only take
  SQL, and SQL has no custom tests, debugger, profiler or shrinker.

### The layout
- **Left, Problem:** tabs **Description**, **Hints**, **Editorial**, **Submissions**,
  **Notes**, **History** (and **Tables** for SQL), with the time and memory limits.
  **See it animated** links to Learn topics that use the problem.
- **Top right, the editor:** pick the **Language** in the header. Drafts save
  automatically (**Saving… / Saved**), per problem and per language. With TypeScript,
  **Check the code as** switches between **JavaScript checking** (no type annotations
  needed) and **TypeScript checking** (strict); Run and Submit work the same either
  way.
- **Bottom right, Tests and results:** **Testcases**, **Result** and **Debug**.
- Toggle panes with **Show or hide the problem** / **Show or hide tests and results**,
  or drag the dividers. **Focus mode** (Ctrl+Shift+F) hides everything but the editor
  and results.
- Code intelligence comes from language servers: basedpyright (Python),
  typescript-language-server (JavaScript/TypeScript) and rust-analyzer (Rust).
  PHP has basic completion of keywords and common functions; SQL completes the
  problem's tables and columns. The status line under the editor shows the server
  state, errors and warnings, the cursor position and a **Format** button.

### Run vs Submit
- **Run** (Ctrl+'): judges the **public** tests plus your **custom** tests. It does not
  record a submission.
- **Submit** (Ctrl+Enter): judges **every** test, hidden ones too, and records the
  result in **Submissions**. Only Submit counts towards solving, reviews, the daily
  problem and the streak.
- Every Run and Submit also saves a snapshot in **History**: select two to compare
  them, or one to compare it with the editor, and **Restore** an old version (the
  replaced code is kept as a snapshot too).
- **Solve in…** (on a solved problem) shows your accepted solution in another language,
  read-only, next to the editor, so you can port it.

### Tests
- **Public** tests (and the statement's examples) show their input and expected
  output. **Hidden** tests do not, except that the **first failure** of a Submit shows
  its input.
- **Add custom test:** enter your own input (JSON values per parameter, or one stdin
  text for a program). The expected output comes from the reference solution. Inputs
  are checked as you type; with Python installed they are also checked against the
  problem's constraints. Custom tests are not saved when you leave the problem.
- On a failing test: **Input**, **Expected** and **Your output**, the first difference
  marked, plus anything printed and, for programs, **Error output (stderr)**.

### Verdicts
**Accepted**, **Wrong answer**, **Time limit exceeded**, **Memory limit exceeded**,
**Output limit exceeded**, **Runtime error**, **Compilation error** and **Internal
error** (a judge failure, not yours). Each comes with a short message explaining it.

### Time limits per language
A problem's time limit is set for Rust; other languages get more time. The workspace
shows the limit for the language you picked, e.g. **Time limit (Python)**.
- Rust ×1, TypeScript ×2, PHP ×5, Python ×10, plus a start-up allowance per language.
- KataBox also measures your computer once (**Judge speed**, see Settings) and scales
  every limit between ×0.5 and ×3; the workspace then says **adjusted ×N for this
  computer**.

## Hints, editorial, notes and reviews
- **Hints:** three tiers, from a nudge to the full idea. Reveal them in order
  (**Reveal hint 1**, then 2, then 3).
- **Editorial:** locked until you solve the problem, or after 3 failed submissions.
- **Notes:** one Markdown note per problem (**Edit** / **Preview**), saved
  automatically.
- **Spaced reviews:** your first Accepted on a problem schedules a review in 3 days.
  - Start one from Home (**Review**) or Progress (**Review now**). In review mode every
    language starts from the starter code, and your earlier code (History,
    Submissions) is hidden until you pass.
  - Passing moves the review to 7 days, then 21; passing the 21-day review means the
    problem is mastered and leaves the queue. Failing restarts it at 3 days.
  - **Snooze 1 day**, **Remove from review**, or add a problem back with **Review in 3
    days** from the workspace.
- **Daily problem:** a new pick every local day, leaning toward your weakest pattern
  and never a solved problem while unsolved ones remain. Solving it with Submit on
  that day extends your **Daily streak**.
- **Next problem:** after an Accepted Submit, **Next** suggests the next unsolved
  problem in the same pattern, then in the following patterns. SQL problems are left
  out of the daily problem and "Next" until you have tried one.
- **Lists:** **Add to list** from a Library row or the workspace header; manage lists
  in **Library → Lists** (**New list**, **Rename**, **Delete**, drag rows or Alt+↑ /
  Alt+↓ to reorder).
- Turn the Home cards on or off in **Settings → Practice** (the **Review a topic** card
  is in **Settings → Learn**).

## Mock interviews
**Interview → Start a mock interview:**
- choose the **Duration** (30, 45 or 60 min), the **Difficulty** mix of two problems
  (e.g. Easy · Medium) and the allowed **Languages**, then **Start**;
- you get two unsolved problems you have never opened (never SQL);
- a timer shows in the top bar and the workspace, even in focus mode, and keeps
  running if you close the app;
- hints, the editorial, earlier submissions, History and **Solve in…** are **Locked
  during the mock interview**. Run, Submit, Notes and the complexity profiler still
  work;
- it ends when time is up or on **End interview**, with a **Scorecard** (solved,
  first-try passes, time used, time and submits per problem). Past interviews are on
  Progress.

## Follow-ups
Some problems have **follow-ups**: harder siblings that change one thing (many queries
instead of one, negative values, a huge input, an extra rule). Once you solve the base
problem, they open from the strip under the problem's title (**Base problem · Follow-up
1 · Follow-up 2**). A follow-up's statement says what changes; its tests, limits and
solutions are its own, and it counts as a problem of its own in your history.

## Problem packs
A pack is a set of problems someone made and shared as a `.kbpack` file. **Settings →
Packs → Import a pack…** opens one and shows what it is before anything is installed:
the name, the author *as stated* (packs are not signed), the version, the licence, how
many problems and the file's SHA-256. KataBox checks every file first and refuses
anything unsafe or malformed. Pack problems carry the pack's name as a badge, and the
Library can show built-in or pack problems only. Their text is shown without links or
images, and their code only ever runs in the sandbox. The daily problem and "next
problem" leave pack problems out unless you tick the option on the same page. A newer
version of a pack replaces the installed one; an older one can't. Uninstalling removes
the pack's files; your history on its problems stays. Changes take effect after a
restart. To make a pack: `kb-pack pack PROBLEM_DIR... --manifest pack.toml --out
my.kbpack` (see docs/contributing-content.md).

## Create problems
To make problems for a pack without writing files by hand, tick **Settings → Packs →
Show the problem editor** and open it. Create a draft by its id, then fill the form: the
titles, difficulty and pattern, the function signature, the statement, hints and
editorial (in English and Spanish, with a preview), the public and hidden tests (with the
same visual builders as custom tests), and one reference solution (**Insert the starter
code** gives the exact declaration the judge expects). Drafts are saved as you type, and
every problem left to fix is shown next to its field. **Check in sandbox** runs your
reference on every test. **Export as pack…** writes a `.kbpack` and checks it exactly
like an import before keeping it. Pack problems need one reference; built-in problems
need all four languages (docs/problem-authoring.md).

## Classroom
A pack can carry an assignment from a teacher: a title, a due date, problems, the
languages to use, an optional time limit, and whether hints stay on. Install it like any
pack, then open **Classroom**. Press **Start** when you are ready: while it runs, its
problems take only its languages and, with hints off, their hints, editorial, earlier
submissions and History are locked. It ends when you press **Finish now** or when the
time limit passes. **Export results** shows exactly what the file will hold (your name,
the assignment, the times, and per problem the best verdict, the submits, the time to
the first Accepted and the language; your code only if you tick it) before you save it.
Teachers import the files they receive under **Class results**, see one row per student,
and export a CSV. A file edited after export is marked. KataBox is offline and has no
accounts, so this is not cheat-proof. See docs/classroom.md (also in Spanish and
Portuguese).

## Focus sessions
Start a focus session from Progress → Overview or the command palette ("Start focus
session"). A chip in the top bar shows whether you are working or on a break and how long
is left; a short notice appears when the phase changes. Work and break lengths are on the
same card (25 and 5 minutes by default). The session keeps running if you close and reopen
KataBox. When you stop it, the card lists what you did: problems touched and solved, katas
and plan items.

## Automatic backups
Settings → Data → **Automatic backups**: choose a folder and KataBox writes an export of
your progress there every few days (7 by default) and, if you like, whenever it closes.
**Back up now** writes one right away. Restore one with **Import**. KataBox keeps the
newest backups (5 by default) and deletes only its own `katabox-backup-….kbx.json` files
in that folder, never anything else; a README in the folder says so. Using a synced
folder (OneDrive, Dropbox…) is your choice.

## Building custom tests
Under each custom test's value, **Build visually** opens a builder that fits the type: a
tree you grow by clicking nodes, rows you can add, remove and reorder, a grid whose cells
you click to change, or a graph where you click two nodes to join them. The JSON field
above stays in sync and editable. **Check inputs** checks every custom test against the
problem's constraints (in the sandbox, without running your code).

## Stress test
When your solution passes the examples but you are not sure about it, open the
**Stress test** tab under the editor. Write a slow but obviously correct brute force (for
example, try every pair), then **Start stress test**: KataBox runs both on many small
inputs in the sandbox until they disagree, and shows that input with both outputs. From
there you can debug it, add it to your custom tests or shrink it. Your brute force is
kept for next time. Without the Python runtime the inputs are small random values that
are not checked against the problem's constraints; program problems need Python. SQL
problems have no stress test.

## Reading code
**Learn → Reading code** has short snippets to read, in Python or TypeScript (switch
above the code):
- **What does this print?** Pick the output, or type it when there are no choices.
- **Complexity:** pick how the time and the extra space grow with n. The answer marks
  the line that dominates the cost.
Every answer shows the explanation. Your progress is kept per item and travels with an
export. Some Learn topics link reading items from their Practice tab.

## Fix the bug
Problems with the **Fix** badge start with code that has a bug. The statement tells the
story; the code fails some tests, and your job is to find the bug and fix it. Python and
TypeScript start from the buggy code (some problems in PHP and Rust too); a language
without a buggy version starts empty, and the banner above the editor says so.
**Reset to the buggy code** in that banner brings the original code back (your snapshots
in History stay). The Library's **Kind** filter lists only these problems, or hides them.
The daily problem and "next problem" never suggest them.

## Kata drills
**Kata** (in the navigation rail) re-solves problems you have already solved, from memory
and against the clock:
- **Daily kata:** three solved problems picked for today, leaning towards the ones you
  haven't touched for a while. About 15 minutes; the clock shows overtime but never
  stops you.
- **Pick a set:** up to five solved problems from one pattern or one of your lists, the
  rustiest first, ten minutes each.
- The editor starts from the starter code. Hints, the editorial, your submissions,
  History and notes are **Locked during the kata** until it ends.
- A kata never changes your submissions, problem status, reviews or daily problem. Its
  results are on **Progress → Katas**: your personal bests (fastest time, fewest
  submits), the kata streak and recent katas.
- **Rusty** (Home card, and on Progress → Katas) lists solved problems you may have
  forgotten: the score grows with the time since you last solved one and with failed
  katas on it. Turn the card off in Settings → Practice.

## Skill rating and study plans
- **Skill rating:** Progress → Overview shows an overall rating, and By pattern one per
  pattern with a small trend line. It is **estimated** from the problems you solved
  (their difficulty, your submits and your time) and guides what the daily problem and
  "Next problem" suggest: a weaker pattern comes up more often, and easy problems you
  have outgrown are skipped. It is a guide, not a grade.
- **Study plan:** Progress → Plan → **Make the plan** from a goal (every pattern, or
  the ones you pick), a number of weeks and the time you have a day. Each pattern's Learn
  topics come first, then its unsolved problems; the last day of each week has a kata
  and your reviews. Mark items **Done**, **Skip** them or move them **Later** (the rest
  of the plan moves with them). Solved problems and viewed topics are ticked off on
  their own. Today's items are on the Home card **Study plan** (turn it off in Settings
  → Practice). **Export to calendar…** saves the plan as an `.ics` file for any
  calendar app.

## Mistake journal and reflections
- After a failed **Submit**, KataBox tags what it can tell: wrong answer, too slow, too
  much memory, the kind of runtime error (index out of range, division by zero, recursion
  too deep…) and, from the failing input, edge cases such as an empty input, one element,
  the largest value, negative numbers or duplicates. Open the submission in the
  **Submissions** tab to change the tags and add a note.
- **Progress → Mistakes** shows **Your common mistakes**: how often each tag appears,
  with the latest examples.
- After **Accepted**, you can write in one sentence **why it works** and pick its time
  and space complexity. Your note comes back at the top of the **Notes** tab, after a
  review passes and after you solve the problem in a kata.

## Snippets, comparing submissions and search
- **Snippets:** Settings → Editor → **Snippets** keeps your own code per language. In
  the editor, type `/snip` and pick one; or press Ctrl+K and choose **Insert snippet: …**.
- **Compare submissions:** in the **Submissions** tab, **Compare** and tick two: they
  are shown side by side.
- **Search everything** (Ctrl+Shift+P, or from the command palette) finds problems,
  Learn topics, your snippets, settings and sections of this guide. **Help** (from the
  palette) shows this guide inside the app.

## Debugger and visualizer
- **Languages:** Python and JavaScript/TypeScript, for function and program problems.
  Not PHP, Rust or SQL yet.
- **Start:** **Debug this case** on any test (or a failure), or F5 to debug the first
  public test. Set breakpoints by clicking just right of a line number, or F9 on the
  cursor's line.
- **Replay, not live:** each step re-runs the test in the sandbox up to that point, so
  you can also **Step back**. A program that behaves differently on each run (time,
  randomness) cannot be replayed. Very long runs stop recording after a number of steps;
  you can still step through those. The editor is read-only while debugging.
- **Toolbar:** **Continue** (F5), **Step over** (F10), **Step into** (F11), **Step out**
  (Shift+F11), **Step back** (Shift+F10), **Restart** (Ctrl+Shift+F5), **Stop
  debugging** (Shift+F5).
- **Panels:** **Variables** (Locals / Globals, expandable), **Watch**, **Call stack**
  and **Debug console** (evaluate in the selected frame; it never changes the program).
- **Visualize tab:** draws the variables you choose at each step: arrays with pointer
  variables, grids, linked lists, trees, sets, maps and graphs, with changes since the
  previous step marked. **Draw … as** switches between **Auto**, **Graph** and **Heap
  (tree)**; **Open larger** shows a zoomable view. Large values are cut so each step
  stays fast.

## Complexity profiler
On problems that support it, an Accepted Run or Submit shows a **Complexity** button
in the Result tab.
- It runs your solution on inputs of growing size (3 times each, about 15 seconds at
  most) and fits common growth curves.
- The result is always **Estimated: O(…)** (sometimes "O(a) or O(b)"), with a
  confidence note, a log-log chart against the reference solution and a table of
  times. Timings vary between runs and machines, so it is an estimate, not a proof.
- For programs, start-up time is subtracted; if what remains is too small, it says
  so instead of guessing.

## Find a smaller failing input
On a failing test you can see (public, custom, or the first failure) after Wrong
answer, Runtime error, Time limit or Memory limit exceeded:
- **Find a smaller failing input** shrinks the input step by step while your code
  still fails on it (at most 60 sandbox runs or 30 s; **Stop** anytime).
- The result offers **Debug this case** and **Add to my tests**.
- Function problems only: a program's input has counts and structure lines that can't
  be cut safely.

## Learn
**Learn** teaches algorithms and data structures by animation.
- **Home:** search topics, **Continue** where you left off, **Learning paths** (Start
  here, Interview essentials, Graph master) and the 14 **Subjects** with your progress.
- **A topic** has a stage (the animation) and three tabs:
  - **Theory**, with links to related topics;
  - **Code**: a tested sample in Python or TypeScript; the highlighted line is the one
    behind the current step, with its **Variables**;
  - **Practice**: **Predict the next step** questions, then **Practice problems** that
    open in the workspace.
- **The player:** **Play** / **Pause**, **Step back**, **Step forward**, **Restart**,
  a **Timeline** to scrub, and **Speed** (0.5×, 1×, 2×, 4×). Change the **Input** with a
  preset or your own values (**Apply**).
- **Compare with…** (on topics that take the same input, such as sorting and
  searching): two algorithms side by side on one input.
- A topic counts as **viewed** when you reach its last step by playing or stepping.
- Links back to practice: **Learn:** links in the Library's pattern filter and the
  Progress pattern map, **See it animated** in the workspace, and the **Review a topic**
  card on Home.

## Export and import
**Settings → Data:**
- **Export…** writes your progress to one file (`.kbx.json`, format
  `katabox-export/v2`; files from v1 still import): problem statuses, submissions, History snapshots, notes,
  drafts, unlocked hints, opened problems, reviews, daily problems, lists, mock
  interviews, settings, and your Learn progress (viewed topics, Continue position).
- Not exported: this computer's speed factor, its install id and sandbox folder.
- **Import…** shows a preview (what the file holds, conflicts) before you **Import**.
  It merges by problem and keeps the newer record; importing the same file twice
  changes nothing. Invalid files are refused whole, and nothing in a file is executed.

## Settings
Settings has one section per topic, listed on the left (or as tabs above, in a narrow
window). **Search settings** finds a setting by its name.

- **Appearance:**
  - **Theme** (System, Light, Dark), **Language** (English, Español, Português
    (Brasil)), **Contrast** (Normal, High), **Motion** (Follow the system, Reduce);
  - with the Portuguese interface, **Problems and lessons in** picks English or Spanish
    for problems and Learn, which are written in those two languages (English by default);
  - **Reset window size and position**. KataBox reopens where you left it, at the same
    size, and maximised if it was. If that screen is gone, it opens centred on the main
    screen.
- **Editor:** **Keymap** (Default, VS Code, Vim, Emacs), **Font ligatures**, **Font
  size** (12-20 px), **Tab width** (Language default, 2, 4 or 8 spaces), **Colour
  theme** (KataBox, High ink, Paper, Dusk), with a live **Preview**.
- **Practice:** **Daily problem on Home**, **Review reminders on Home**, **Show the
  welcome again**.
- **Languages and runtimes:**
  - install or **Uninstall** runtimes. Uninstalling keeps your drafts, notes and
    submissions;
  - **Judge speed:** the **Speed factor** for this computer, when and with what it was
    measured, and **Recalibrate**.
- **Learn:** **Review a topic card**, and the **Animation speed**.
- **Data:** **Export…** / **Import…** (above).
- **About:** versions, the **Data folder** and **Sandbox folder**, **Third-party
  notices** and **Copy diagnostics…**.

## Keyboard shortcuts
A **Keyboard shortcuts** dialog lists them in the app: press F1, pick it from the
command palette, or use the keyboard button in the top bar.

| Where | Keys | Action |
| --- | --- | --- |
| Anywhere | Ctrl+K | Command palette |
| Anywhere | Ctrl+Shift+P | Search everything |
| Library | `/` | Search problems |
| Library | ↑ / ↓, PageUp / PageDown, Home / End, Enter | Move through the list, open a problem |
| Library → Lists | Alt+↑ / Alt+↓ | Move a list row |
| Workspace | Ctrl+' | Run |
| Workspace | Ctrl+Enter | Submit |
| Workspace | Ctrl+Shift+F | Focus mode on/off |
| Workspace | Esc (outside the editor) | Leave focus mode |
| Editor | Shift+Alt+F | Format document (with a language server) |
| Editor | F12 | Go to definition (with a language server) |
| Editor | Ctrl+F, Ctrl+Z, Ctrl+Y | Search, undo, redo (Vim and Emacs keymaps add their own keys) |
| Debugger | F9 | Toggle a breakpoint on the cursor's line |
| Debugger | F5 | Start debugging / Continue |
| Debugger | F10 / Shift+F10 | Step over / Step back |
| Debugger | F11 / Shift+F11 | Step into / Step out |
| Debugger | Ctrl+Shift+F5 / Shift+F5 | Restart / Stop debugging |
| Learn player | Space | Play or pause |
| Learn player | ← / → | Step back / forward |
| Learn player | Home | Restart |
| Progress heatmap | Arrow keys, Home / End | Move between days |
| Tour | → or Enter, ←, Esc | Next, back, end |

## Where your data lives
- **Your progress** (user.db: drafts, submissions, notes, settings):
  `%APPDATA%\dev.katabox.app`.
- **Runtimes, sandbox and caches:** `%LOCALAPPDATA%\dev.katabox.app`.
- **The app itself:** `%LOCALAPPDATA%\KataBox`.
- **Settings → About** shows the exact data and sandbox folders.
- **Uninstalling** (Windows Settings → Apps) removes the app but keeps your data,
  unless you tick **Delete the application data** in the uninstaller. The uninstaller
  also removes the sandbox's Windows profile and folder permissions.
- **To remove data by hand** after uninstalling, delete the two folders above (paste
  `%APPDATA%\dev.katabox.app` and `%LOCALAPPDATA%\dev.katabox.app` into File Explorer's
  address bar). Export first if you want to keep your progress.

## Getting help
- **Settings → About → Copy diagnostics…** copies versions, settings flags and recent
  error codes for a bug report. It shows exactly what will be copied: no code, file
  paths or personal data.
- Known problems and workarounds: [known issues](known-issues.md).
