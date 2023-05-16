---
name: API Change Proposal
about: Propose a new API change to the libs-api team
title: "(My API Change Proposal)"
labels: api-change-proposal, T-libs-api
assignees: ""
---

# Proposal

## Problem statement

<!-- Start with a concise description of the problem you're trying to solve. Don't talk about possible solutions yet. -->

## Motivating examples or use cases

<!-- Next add any motivating examples. Examples should ideally be real world examples, or minimized versions of the real world example in scenarios where the motivating code is not open source. Don't propose changes you think might *hypothetically* be useful; real use cases help make sure we have the right design. -->

## Solution sketch

<!--
If you have a sketch of a concrete solution, please include it here. You don't have to have all the details worked out, but it should be enough to convey the idea.

If you want to quickly check whether *any* some solution to the problem would be acceptable, you can delete this section.
-->

## Alternatives

<!--
Please also discuss alternative solutions to the problem. Include any reasoning for why you didn't suggest those as the primary solution.

Could this be written using existing APIs? If so, roughly what would that look like? Why does it need to be different? Could this be done as a crate on crates.io?
-->

## Links and related work

<!-- Provide links to any <https://internals.rust-lang.org> thread(s), github issues, approaches to this problem in other languages/libraries, or similar supporting information. -->

## What happens now?

This issue is part of the libs-api team [API change proposal process]. Once this issue is filed the libs-api team will review open proposals as capability becomes available. Current response times do not have a clear estimate, but may be up to several months.

[API change proposal process]: https://std-dev-guide.rust-lang.org/feature-lifecycle/api-change-proposals.html

Some common possible responses from the libs team:

- We like this specific solution, approved, you or someone else should implement this.
- We think this problem is worth solving, but we don't think the proposed solution is the right solution. Here are some hints that might help shape a better solution.
- We don't think this should be part of the standard library. This doesn't mean the problem is unimportant, but not everything belongs in the standard library.
