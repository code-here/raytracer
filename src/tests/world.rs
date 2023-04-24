use crate::world::World;

#[test]
fn new_world_has_no_light_source_and_no_objects() {
    let world = World::new();
    assert!(world.light.is_none());
    assert_eq!(world.objects.len(), 0);
}
