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

A test MAY be added without asking.
A specification MUST NOT be added until the user explicitly asks for one.
The name of a test case in a specification SHOULD start with "should", unless the language, the test framework, or the project's own conventions call for another form.
A test case MUST run from as near the entry point as it can to as near the outside as it can.
An external server MUST be replaced with a fake.
Any other part MUST NOT be replaced with a fake, with only these exceptions:

- where the test cannot have the real thing
- where the real thing cannot be made to do what the test needs it to do

### Git

Commit messages and pull requests MUST be written in plain English and be as concise as possible, so that as many people around the world as possible can read them.
Pull request descriptions MUST NOT be hard wrapped.
Commit messages and pull request descriptions MUST carry the attribution of the AI agent that helped write the change.
Branch names MUST follow Conventional Branch, and commit messages MUST follow Conventional Commits.
Pull request titles MUST follow Conventional Commits too, because a squash merge makes the title the commit message.
Work MUST be done on a topic branch unless the user explicitly says otherwise.
Before a branch is created, the latest changes on the remote default branch MUST be pulled in.
Changes MAY be committed and pushed to a topic branch without asking.
A pull request MUST NOT be opened until the user asks for one.

### Structure

Anything written, whether code, a document, a commit message, a pull request, or a reply, SHOULD follow The Newspaper Metaphor: the top gives the whole, and each step down goes into more detail.
Where a language or the project's own conventions call for another order, that order wins.

### Replies

Replies to the user MUST be written in Japanese.
Replies to the user MUST be understandable to someone who has not read the source code.
Replies to the user MUST follow The Newspaper Metaphor.
In a repository the user owns, the lead MUST be left to the user.
Replies MAY ask the user for something, but MUST NOT instruct the user.

## When in Doubt

A doubt SHOULD be resolved by asking, not by guessing. A question costs far less than work built on a wrong assumption.
