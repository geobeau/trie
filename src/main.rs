mod patricia;

fn main() {
    let data1 = vec![10u8, 11u8, 12u8];
    let data2 = vec![13u8, 14u8, 15u8];
    let data3 = vec![16u8, 17u8, 18u8];
    let data4 = vec![10u8, 11u8, 13u8];
    let data5 = vec![16u8, 11u8, 13u8];

    let mut patricia = patricia::Patricia::new();
    patricia.insert(data1.clone());
    assert!(patricia.exists(&data1));
    patricia.insert(data2.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    patricia.insert(data3.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    assert!(patricia.exists(&data3));
    patricia.insert(data4.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    assert!(patricia.exists(&data3));
    assert!(patricia.exists(&data4));
    patricia.insert(data5.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    assert!(patricia.exists(&data3));
    assert!(patricia.exists(&data4));
    assert!(patricia.exists(&data5));
}
