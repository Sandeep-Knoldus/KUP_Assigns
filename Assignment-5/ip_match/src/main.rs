///enum 'IpAddress' which have variants for classes of ipaddress
///
/// #variant
///
/// ClassA:- variant of enum IpAddress and it is String type
///
/// ClassB:- variant of enum IpAddress and it is String type
///
/// ClassC:- variant of enum IpAddress and it is String type
///
/// ClassD:- variant of enum IpAddress and it is String type
///
/// ClassE:- variant of enum IpAddress and it is String type
#[derive(PartialEq, Eq, Debug)]
pub enum IpAddress {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
}

/// function 'check_ip' which is used check the given ip_Address
///
/// #Arguments
///
/// ip: is tuple object of unsigned integer type
///
/// #Return
///
/// Returns Result enum which used give the Class Of Ip
pub fn check_ip(ip: (u128, u128, u128, u128)) {
    match ip {
        (1..=126, 0..=255, 0..=255, 1..=254) => {
            println!("IpAddress::ClassA({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (128..=191, 0..=255, 0..=255, 1..=254) => {
            println!("IpAddress::ClassB({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (192..=223, 0..=255, 1..=254, 1..=254) => {
            println!("IpAddress::ClassC({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (224..=239, 0..=255, 0..=255, 0..=255) => {
            println!("IpAddress::ClassD({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (240..=254, 0..=255, 0..=255, 0..=254) => {
            println!("IpAddress::ClassE({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        _ => println!("Unwanted Input"),
    }
}

fn main() {
    // initializing variable ip_1
    let ip_1 = (10, 17, 89, 1);

    // calling 'match_ip_address' passing ip_1 variable
    check_ip(ip_1);

    // initializing variable ip_2
    let ip_2 = (192, 168, 31, 1);

    // calling 'match_ip_address' passing ip_2 variable
    check_ip(ip_2);
}
