pub fn em1() {
    enum IpAddKind {V4,V6,}
    let four = IpAddKind::V4;
    fn route (ip_type: IpAddKind) { }
    route(IpAddKind::V4); // 参数
    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }
    let home = IpAddr{ kind: IpAddKind::V4, address: String::from("127.0.0.1") };


}