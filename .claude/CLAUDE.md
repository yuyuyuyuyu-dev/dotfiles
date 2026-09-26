## Rules

### Comments

Comments and embedded documentation MUST NOT be written in any file.
The only exceptions, allowed in a test case alone:

- "Arrange", "Act", and "Assert" comments that mark test sections
- a comment that explains why, only where the user has explicitly asked for it

### Design

Whether something can be reused MUST NOT be considered.
Whether the result is how it should be MUST be considered instead.

### Libraries

A library or framework MAY be introduced without asking, as long as its license is complied with.
Where the license is unclear or cannot be complied with, the library MUST NOT be introduced.

### Tests

A test case MUST run from as near the entry point as it can to as near the outside as it can.
A part MAY be replaced with a fake only where the test cannot have the real thing.
Where it can, a fake is allowed only when the real thing would let the assertion pass where the specification does not, and another test case states what that part does.

### Git

Commit messages and pull requests MUST be written in plain English and be as concise as possible, so that as many people around the world as possible can read them.
Pull request descriptions MUST NOT be hard wrapped.
Commit messages and pull request descriptions MUST carry the attribution of the AI agent that helped write the change.
Branch names MUST follow Conventional Branch, and commit messages MUST follow Conventional Commits.
Pull request titles MUST follow Conventional Commits too, because a squash merge makes the title the commit message.
Before a branch is created, the latest changes on the remote default branch MUST be pulled in.

### Replies

Replies to the user MUST be written in Japanese.
A report or explanation MUST start with an overview of the whole, then go into detail one step at a time.

## When in Doubt

Ask instead of guessing. A question costs far less than work built on a wrong assumption.
