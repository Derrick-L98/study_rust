use redis::cluster::ClusterClient;
use redis::Commands;

fn main() {
	let nodes = vec!["redis://:pl9rz1L0hbOoacesE3Jh@172.16.1.221:7000/","redis://:pl9rz1L0hbOoacesE3Jh@172.16.1.221:7002/"];

	let client = ClusterClient::open(nodes).unwrap();
	let mut conn = client.get_connection().unwrap();
	//let _: () = conn.hset("QUOTATIONS", "688330_XSHG","142.800000|4.530000|3.280000").unwrap();
	let rv:String = 
	// match redis::cmd("HGET").arg("QUOTATIONS").arg("688330_XSHG").query(&mut conn) {
	// 	Ok(val) => val,
	// 	Err(_) => "no".to_string(),
	// };
	match conn.hget("QUOTATIONS","688330_XSHG") {
		Ok(val) => val,
		Err(_) => "no".to_string(),
	};

	let s = match conn.hget("commodity_5210","HandsNum") {
		Ok(val) => val,
		Err(_) => "no".to_string(),
	};
    println!("Hello, world! {},{}",rv,   s);
}
