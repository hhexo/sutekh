# Sutekh

Sutekh, the god of chaos, is a testing utility designed to randomly kill Docker
containers or Kubernetes pods in a clustered set so that its resilience can be
tested.

It's nothing special, really. You could do that with a Bash script too.

The current version kills a container every X seconds, where X is exponentially
distributed with a mean value of 10.

# Usage

## Docker containers

You can build a local `sutekh` image with:

```bash
docker build -t sutekh .
```

Then you can run it and randomly kill all containers with "blah" in the name
like this:

```bash
docker run -d -v /var/run/docker.sock:/var/run/docker.sock sutekh sutekh blah
```

(the first `sutekh` is the image name, the second one the command to execute)

## Kubernetes pods

Because you need your `kubectl` to be configured with your own Kubernetes
cluster and secrets, you need to customize your own Docker image to support
that.

First build the Sutekh image as above:

```bash
docker build -t sutekh .
```

Then create your own image using `FROM sutekh:latest` and make sure your
`kubectl` is set up with all the information it needs.

Then you can run your image with the command `sutekh --kube blah` to kill all
pods with "blah" in their name.
