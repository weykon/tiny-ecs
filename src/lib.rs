struct Entity {
    id: u32,
    components: Vec<Box<dyn Component>>,
}

trait Component {
    fn update(&mut self);
}

struct PositionComponent {
    x: f32,
    y: f32,
}

impl Component for PositionComponent {
    fn update(&mut self) {
        self.x += 1.0;
        self.y += 1.0;
    }
}

struct CollisionComponent {
    radius: f32,
}

impl Component for CollisionComponent {
    fn update(&mut self) {
        self.radius += 1.0;
    }
}

struct CollisionSystem {
    entities: Vec<Entity>,
}
impl CollisionSystem {
    fn update(&mut self) {
        for entity in &mut self.entities {
            let mut position_component: Option<PositionComponent> = None;
            let mut collision_component: Option<CollisionComponent> = None;
            for component in &mut entity.components {}
            if let (Some(position), Some(collision)) = (position_component, collision_component) {
                // Do collision logic here
            }
        }
    }
}

struct AnimationSystem {
    entities: Vec<Entity>,
}

impl AnimationSystem {
    fn update(&mut self) {
        for entity in &mut self.entities {
            for component in &mut entity.components {}
        }
    }
}
