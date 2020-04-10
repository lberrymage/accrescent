# Contribution guide

First of all, thanks for considering (contributing to) my project! If you want
to help me in this venture, here's what you need to know:

- [Code style](#code-style)
- [Git guidelines](#git-guidelines)
- [Submitting pull requests](#submitting-pull-requests)

## Code style

All code style information can be found in [CODESTYLE.md](CODESTYLE.md). This
includes documentation guidelines and miscellaneous preferences.

## Git guidelines

These are guidelines you should follow when making changes with `git`. I know,
it seems unnecessary, but I like a pretty `git log`, don't you? Please note that
I _really_ do want these rules to be followed, so if you ignore them repeatedly,
I may have to ignore your PRs. With that, they're pretty easy to follow, and if
you do forget something, it's not that big of a deal.

Almost everything is in [the seven rules of a great git commit message][rules].

The only other thing I'll mention is that if you're updating your fork with
upstream changes, please use `git rebase` to avoid merge commits.

## Submitting pull requests

To run integration tests, use the `inspector` script found in the crate root. It
is simply a `cargo` wrapper with linting preferences built-in.

We use the [GitHub flow] workflow in this repository. You may already be
familiar with it, but just in case, here's how you use it:

1. [Fork] the repository.
2. Create a new topic branch.
3. Make your changes.
4. When you're done, submit a PR to the `master` branch.

It's that simple!

If you're adding a new feature, please make sure there's a relevant feature
request issue that has been accepted. We don't want you to spend time on a
feature which will ultimately be rejected.


[Fork]: https://github.com/lberrymage/accrescentfork
[GitHub flow]: https://guides.github.com/introduction/flow/
[rules]: https://chris.beams.io/posts/git-commit/#seven-rules
