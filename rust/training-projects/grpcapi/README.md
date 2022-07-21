<h1>
Rust gRPC usage
</h1>
<p>

</p>
<table>
  <tr>
    <td><b><a href="https://github.com/grpc/grpc">gRPC</a></b></td>
    <td>
        Git clone this package<br>
        sudo apt-get install libgflags-dev <br>
        mkdir -p cmake/build<br>
        cd cmake/build<br>
        cmake -DgRPC_BUILD_TESTS=ON ../..<br>
        make grpc_cli<br>
        git submodule update --init<br>
        ./grpc_cli call localhost:50051 deviceconfig.Deviceproto.GetDevice "id:'1'"<br>
    </td>
  </tr>

  <tr>
    <td><b>Rust gRPC</b></td>
    <td>
        enter to the root folder and use: Cargo run
    </td>
  </tr>
</table>