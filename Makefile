default:
	set -xe
	docker build --rm -t zalando/rusty-oauth-build .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock -ti zalando/rusty-oauth-build
