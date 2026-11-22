# Classroom mode

KataBox can hand out assignments and collect results without a server or accounts. Both
travel as files you share any way you like (email, a shared drive, a USB stick).

**It is not cheat-proof.** KataBox runs offline on the student's own computer, so a
student who wants to can change their data. The results file carries a hash chain that
shows *casual* edits (a verdict changed in a text editor), but anyone can recompute it.
Use it for practice and feedback, not for high-stakes grading.

## For teachers: make an assignment
1. Write `assignment.toml`:

   ```toml
   title = "Week 3: two pointers"
   due = "2026-12-01"                    # YYYY-MM-DD
   problems = ["budget-pair", "two-sum"] # your pack's problems or built-in ids
   languages = ["python", "typescript"]  # python, typescript, php, rust
   time_limit_min = 90                   # optional, 5-600 minutes from Start
   hints = false                         # optional; false locks hints and the editorial
   ```

2. Write `pack.toml` (see the user guide's "Problem packs"), then build the pack. Your own
   problem folders are optional; an assignment of built-in problems needs none:

   ```sh
   kb-pack pack my-problem-1 my-problem-2 --manifest pack.toml --assignment assignment.toml --out week3.kbpack
   ```

3. Send `week3.kbpack` to your students.

## For students
1. **Settings → Packs → Import a pack…**, check the preview (it names the assignment),
   install, and restart.
2. Open **Classroom**. Read the rules, then press **Start** when you are ready. While it
   runs, its problems take only the listed languages, and with hints off their hints,
   editorial, earlier submissions and History are locked. Only submissions made while
   it runs count. It ends when you press **Finish now** or when the time limit passes.
3. **Export results**: type your name, choose whether to include your code, read the
   list of exactly what the file holds, and save it. Send that file to your teacher.

## For teachers: collect results
1. Open **Classroom → Class results → Import results files…** and pick every file you
   received (you can pick many at once). Each file is checked; a broken or foreign file
   is refused with the reason, the same file twice is imported once, and a file edited
   after export is kept and marked **Edited after export**.
2. The table shows one row per student and one column per problem.
3. **Export CSV…** writes one row per student and problem. Cells that could start a
   spreadsheet formula are prefixed with `'`.
