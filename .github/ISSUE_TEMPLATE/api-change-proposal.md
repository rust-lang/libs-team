---
name: API Change Proposal
about: Propose an API change for the standard library
title: ''
labels: api-change-proposal
assignees: ""
---

# Proposal

## Problem statement

<!-- Start with a concise description of the problem you're trying to solve. Don't talk about possible solutions yet. -->

## Motivating examples or use cases

<!-- Next add any motivating examples. Examples should ideally be real world examples, or minimized versions of the real world example in scenarios where the motivating code is not open source. Don't propose changes you think might *hypothetically* be useful; real use cases help make sure we have the right design. -->

## Solution sketch

<!--
Please write down all the functions, types or traits that you propose here. Make sure you include the *full type signatures* (arguments, return type, trait bounds etc.).

You don't have to include the function bodies, but the signatures are a critical portion of the API and it is really difficult to evaluate the proposal without them.
-->

## Alternatives

<!--
Please also discuss alternative solutions to the problem. Include any reasoning for why you didn't suggest those as the primary solution.

Could this be written using existing APIs? If so, roughly what would that look like? Why does it need to be different? Could this be done as a crate on crates.io?
-->

## Links and related work

<!-- Provide links to any <https://internals.rust-lang.org> thread(s), github issues, approaches to this problem in other languages/libraries, or similar supporting information. -->

## What happens now?

This issue contains an API change proposal (or ACP) and is part of the libs team [feature lifecycle]. Once this issue is filed, the libs team will review open proposals as capability becomes available. Current response times do not have a clear estimate, but may be up to several months.

[feature lifecycle]: https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html

## Possible responses

The libs team may respond in various different ways. First, the team will consider the *problem* (this doesn't require any concrete solution or alternatives to have been proposed):

- We think this problem seems worth solving, and the standard library might be the right place to solve it.
- We think that this probably doesn't belong in the standard library.

Second, if there's a concrete solution:

- We think this specific solution looks roughly right, approved, you or someone else should implement this. (Further review will still happen on the subsequent implementation PR.)
- We're not sure this is the right solution, and the alternatives or other materials don't give us enough information to be sure about that. Here are some questions we have that aren't answered, or rough ideas about alternatives we'd want to see discussed.

If we do think an API is worth implementing, we'll add the `ACP-accepted` label to your issue, at which point a [tracking issue] on the `rust-lang/rust` repository should be opened. Once the tracking issue exists, we'll close the issue as completed.

[tracking issue]: https://github.com/rust-lang/rust/issues/new?assignees=&labels=C-tracking-issue%2C+T-libs&template=library_tracking_issue.md&title=Tracking+Issue+for+XXX

In general, we prefer that authors close their own ACPs instead of the team rejecting them, since generally, the solution to a problem is "not now/differently," not "never, in no possible way." However, if an ACP ends up being merged in some other form (e.g. a similar one is accepted) or an ACP cannot be done without breaking stability guarantees, we may close it for that reason.
