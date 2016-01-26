# Rusty OAuth

This is a mock OAuth server.

We provide an access token with a specific format as an url parameter. This token will be parsed and data from the token extracted.

For example:

    curl -v http://localhost:6767/oauth2/tokeninfo\?access_token\=token-realm1-scope1-scope2

Will return:

    {"scope":["scope1","scope2"],"realm":"realm1"}%

# Benchmarking

    wrk -t12 -c400 -d30s http://localhost:6767/oauth2/tokeninfo\?access_token\=token-emp-scope1-scope2

    Running 30s test @ http://localhost:6767/oauth2/tokeninfo?access_token=token-emp-scope1-scope2
      12 threads and 400 connections
      Thread Stats   Avg      Stdev     Max   +/- Stdev
        Latency   702.37us    1.38ms 126.03ms   97.19%
        Req/Sec     3.54k     2.24k    7.51k    56.83%
      211311 requests in 30.09s, 37.48MB read
      Socket errors: connect 157, read 316, write 0, timeout 0
    Requests/sec:   7021.87
    Transfer/sec:      1.25MB

# Docker Machine with Virtual Box

    VBoxManage controlvm "default" natpf1 "tcp-port6767,tcp,,6767,,6767"
