# API Guidelines

## Writing a 2021 Edition of the API Guidelines

Some patterns have changed and new idioms have emerged since we last spent time on the API Guidelines.
There are some new language features like `impl Trait` and `async` / `await` that haven't been fully considered in the existing guidelines yet.

As of this writing, it's been about two years since any substantial new content was added to the Guidelines.
That's not necessarily an issue, we could consider the Guidelines "done" for the 2018 language edition.
Coming up to 2021 we should revist existing recommendations and see if any new ones have come up that make sense for the new edition.

## Enabling open discussion and focused reviews

The API Guidelines generate a lot of interest, because anybody building libraries in the ecosystem can use and contribute to them.

It's been a while since new content was added, but discussion on guidelines has continued in GitHub issues on the repository.
GitHub issues historically don't scale to live discussion and need regular gardening to keep them useful and directed.
There's a sense of debt in each open issue that isn't really the case for the API Guidelines.
GitHub's UX simply isn't geared to surfacing a treasure-trove of design insight for users to explore.

Going forward, we should direct open discussion to the `#t-libs/api-guidelines` Zulip stream.
This doesn't entirely sidestep the issue of needing to curate discussion and adds ambiguity on whether to go to GitHub or Zulip.
On Zulip though we can naturally let conversations run their course and leave them to be searched later without having to send any terminal signals by closing them.

Once discussion on Zulip has settled on a guideline then a PR to `rust-lang/api-guidelines` should be opened to propose it.
We should try keep the bar for proposing new guidelines high, so that any that appear should be possible to evaluate in Libs meetings.
