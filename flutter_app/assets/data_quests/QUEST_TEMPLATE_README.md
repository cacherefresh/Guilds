# Quest Template (YAML)

This file shows the YAML structure to use for quest files. Fields:
- `quest`: object containing quest metadata
- `goals.required`: list of required goals (each with `required_goal` and optional `prereqs` list)
- `goals.bonus_optional`: list of bonus goals (each with `bonus_goal` and `bonus_completed` boolean)

---
quest:
  name: "find_lost_code"            # unique machine name for the quest
  display_title: "Find the Lost Code" # user-visible title
  difficulty: "Medium"               # e.g., Easy, Medium, Hard
  active: true                        # boolean: is the quest currently active
  start_date: 2026-02-08              # ISO date or null if not started
  completed_date: null                # ISO date or null if not completed
  long_description: >
    Journey into the old repository to recover the lost algorithm. You will
    need to collect fragments from several locations and assemble them to
    reconstruct the original code.

  goals:
    required:
      - required_goal: "collect_fragments"
        # prereqs references other quest `name` values (0..many)
        prereqs: []

      - required_goal: "assemble_fragments"
        prereqs:
          - "collect_fragments"

    bonus_optional:
      - bonus_goal: "find_secret_easter_egg"
        bonus_completed: false

# Example notes:
# - Use `start_date` and `completed_date` as ISO dates (YYYY-MM-DD) or `null`.
# - `prereqs` is an array of other quest `name` values; it can be empty.
# - The `required` list is ordered; objectives should be met in sequence if desired.
