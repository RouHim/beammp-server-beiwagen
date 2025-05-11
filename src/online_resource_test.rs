use crate::online_resource;

#[test]
fn test_read_invalid_mod_id() {
    // GIVEN an invalid mod_id
    let mod_id = "invalid";

    // WHEN the function is called
    let resource = online_resource::read(mod_id);

    // THEN it should return None
    assert!(resource.is_none());
}

#[test]
fn test_read_non_existent_mod() {
    // GIVEN a mod_id for a mod that does not exist
    let mod_id = "9999999999"; // Assuming this mod_id does not exist

    // WHEN the function is called
    let resource = online_resource::read(mod_id);

    // THEN it should return None
    assert!(resource.is_none());
}

#[test]
fn test_read_existing_mod() {
    // GIVEN a mod_id for a mod that exists
    let mod_id = "1362";

    // WHEN the function is called
    let resource = online_resource::read(mod_id);

    // THEN it should return a Resource
    assert!(resource.is_some());
    let resource = resource.unwrap();
    println!("Resource: {:?}", resource);
    assert_eq!(resource.id, 1362);
}
