# API Guidelines

## Writing a 2021 Edition of the API Guidelines

Some patterns have changed and new idioms have emerged since we last spent time on the API Guidelines.
There are some new language features like `impl Trait` and `async` / `await` that haven't been fully considered in the existing guidelines yet.

As of this writing, it's been about two years since any substantial new content was added to the API Guidelines.
That's not necessarily an issue, we could consider the API Guidelines "done" for the 2018 language edition.
Coming up to 2021 we should revist existing recommendations and see if any new ones have come up that make sense for the new edition.

## Who owns the API Guidelines?

This MCP proposes the Libs team as a whole be responsible for the recommendations made by the API Guidelines.
That's basically always been the case, but it's worth making clear.

## Enabling open discussion and clear decisions

The API Guidelines generate a lot of interest, because anybody building libraries in the ecosystem can use and contribute to them.

It's been a while since new content was added, but discussion on guidelines has continued in GitHub issues on the repository.
GitHub issues historically don't scale to live discussion and need regular gardening to keep them useful and directed.
There's a sense of debt in each open issue that isn't really the case for the API Guidelines.
GitHub's issues UX simply isn't geared to surfacing a treasure-trove of design insight for users to explore.

Going forward, we should direct discussion, questions, and proposals related to guidelines to GitHub [discussions] instead of [issues].
The disussions feature in GitHub is a lot like issues, except discussions don't have an open or closed status and the UX is better suited to unbounded threaded comments.
Discussions also offer a question/answer UX somewhat like Stack Overflow that we might want to look at sometime.

Most activity on the API Guidelines will be expected to take place in discussions, which will form a useful resource in their own right for anybody looking for a place to talk and read about API design in Rust.
Issues will be reserved for concrete acitonable changes to the API Guidelines content that are either uncontroversial enough to accept on-the-spot or issue an FCP on.
If an issue or pull request is opened that isn't the result of an existing discussion, or that a member of the Libs team doesn't feel is concrete enough to FCP for, then it should be converted into a discussion or closed.
Pull requests that aren't for accepted changes to the guidelines (either through an FCP or on-the-spot approval) should be closed.
Comments on issues with a proposed FCP should be limited to interacting with `rfcbot` to raise and resolve blocking concerns.
Actual discussion of concerns should be done back in the relevant discussion.
Issues will be a tool for coordinating concensus and surfacing accepted work, and discussions will be the place to talk about API design.

[discussions]: https://github.com/rust-lang/api-guidelines/discussions
[issues]: https://github.com/rust-lang/api-guidelines/issues
