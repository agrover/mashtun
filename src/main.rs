use etherparse::PacketBuilder;
use tokio_tun::TunBuilder;

#[tokio::main]
async fn main() {
    let builder = PacketBuilder::ipv4([192, 0, 2, 2], [198, 145, 19, 9], 100)
        .tcp(5041, 22, 6666, 4000)
        .syn();
    let payload = [];

    //get some memory to store the result
    let mut packet = Vec::<u8>::with_capacity(builder.size(payload.len()));
    builder.write(&mut packet, &payload).unwrap();

    let tun = TunBuilder::new()
        .name("tun0")
        .packet_info(false)
        .owner(1000)
        .group(1000)
        .up()
        .persist()
        .try_build()
        .unwrap();

    //tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    let res = tun.send(&packet).await.unwrap();

    println!("Hello, world! {:?}", res);

    let mut recv_buf = vec![50];
    let res2 = tun.recv(&mut recv_buf).await.unwrap();

    println!("AG buf res{:?} buf {:?}", res2, recv_buf);
}
