# Berlin Rust Hack and Learn: The NAIVE<sup>*</sup> Sessions

## 2024-02-29: NAIVE<sup>*</sup> session #002 – Workspaces

<sup>*</sup>Novice And Intermediate Vocational Exercises

### Challenge description

We would like to host the NAIVE sessions on Github. What is the best way to organize the single sessions, so that we do not clutter the repository with too many individual projects?

We discussed and tried the following questions:

#### Should we organize the individual NAIVE sessions as cargo crates and collect them in a cargo workspace?

A workspace is not required in regard to Github repository organization. A simple parent folder does the job. A workspace for multiple cargo crates is only recommended, if the individual crates are somehow in dependency of each other.

#### There is no cargo command for creating new workspaces?

No, actually not. There is a pending feature request, but it is not available yet. In consequence, the workspace parent directory and its Cargo.toml have to be created manually. Git also needs to be initialized in the workspace root and .git-directories in the sub-crates should be removed.

#### As there is only one target dir on the workspace, how about naming conflicts?

The compiler will throw an error in case of naming conflicts.

#### How do we proceed with the NAIVE repo?

We created a parent folder (NOT a workspace!) and initialized git for it. We then moved the existing project folder inside and deleted the .git subdirectory from it. We checked in the parent directory to the Github repository.
