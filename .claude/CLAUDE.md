## Rules

### Comments

Comments and embedded documentation MUST NOT be written in any file.
The only exceptions:

- a comment that explains why, only where the reason cannot be understood without it, and as concise as possible, so that it is less likely to go stale or to say something untrue
- "Arrange", "Act", and "Assert" comments that mark test sections

### Tests

A test case MUST run from as near the entry point as it can to as near the outside as it can.
A part MAY be replaced with a fake only where the test cannot have the real thing.
Where it can, a fake is allowed only when the real thing would let the assertion pass where the specification does not, and another test case states what that part does.

### Git

Commit messages and pull requests MUST be written in plain English and be as concise as possible, so that as many people around the world as possible can read them.
Pull request descriptions MUST NOT be hard wrapped.
Commit messages and pull request descriptions MUST carry the attribution of the AI agent that helped write the change.
Branch names MUST follow Conventional Branch, and commit messages MUST follow Conventional Commits.
Before a branch is created, the latest changes on the remote default branch MUST be pulled in.

## When in Doubt

Ask instead of guessing. A question costs far less than work built on a wrong assumption.
