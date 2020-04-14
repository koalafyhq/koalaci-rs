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
- [x] Handle error signal from docker

To trigger new job/deployment, send POST request to `/job/create` with following payload:

```json
{
  "project_id": "String",
  "deployment_id": "String",
  "project_branch": "String",
  "project_repo_url": "String",
  "project_build_command": "String",
  "project_package_manager": "String",
  "project_dist_directory": "String"
}
```

Also, you need `Authorization` header with JWT format. Currently the JWT payload is like this:

```json
{
  "user_id": "String"
}
```

`user_id` is required for storing job status & log into Redis database as an identifier.

Example:

```bash
curl -X "POST" "<protocol>://<host>:<port>/job/create" \
     -H 'Authorization: Bearer <jwt token>' \
     -d $'{
  "project_id": "project_id_commonly_for_caching_thing",
  "deployment_id": "deployment_id",
  "project_branch": "project_branch",
  "project_repo_url": "project_repo_url",
  "project_build_command": "'"project_build_command"'",
  "project_package_manager": "project_package_manager",
  "project_dist_directory": "project_dist_directory"
}'

```

If the response is `200`, the "build" job will start to run.

Payload above will run "build process" for `deployment_id` of `project_id` using `repo_url` as source
and `build_command` as build command. Very obvious.

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
