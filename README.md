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

## High-level concept

![](https://s3.edgyfn.app/koalafy/misc/3e5b0b928ba194a46e3c23d1804640a9.png)

## Run in development

Just run `cargo run`. For more verbose logging, you can set `RUST_LOG` with `trace` level so all log will appear
in `stdout`:

```bash
JWT_SECRET=<some_jwt_secret> REDIS_HOST=<some_redis_host> RUST_LOG=trace cargo run
```

Also, you need `koalaci` (docker) image for this. Build it (~900MB) by running this command:

```bash
cd ci; sh build.sh
```

In case there are permission-related error related with `.sh` file, make sure to give
execute permission by using `chmod +x`.

And yes, make sure Redis is listening into default host & port.

## API

This app is (User) Interface & Auth free, it means like "bring your own UI & Authentication strategy". Currently we're using
[JWT](https://jwt.io) for authorization & [JSON](https://www.json.org/json-en.html) for view. Its commonly used in web
application development so its not a big deal.

Currently each endpoint is requiring **Authorization: Bearer `<token>`** header in each request, where the `<token>` is
base64-encoded string of JWT.

### Create Job

Method: **POST** — Endpoint: `/job/create`

Payload & response:

```
// payload

{
  "project_id": "String",
  "deployment_id": "String",
  "project_branch": "String",
  "project_repo_url": "String",
  "project_build_command": "'String'",
  "project_package_manager": "String",
  "project_dist_directory": "String"
}

// response 200/401/500

{
  "data": "OK"
}
```

### Get Job log

Method: **Get** — Endpoint: `/job/:id/log`

Response:

```
// response 200/401/404/500

{
  "data": "Some long job log"
}
```

### Cancel Running Job

Method: **Patch** — Endpoint: `/job/:id/cancel`

Response:

```
// response 200/401/404/500

{
  "data": "Queued for cancellation"
}
```


## Deployment

This app is bundled into Container Image, with `rust:stable` as a base image and `docker:stable` as runtime
image. We use docker in docker (fortunately it was Alpine!) and mounting the local `/var/run/docker.sock` so we can
call docker command (inside docker container) on the docker host.

We use docker-in-docker approach to make sure the "build" process is isolated from any interruption and
to avoid some unexpected side-effect ;)

Also, we mount local directory into `/opt/koalaci` in container for caching & artifacts things, so make sure to configure it.
