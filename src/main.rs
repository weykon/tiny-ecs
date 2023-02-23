#[derive(PartialEq, PartialOrd)]
struct Health(i32);
struct Name(&'static str);

struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}
impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }
    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }
    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = Some(component);
                return;
            }
        }

        /* continued below */
        // No matching component storage exists yet, so we have to make one.
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count); // 在这个comp创建好目前实体数的长度

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push(None); // 这个comp目前的数据全部为None
        }

        // Give this Entity the Component.
        new_component_vec[entity] = Some(component); // 为当前实体赋值为传入的值
        self.component_vecs.push(Box::new(new_component_vec)); // 最后将这个未知大小的数组Box包起来
    }

    fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<&Vec<Option<ComponentType>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }
}

fn main() {
    let mut world = World::new();
    let entity0 = world.new_entity();
    world.add_component_to_entity(entity0, Health(100));

    let zip = world
        .borrow_component_vec::<Health>()
        .unwrap()
        .iter()
        .zip(world.borrow_component_vec::<Name>().unwrap().iter());

    // Same as before!
    for (health, name) in zip.filter_map(|(health, name)| Some((health.as_ref()?, name.as_ref()?)))
    {
        if health < 0 {
            println!("{} has perished!", name);
        }
    }
}
