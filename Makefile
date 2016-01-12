default:
	set -xe
	docker build --rm -t zalando/rusty-oauth-build .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock -ti zalando/rusty-oauth-build

docker-run:
	docker run -p 6767:6767 -e RUST_LOG=debug -t danpersa/rusty-oauth:latest

docker-push:
	docker push danpersa/rusty-oauth:latest
