default:
	set -xe
	docker build --rm -f docker/Dockerfile -t zalando/rusty-oauth-build .
	docker run --rm -v /var/run/docker.sock:/var/run/docker.sock -ti zalando/rusty-oauth-build

# run this as soon as you add, remove or modify one of the dependencies of the project
build-base-image:
	docker build -f docker/Dockerfile.base --rm -t zalando/rusty-oauth-build-base .

build-all: build-base-image default

docker-run:
	docker run -p 6767:6767 -e RUST_LOG=info -t danpersa/rusty-oauth:latest

docker-push:
	docker push danpersa/rusty-oauth:latest

# virtual box port forwarding
# VBoxManage controlvm "default" natpf1 "tcp-port6767,tcp,,6767,,6767"
curl:
	curl -v http://localhost:6767/oauth2/tokeninfo\?access_token\=token-employees-uid-route.write_read

bench:
	wrk -t12 -c900 -d10s http://localhost:6767/oauth2/tokeninfo\?access_token\=token-employees-uid-route.write_read
