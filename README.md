# koalaCI

A minimal ~~ci~~ "builder" for Koalafy's Dedicated Hosting service.

Currently this program is single-thread and heavily rely to bash script.

## Prerequisites

- Docker
- Bash
- Rustc
- Redis

## Concept

As far as I know, the build process is like this:

- [x] Get source code (commonly from Remote repository)
- [x] Install dependencies (cached deps is better, right?)
- [x] Since this is not a CI platform™, we skip the `test` stage ;)
- [ ] Build the project (incremental build is better)
- [x] Save cache (both for deps & cache artifacts)
- [ ] Upload artifacts
- [ ] Handle error signal from docker

This is still work in progress, so the example payload can be find on `main.rs`.

```rs
let mut example_job = job::Job::new(
  redis_instance, // redis instance for storing log & build status
  String::from("p_ke4guri8dh2pgpx"), // project Id (for deps cache)
  String::from("d_fj2icskbtuo9odz"), // deployment Id
  String::from("master"), // (project) repo branch
  String::from("https://github.com/evilfactorylabs/alchemy.git"), // (project) repo url
  String::from("'npm run build && npm run export'"), // (project) build command
  String::from("npm"), // (project) package manager
  String::from("out"), // (project) dist directory
);
```

Payload above will run "build process" for `deployment_id` of `project_id` using `repo_url` as source
and `build_command` as build command. Very obvious.

## Problem

As today, we are very rely on `stdout` to store the log process into redis database. The problem,
currently we can't capture the "trap" exit (via SIGTERM) to test, so that's why I push the code for getting help!

## Run in development

Just run `cargo run`. For more verbose logging, you can set `RUST_LOG` with `trace` level so all log will appear
in `stdout`:

```bash
RUST_LOG=trace cargo run
```

Also, you need `koalaci` (docker) image for this. Build it (~900MB) by running this command:

```bash
cd ci; sh build.sh
```

In case there are permission-related error related with `.sh` file, make sure to give
execute permission by using `chmod +x`.

And yes, make sure Redis is listening into default host & port.
